//! Graphic object definition and parsing.

use dfraw_parser_proc_macros::{Cleanable, IsEmpty};
use tracing::warn;
use uuid::Uuid;

use crate::{
    CustomGraphicExtension, GraphicPalette, SpriteGraphic, SpriteLayer,
    metadata::RawMetadata,
    tokens::{
        ConditionToken, GraphicTypeToken, ObjectType,
        raw_definitions::{CUSTOM_GRAPHIC_TOKENS, GROWTH_TOKENS, PLANT_GRAPHIC_TEMPLATE_TOKENS},
    },
    traits::TagOperations,
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
    #[serde(default)]
    pub caste_identifier: Option<String>,
    /// The type of graphic
    #[cleanable(ignore)]
    pub kind: GraphicTypeToken,
    /// A vector of sprites defined in the raw
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    pub sprites: Vec<SpriteGraphic>,
    /// A vector of layers defined in the raw
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    pub layers: Vec<(String, Vec<SpriteLayer>)>,
    /// A vector of growths defined in the raw
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    pub growths: Vec<(String, Vec<SpriteGraphic>)>,
    /// A vector of custom extensions defined in the raw
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    pub custom_extensions: Vec<CustomGraphicExtension>,
    /// A vector of the defined tags in the raw
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    pub tokens: Vec<ConditionToken>,
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
        self.sprites.clone()
    }

    /// Get the sprites defined in this graphic
    #[must_use]
    pub fn get_layers(&self) -> Vec<(String, Vec<SpriteLayer>)> {
        self.layers.clone()
    }

    /// Get the growths defined in this graphic
    #[must_use]
    pub fn get_growths(&self) -> Vec<(String, Vec<SpriteGraphic>)> {
        self.growths.clone()
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
        vec.extend(
            self.sprites
                .iter()
                .map(|s| s.get_tile_page_id().to_string()),
        );
        vec.extend(
            self.layers
                .iter()
                .flat_map(|layer| layer.1.iter().map(|s| s.get_tile_page_id().to_string())),
        );
        vec
    }

    /// Merge another Graphic object into this one.
    ///
    /// This transfers all sprites/layers/etc from `other` to `self`
    pub fn merge(&mut self, other: Graphic) {
        // Merge Sprites
        if !other.sprites.is_empty() {
            self.sprites.extend(other.sprites);
        }

        // Merge Layers
        if !other.layers.is_empty() {
            self.layers.extend(other.layers);
        }

        // Merge Growths
        if !other.growths.is_empty() {
            self.growths.extend(other.growths);
        }

        // Merge Custom Extensions
        if !other.custom_extensions.is_empty() {
            self.custom_extensions.extend(other.custom_extensions);
        }

        // Merge Tags
        if !other.tokens.is_empty() {
            self.tokens.extend(other.tokens);
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
            self.growths.push((String::from(value), Vec::new()));
            return;
        }

        // Check if the value is empty, which means we have a tag
        if value.is_empty() {
            if let Some(token) = ConditionToken::parse(key, value) {
                self.tokens.push(token);
            } else {
                warn!("Unknown graphic token '{key}':'{value}'")
            }
            return;
        }

        // If the key is a custom extension, parse it into a CustomGraphicExtension and add it to the current sprite
        if CUSTOM_GRAPHIC_TOKENS.get(key).is_some()
            && let Some(extension_graphic_type) = GraphicTypeToken::parse(key, value)
        {
            if let Some(custom_extension) =
                CustomGraphicExtension::from_value(extension_graphic_type, value)
            {
                self.custom_extensions.push(custom_extension);
            } else {
                warn!(
                    "Graphic::parse_sprite_from_tag:_extension_type [{}] Failed to parse {},{} as CustomGraphicExtension",
                    self.identifier, key, value
                );
            }
            return;
        }

        // If the key is a growth token, parse it into a SpriteGraphic and add it to the current growth
        if GROWTH_TOKENS.get(key).is_some()
            && let Some(sprite_graphic) = SpriteGraphic::from_token(key, value, graphic_type)
        {
            if let Some(last_growth) = self.growths.last_mut() {
                last_growth.1.push(sprite_graphic);
            } else {
                warn!(
                    "Graphic::parse_sprite_from_tag: {} out of order (not after a GROWTH)",
                    self.identifier
                )
            }
            return;
        }

        // Check if the key is plant graphic template, which for now we accept only on growths
        if PLANT_GRAPHIC_TEMPLATE_TOKENS.get(key).is_some() {
            if let Some(sprite_graphic) =
                SpriteGraphic::from_token(key, value, GraphicTypeToken::Template)
            {
                if let Some(last_growth) = self.growths.last_mut() {
                    last_growth.1.push(sprite_graphic);
                } else {
                    warn!(
                        "Graphic::parse_sprite_from_tag: {} out of order (not after a GROWTH)",
                        self.identifier
                    )
                }
            } else {
                warn!(
                    "Graphic::parse_sprite_from_tag: [{}] Failed to parse {},{} as SpriteGraphic",
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
            self.sprites.push(sprite_graphic);
            return;
        }

        warn!(
            "Graphic::parse_sprite_from_tag: [{}] Failed to parse [{}:{}] (fell through)",
            self.identifier, key, value
        );
    }

    fn add_layer_if_not_exists(&mut self, layer_name: String) {
        if !self.layers.iter().any(|(name, _)| name == &layer_name) {
            self.layers.push((layer_name, Vec::new()));
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

            if self.layers.is_empty() {
                self.add_layer_if_not_exists(String::from("default"));
            }

            if let Some(last_layer) = self.layers.last_mut() {
                last_layer.1.push(layer);
            }
        }
    }

    #[tracing::instrument(skip(self), fields(self.identifier = &self.identifier))]
    fn parse_layer_palette_info(&mut self, key: &str, value: &str) {
        if let Some(condition_token) = ConditionToken::parse(key, value) {
            match condition_token {
                ConditionToken::LayerSetPalette { name } => {
                    self.palletes.push(GraphicPalette::new(&name))
                }
                ConditionToken::LayerSetPaletteDefault { default_row } => {
                    if let Some(pallete) = self.palletes.last_mut() {
                        pallete.set_default_row(default_row);
                    } else {
                        warn!("palette default_row attempted set out of order");
                    }
                }
                ConditionToken::LayerSetPaletteFile { path } => {
                    if let Some(pallete) = self.palletes.last_mut() {
                        pallete.set_file(&path);
                    } else {
                        warn!("palette file_path attempted set out of order");
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
        if let Some(last_layer) = self.layers.last_mut() {
            if last_layer.1.is_empty() {
                warn!("Failed to parse, No SpriteLayer defined yet: {last_layer:?}")
            } else if let Some(sprite_layer) = last_layer.1.last_mut() {
                sprite_layer.parse_condition_token(key, value);
            } else {
                warn!("Failed to parse, no mutable SpriteLayer: {last_layer:?}",);
            }
        } else {
            warn!("Failed to parse, no layer to append to: {:?}", self.layers);
        }
    }
}
