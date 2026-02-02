//! Graphic object definition and parsing.

use dfraw_parser_proc_macros::{Cleanable, IsEmpty};
use tracing::warn;
use uuid::Uuid;

use crate::{
    CustomGraphicExtension, GraphicPalette, SpriteGraphic, SpriteLayer,
    metadata::RawMetadata,
    raw_definitions::{
        CONDITION_TOKENS, CUSTOM_GRAPHIC_TOKENS, GROWTH_TOKENS, PLANT_GRAPHIC_TEMPLATE_TOKENS,
    },
    tokens::{ConditionToken, GraphicTypeToken, ObjectType},
    utilities::generate_object_id_using_raw_metadata,
};

/// A struct representing a Graphic object.
///
/// Stores data about layers and sprites defined in the graphic raw.
#[allow(clippy::module_name_repetitions)]
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    Default,
    specta::Type,
    PartialEq,
    Eq,
    IsEmpty,
    Cleanable,
)]
#[serde(rename_all = "camelCase")]
pub struct Graphic {
    /// The `metadata` field is of type `RawMetadata` and is used to provide additional information
    /// about the raws the `Graphic` is found in.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub metadata: RawMetadata,
    /// The `identifier` field is a string that represents the identifier of the graphic. It is used
    /// to uniquely identify the graphic
    pub identifier: String,
    /// A generated id that is used to uniquely identify this object.
    ///
    /// This is deterministic based on the following:
    /// * The raw's `identifier`
    /// * The raw's [`ObjectType`]
    /// * [`RawModuleLocation`] where the raw was found
    /// * The containing module's `numeric_version`
    ///
    /// See [`crate::utilities::generate_object_id`]
    pub object_id: Uuid,
    /// An optional identifier targeting a specific caste
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub caste_identifier: Option<String>,
    /// The type of graphic
    #[cleanable(ignore)]
    pub kind: GraphicTypeToken,
    /// A vector of sprites defined in the raw
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub sprites: Option<Vec<SpriteGraphic>>,
    /// A vector of layers defined in the raw
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub layers: Option<Vec<(String, Vec<SpriteLayer>)>>,
    /// A vector of growths defined in the raw
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub growths: Option<Vec<(String, Vec<SpriteGraphic>)>>,
    /// A vector of custom extensions defined in the raw
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub custom_extensions: Option<Vec<CustomGraphicExtension>>,
    /// A vector of the defined tags in the raw
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub tags: Option<Vec<String>>,
    /// Internal switch used during parsing to indicate whether currently within a layer
    #[serde(skip)]
    pub layer_mode: bool,
    /// Internal cache for tracking the group conditions defined for a layer
    #[serde(skip)]
    #[cleanable(ignore)]
    pub group_conditions: Vec<(String, String)>,
    /// The palletes used or defined in the raw
    pub palletes: Vec<GraphicPalette>,
}

impl Graphic {
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
    pub fn new(identifier: &str, metadata: &RawMetadata, graphic_type: GraphicTypeToken) -> Self {
        Self {
            identifier: String::from(identifier),
            metadata: metadata.clone(),
            object_id: generate_object_id_using_raw_metadata(
                identifier,
                ObjectType::Graphics,
                metadata,
            ),
            kind: graphic_type,
            ..Self::default()
        }
    }

    /// Function to create a new empty Graphic.
    ///
    /// # Returns
    ///
    /// * `Graphic` - The new empty Graphic.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            metadata: RawMetadata::default()
                .with_object_type(ObjectType::Graphics)
                .with_hidden(true),
            ..Default::default()
        }
    }

    /// Get the sprites defined in this graphic
    #[must_use]
    pub fn get_sprites(&self) -> Vec<SpriteGraphic> {
        match self.sprites.as_ref() {
            Some(sprites) => sprites.clone(),
            None => Vec::new(),
        }
    }

    /// Get the sprites defined in this graphic
    #[must_use]
    pub fn get_layers(&self) -> Vec<(String, Vec<SpriteLayer>)> {
        match self.layers.as_ref() {
            Some(layers) => layers.clone(),
            None => Vec::new(),
        }
    }

    /// Get the growths defined in this graphic
    #[must_use]
    pub fn get_growths(&self) -> Vec<(String, Vec<SpriteGraphic>)> {
        match self.growths.as_ref() {
            Some(growths) => growths.clone(),
            None => Vec::new(),
        }
    }

    /// Get the type of the Graphic.
    ///
    /// # Returns
    ///
    /// * `GraphicType` - The type of the Graphic.
    #[must_use]
    pub const fn get_graphic_type(&self) -> GraphicTypeToken {
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

    /// Merge another Graphic object into this one.
    ///
    /// This transfers all sprites/layers/etc from `other` to `self`
    pub fn merge(&mut self, other: Graphic) {
        // Merge Sprites
        if let Some(other_sprites) = other.sprites {
            if let Some(my_sprites) = self.sprites.as_mut() {
                my_sprites.extend(other_sprites);
            } else {
                self.sprites = Some(other_sprites);
            }
        }

        // Merge Layers
        if let Some(other_layers) = other.layers {
            if let Some(my_layers) = self.layers.as_mut() {
                my_layers.extend(other_layers);
            } else {
                self.layers = Some(other_layers);
            }
        }

        // Merge Growths
        if let Some(other_growths) = other.growths {
            if let Some(my_growths) = self.growths.as_mut() {
                my_growths.extend(other_growths);
            } else {
                self.growths = Some(other_growths);
            }
        }

        // Merge Custom Extensions
        if let Some(other_extensions) = other.custom_extensions {
            if let Some(my_extensions) = self.custom_extensions.as_mut() {
                my_extensions.extend(other_extensions);
            } else {
                self.custom_extensions = Some(other_extensions);
            }
        }

        // Merge Tags
        if let Some(other_tags) = other.tags {
            if let Some(my_tags) = self.tags.as_mut() {
                my_tags.extend(other_tags);
            } else {
                self.tags = Some(other_tags);
            }
        }

        // Merge Palettes
        self.palletes.extend(other.palletes);
    }

    /// Parse a token from a tag into a `SpriteGraphic` and add it to the current sprite.
    ///
    /// # Parameters
    ///
    /// * `key` - The key of the token.
    /// * `value` - The value of the token.
    /// * `graphic_type` - The type of graphic.
    #[allow(clippy::too_many_lines)]
    pub fn parse_sprite_from_tag(
        &mut self,
        key: &str,
        value: &str,
        graphic_type: GraphicTypeToken,
    ) {
        // Check if key is related to setting palette information
        if key == "LS_PALETTE" || key == "LS_PALETTE_FILE" || key == "LS_PALETTE_DEFAULT" {
            self.parse_layer_palette_info(key, value);
            return;
        }

        // Check if key is LAYER_SET meaning a new layer group is starting
        // We clear the group conditions since this is a new layer group
        if key == "LAYER_SET" {
            self.group_conditions.clear();
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

        // End of a group, so clear any groups and end layer mode
        if key == "END_LAYER_GROUP" {
            self.group_conditions.clear();
            self.layer_mode = false;
            return;
        }
        // Handle Layer Group Conditions (like LG_CONDITION_BP), register conditions into groups for later reference
        if key == "LG_CONDITION_BP" {
            self.group_conditions
                .push((String::from(key), String::from(value)));
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
                SpriteGraphic::from_token(key, value, GraphicTypeToken::Template)
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
        if let Some(mut layer) = SpriteLayer::parse_layer_from_value(value) {
            // Apply any active group conditions to this new layer
            for (key, val) in &self.group_conditions {
                layer.parse_condition_token(key, val);
            }

            if self.layers.is_none() {
                self.add_layer_if_not_exists(String::from("default"));
            }
            if let Some(layers) = self.layers.as_mut() {
                #[allow(clippy::unwrap_used)]
                layers.last_mut().unwrap().1.push(layer);
            }
        }
    }

    #[tracing::instrument(skip(self), fields(self.identifier = &self.identifier))]
    fn parse_layer_palette_info(&mut self, key: &str, value: &str) {
        if let Some(condition_tag) = CONDITION_TOKENS.get(key) {
            let last_pallete = self.palletes.last_mut();
            match condition_tag {
                ConditionToken::LayerSetPalette => self.palletes.push(GraphicPalette::new(value)),
                ConditionToken::LayerSetPaletteDefault => {
                    if let Some(palette) = last_pallete {
                        palette.set_default_row(value.parse().unwrap_or_default());
                    }
                }
                ConditionToken::LayerSetPaletteFile => {
                    if let Some(palette) = last_pallete {
                        palette.set_file(value);
                    }
                }
                _ => {}
            }
        } else {
            warn!("Expected LS_PALETTE token was invalid")
        }
    }

    #[tracing::instrument(skip(self), fields(self.identifier = &self.identifier))]
    fn parse_layer_condition_token(&mut self, key: &str, value: &str) {
        if let Some(layers) = self.layers.as_mut() {
            // Conditions get attached to the last layer in the last layer group
            #[allow(clippy::unwrap_used)]
            if let Some(layer_entry) = layers.last_mut() {
                if layer_entry.1.is_empty() {
                    warn!("Failed to parse, No SpriteLayer defined yet: {layer_entry:?}")
                } else if let Some(layer) = layer_entry.1.last_mut() {
                    layer.parse_condition_token(key, value);
                } else {
                    warn!("Failed to parse, no mutable SpriteLayer: {layer_entry:?}",);
                }
            } else {
                warn!("Failed to parse, no layer to append to: {layers:?}");
            }
        } else {
            warn!("Failed to parse, (No existing layers)");
        }
    }
}
