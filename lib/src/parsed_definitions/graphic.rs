//! Graphic object definition and parsing.

use tracing::warn;

use crate::{
    custom_graphic_extension::CustomGraphicExtension,
    metadata::{ObjectType, RawMetadata},
    raw_definitions::{CUSTOM_GRAPHIC_TOKENS, GROWTH_TOKENS, PLANT_GRAPHIC_TEMPLATE_TOKENS},
    sprite_graphic::SpriteGraphic,
    sprite_layer::SpriteLayer,
    tags::GraphicTypeTag,
    traits::{RawObject, Searchable, searchable::clean_search_vec},
    utilities::build_object_id_from_pieces,
};

/// A struct representing a Graphic object.
#[allow(clippy::module_name_repetitions)]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Graphic {
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RawMetadata>,
    identifier: String,
    object_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    caste_identifier: Option<String>,
    kind: GraphicTypeTag,

    #[serde(skip_serializing_if = "Option::is_none")]
    sprites: Option<Vec<SpriteGraphic>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    layers: Option<Vec<(String, Vec<SpriteLayer>)>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    growths: Option<Vec<(String, Vec<SpriteGraphic>)>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    custom_extensions: Option<Vec<CustomGraphicExtension>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,

    #[serde(skip)]
    layer_mode: bool,
}

impl Graphic {
    /// Function to create a new empty Graphic.
    ///
    /// # Returns
    ///
    /// * `Graphic` - The new empty Graphic.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            metadata: Some(
                RawMetadata::default()
                    .with_object_type(ObjectType::Graphics)
                    .with_hidden(true),
            ),
            ..Default::default()
        }
    }
    /// Function to create a new Graphic.
    ///
    /// # Parameters
    ///
    /// * `identifier` - The identifier for the Graphic.
    /// * `metadata` - The metadata for the Graphic.
    /// * `graphic_type` - The type of graphic.
    ///
    /// # Returns
    ///
    /// * `Graphic` - The new Graphic.
    #[must_use]
    pub fn new(identifier: &str, metadata: &RawMetadata, graphic_type: GraphicTypeTag) -> Self {
        Self {
            identifier: String::from(identifier),
            metadata: Some(metadata.clone()),
            object_id: build_object_id_from_pieces(metadata, identifier, &ObjectType::Graphics),
            kind: graphic_type,
            ..Self::default()
        }
    }
    fn add_layer_if_not_exists(&mut self, layer_name: String) {
        if let Some(layers) = self.layers.as_mut() {
            if !layers.iter().any(|(name, _)| name == &layer_name) {
                layers.push((layer_name, Vec::new()));
            }
        } else {
            self.layers = Some(vec![(layer_name, Vec::new())]);
        }
    }
    fn parse_layer_set_from_value(&mut self, value: &str) {
        self.add_layer_if_not_exists(String::from(value));
    }
    fn parse_layer_from_value(&mut self, value: &str) {
        if let Some(layer) = SpriteLayer::parse_layer_from_value(value) {
            if self.layers.is_none() {
                self.add_layer_if_not_exists(String::from("default"));
            }
            if let Some(layers) = self.layers.as_mut() {
                #[allow(clippy::unwrap_used)]
                layers.last_mut().unwrap().1.push(layer);
            }
        }
    }
    fn parse_layer_condition_token(&mut self, key: &str, value: &str) {
        if let Some(layers) = self.layers.as_mut() {
            // Conditions get attached to the last layer in the last layer group
            #[allow(clippy::unwrap_used)]
            if let Some(layer) = layers.last_mut().unwrap().1.last_mut() {
                layer.parse_condition_token(key, value);
            } else {
                warn!(
                    "Graphic::parse_condition_token: [{}] Failed to parse {}:{} as LayerCondition",
                    self.identifier, key, value
                );
            }
        } else {
            warn!(
                "Graphic::parse_condition_token: [{}] Failed to parse {}:{} as LayerCondition (No existing layers)",
                self.identifier, key, value
            );
        }
    }
    /// Parse a token from a tag into a `SpriteGraphic` and add it to the current sprite.
    ///
    /// # Parameters
    ///
    /// * `key` - The key of the token.
    /// * `value` - The value of the token.
    /// * `graphic_type` - The type of graphic.
    #[allow(clippy::too_many_lines)]
    pub fn parse_sprite_from_tag(&mut self, key: &str, value: &str, graphic_type: GraphicTypeTag) {
        // Check if key is LAYER_SET meaning a new layer group is starting
        if key == "LAYER_SET" {
            // Parse the value into a SpriteLayer
            self.parse_layer_set_from_value(value);
            self.layer_mode = true;
            return;
        }

        // Check if key is LAYER meaning a new layer should be added to the current layer group
        if key == "LAYER" {
            // Parse the value into a SpriteLayer
            self.parse_layer_from_value(value);
            self.layer_mode = true;
            return;
        }

        // Layers can be defined in groups.. for now we just ignore it
        if key == "LAYER_GROUP" {
            self.layer_mode = true;
            return;
        }
        if key == "END_LAYER_GROUP" {
            self.layer_mode = false;
            return;
        }

        // Right now we don't handle TREE_TILE
        if key == "TREE_TILE" {
            return;
        }

        // Check if the key indicates a new growth.
        if key == "GROWTH" {
            if let Some(growths) = self.growths.as_mut() {
                growths.push((String::from(value), Vec::new()));
            } else {
                self.growths = Some(vec![(String::from(value), Vec::new())]);
            }
            return;
        }

        // Check if the value is empty, which means we have a tag
        if value.is_empty() {
            if let Some(tags) = self.tags.as_mut() {
                tags.push(String::from(key));
            } else {
                self.tags = Some(vec![String::from(key)]);
            }
            return;
        }

        // If the key is a custom extension, parse it into a CustomGraphicExtension and add it to the current sprite
        if let Some(extension_type) = CUSTOM_GRAPHIC_TOKENS.get(key) {
            if let Some(custom_extension) =
                CustomGraphicExtension::from_value(*extension_type, value)
            {
                if let Some(custom_extensions) = self.custom_extensions.as_mut() {
                    custom_extensions.push(custom_extension);
                } else {
                    self.custom_extensions = Some(vec![custom_extension]);
                }
            } else {
                warn!(
                    "Graphic::parse_sprite_from_tag:_extension_type [{}] Failed to parse {},{} as CustomGraphicExtension",
                    self.identifier, key, value
                );
            }
            return;
        }

        // If the key is a growth token, parse it into a SpriteGraphic and add it to the current growth
        if let Some(_growth_type) = GROWTH_TOKENS.get(key) {
            if let Some(sprite_graphic) = SpriteGraphic::from_token(key, value, graphic_type) {
                if let Some(growths) = self.growths.as_mut()
                    && let Some(growth) = growths.last_mut()
                {
                    growth.1.push(sprite_graphic);
                };
            } else {
                warn!(
                    "Graphic::parse_sprite_from_tag:_growth_type [{}] Failed to parse {},{} as SpriteGraphic",
                    self.identifier, key, value
                );
            }
            return;
        }
        // Check if the key is plant graphic template, which for now we accept only on growths
        if let Some(_plant_graphic_template) = PLANT_GRAPHIC_TEMPLATE_TOKENS.get(key) {
            if let Some(sprite_graphic) =
                SpriteGraphic::from_token(key, value, GraphicTypeTag::Template)
            {
                if let Some(growths) = self.growths.as_mut()
                    && let Some(growth) = growths.last_mut()
                {
                    growth.1.push(sprite_graphic);
                };
            } else {
                warn!(
                    "Graphic::parse_sprite_from_tag:_plant_graphic_template [{}] Failed to parse {},{} as SpriteGraphic",
                    self.identifier, key, value
                );
            }
            return;
        }

        // Check if we are in layer mode, and if so, parse the token as a layer condition
        if self.layer_mode {
            self.parse_layer_condition_token(key, value);
            return;
        }

        // Otherwise we can parse it for a sprite and report an error if that fails.
        if let Some(sprite_graphic) = SpriteGraphic::from_token(key, value, graphic_type) {
            if let Some(sprites) = self.sprites.as_mut() {
                sprites.push(sprite_graphic);
            } else {
                self.sprites = Some(vec![sprite_graphic]);
            }
        } else {
            warn!(
                "Graphic::parse_sprite_from_tag:_from_token [{}] Failed to parse [{}:{}] as SpriteGraphic::{:?}",
                self.identifier, key, value, graphic_type
            );
        }
    }
    /// Get the type of the Graphic.
    ///
    /// # Returns
    ///
    /// * `GraphicType` - The type of the Graphic.
    #[must_use]
    pub const fn get_graphic_type(&self) -> GraphicTypeTag {
        self.kind
    }
    /// Get the tile page IDs for the Graphic.
    ///
    /// # Returns
    ///
    /// * `Vec<String>` - The tile page IDs for the Graphic.
    #[must_use]
    pub fn get_tile_pages(&self) -> Vec<String> {
        let mut vec = Vec::new();
        if let Some(sprites) = &self.sprites {
            for sprite in sprites {
                vec.push(sprite.get_tile_page_id().to_string());
            }
        }
        if let Some(layers) = &self.layers {
            for layer in layers {
                for sprite in &layer.1 {
                    vec.push(sprite.get_tile_page_id().to_string());
                }
            }
        }
        vec
    }
    /// Function to "clean" the creature. This is used to remove any empty list or strings,
    /// and to remove any default values. By "removing" it means setting the value to None.
    ///
    /// This also will remove the metadata if `is_metadata_hidden` is true.
    ///
    /// Steps for all "Option" fields:
    /// - Set any metadata to None if `is_metadata_hidden` is true.
    /// - Set any empty string to None.
    /// - Set any empty list to None.
    /// - Set any default values to None.
    ///
    /// # Returns
    ///
    /// * `Graphic` - The cleaned Graphic.
    #[must_use]
    pub fn cleaned(&self) -> Self {
        let mut cleaned = self.clone();

        if let Some(metadata) = &cleaned.metadata
            && metadata.is_hidden()
        {
            cleaned.metadata = None;
        }

        if let Some(custom_extensions) = &cleaned.custom_extensions
            && custom_extensions.is_empty()
        {
            cleaned.custom_extensions = None;
        }

        if let Some(tags) = &cleaned.tags
            && tags.is_empty()
        {
            cleaned.tags = None;
        }

        if let Some(sprites) = &cleaned.sprites {
            let mut new_sprites = Vec::new();
            for sprite in sprites {
                new_sprites.push(sprite.cleaned());
            }
            cleaned.sprites = Some(new_sprites);
        }

        if let Some(layers) = &cleaned.layers {
            let mut new_layers = Vec::new();
            for (name, sprites) in layers {
                let mut new_sprites = Vec::new();
                for sprite in sprites {
                    new_sprites.push(sprite.cleaned());
                }
                new_layers.push((name.clone(), new_sprites));
            }
            cleaned.layers = Some(new_layers);
        }

        if let Some(growths) = &cleaned.growths {
            let mut new_growths = Vec::new();
            for (name, sprites) in growths {
                let mut new_sprites = Vec::new();
                for sprite in sprites {
                    new_sprites.push(sprite.cleaned());
                }
                new_growths.push((name.clone(), new_sprites));
            }
            cleaned.growths = Some(new_growths);
        }

        cleaned
    }
}

#[typetag::serde]
impl RawObject for Graphic {
    fn get_searchable_tokens(&self) -> Vec<&str> {
        Vec::new()
    }
    fn get_metadata(&self) -> RawMetadata {
        self.metadata.as_ref().map_or_else(
            || {
                warn!("Metadata is missing for {}", self.get_identifier());
                RawMetadata::default()
                    .with_object_type(ObjectType::Graphics)
                    .with_hidden(true)
            },
            std::clone::Clone::clone,
        )
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
    fn get_name(&self) -> &str {
        &self.identifier
    }
    fn is_empty(&self) -> bool {
        self.identifier.is_empty()
    }
    fn get_type(&self) -> &ObjectType {
        &ObjectType::Graphics
    }
    fn clean_self(&mut self) {
        *self = self.cleaned();
    }

    fn parse_tag(&mut self, key: &str, value: &str) {
        // Any tags should just be able to be handled by the sprite graphic, but it needs to call the right function
        warn!(
            "Graphics tag attempted parse with wrong method: {}:{} for {}",
            key,
            value,
            self.get_identifier()
        );
    }

    fn get_object_id(&self) -> &str {
        &self.object_id
    }
}

impl Searchable for Graphic {
    fn get_search_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();

        vec.push(self.get_identifier().to_string());
        vec.push(format!("{:?}", self.kind));
        vec.push("graphic".to_string());

        clean_search_vec(vec.as_slice())
    }
}
