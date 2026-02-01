//! Contains the `SpriteLayer` struct and associated functions.

use dfraw_parser_proc_macros::IsEmpty;
use tracing::warn;

use crate::{custom_types::Dimensions, raw_definitions::CONDITION_TOKENS, tokens::ConditionToken};

/// A struct representing a `SpriteLayer` object.
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
)]
#[serde(rename_all = "camelCase")]
pub struct SpriteLayer {
    /// Name of the layer (its identifier)
    layer_name: String,
    /// Identifier of the tile page used by this layer
    tile_page_id: String,
    /// The position in the tile page that is the top-left corner of the defined sprite
    offset: Dimensions,
    /// Optionally defines the bottom-right position in the tile page (for non-square sprites)
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    offset_2: Option<Dimensions>,
    /// Whether the sprite is a large image (i.e., includes a 2nd offset)
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    large_image: Option<bool>,
    /// An array of required conditions for this sprite to be visible/used
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    conditions: Option<Vec<(ConditionToken, String)>>,
}

impl SpriteLayer {
    /// Get the top-left position of the defined sprite
    #[must_use]
    pub fn get_offset(&self) -> Dimensions {
        self.offset
    }
    /// Get the bottom-right position of the defined sprite, or None if it isn't defined
    #[must_use]
    pub fn get_offset2(&self) -> Option<Dimensions> {
        self.offset_2
    }
    /// Get the name/identifier of the sprite
    #[must_use]
    pub fn get_name(&self) -> String {
        self.layer_name.clone()
    }
    /// Returns the `tile_page_id` of the `SpriteLayer`.
    ///
    /// # Returns
    ///
    /// * `&str` - The `tile_page_id` of the `SpriteLayer`.
    #[must_use]
    pub fn get_tile_page_id(&self) -> &str {
        self.tile_page_id.as_str()
    }
    /// Parse a condition token into a `LayerCondition`.
    ///
    /// # Parameters
    ///
    /// * `key` - The key of the condition token.
    /// * `value` - The value of the condition token.
    pub fn parse_condition_token(&mut self, key: &str, value: &str) {
        // Condition is the key, and it should match a value in LAYER_CONDITION_TAGS
        if let Some(condition) = CONDITION_TOKENS.get(key) {
            if self.conditions.is_none() {
                self.conditions = Some(Vec::new());
            }
            if let Some(conditions) = &mut self.conditions {
                // It's true that some conditions have a value, some have a tag, and some are standalone.
                // At the moment we only care about saving the tag, so we'll just save the value as a string.
                conditions.push((*condition, String::from(value)));
            }
        } else {
            warn!(
                "Failed to parse {} as LayerCondition, unknown key {}",
                value, key
            );
        }
    }
    /// Parse a layer value into a `SpriteLayer`.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to parse.
    ///
    /// # Returns
    ///
    /// * `Option<SpriteLayer>` - The parsed `SpriteLayer`.
    #[must_use]
    pub fn parse_layer_from_value(value: &str) -> Option<Self> {
        // ...BODY:CREATURES_DOMESTIC:0:21]
        let mut split = value.split(':');

        let layer_name = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };
        let tile_page_id = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };

        let fourth_position_token = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };

        let large_image = matches!(fourth_position_token.as_str(), "LARGE_IMAGE");

        if large_image {
            return Self::parse_large_layer_with_split(
                layer_name.as_str(),
                tile_page_id.as_str(),
                split.collect::<Vec<&str>>().as_slice(),
            );
        }

        let tile_offset_y = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };

        let offset_x: u32 = match fourth_position_token.parse() {
            Ok(n) => n,
            Err(_e) => {
                warn!(
                    "parse_layer_from_value: Failed to parse {} as offset_x, {}",
                    fourth_position_token, value
                );
                return None;
            }
        };

        let offset_y: u32 = match tile_offset_y.parse() {
            Ok(n) => n,
            Err(_e) => {
                warn!(
                    "parse_layer_from_value: Failed to parse {} as offset_y, {}",
                    tile_offset_y, value
                );
                return None;
            }
        };

        Some(Self {
            layer_name,
            tile_page_id,
            offset: Dimensions::from_xy(offset_x, offset_y),
            ..Self::default()
        })
    }
    /// Parse a large layer value into a `SpriteLayer`.
    ///
    /// # Parameters
    ///
    /// * `layer_name` - The name of the layer.
    /// * `tile_page_id` - The `tile_page_id` of the layer.
    /// * `split` - The split of the value.
    ///
    /// # Returns
    ///
    /// * `Option<SpriteLayer>` - The parsed `SpriteLayer`.
    #[must_use]
    fn parse_large_layer_with_split(
        layer_name: &str,
        tile_page_id: &str,
        split: &[&str],
    ) -> Option<Self> {
        let x1: u32 = match split.first() {
            Some(v) => match v.parse() {
                Ok(n) => n,
                Err(_e) => {
                    warn!(
                        "parse_large_creature_with_split: Failed to parse {} as offset_x1 {:?}",
                        v, split
                    );
                    return None;
                }
            },
            _ => {
                return None;
            }
        };

        let y1: u32 = match split.get(1) {
            Some(v) => match v.parse() {
                Ok(n) => n,
                Err(_e) => {
                    warn!(
                        "parse_large_creature_with_split: Failed to parse {} as offset_y1 {:?}",
                        v, split
                    );
                    return None;
                }
            },
            _ => {
                return None;
            }
        };

        let x2: u32 = match split.get(2) {
            Some(v) => match v.parse() {
                Ok(n) => n,
                Err(_e) => {
                    warn!(
                        "parse_large_creature_with_split: Failed to parse {} as offset_x2 {:?}",
                        v, split
                    );
                    return None;
                }
            },
            _ => {
                return None;
            }
        };

        let y2: u32 = match split.get(3) {
            Some(v) => match v.parse() {
                Ok(n) => n,
                Err(_e) => {
                    warn!(
                        "parse_large_creature_with_split: Failed to parse {} as offset_y2 {:?}",
                        v, split
                    );
                    return None;
                }
            },
            _ => {
                return None;
            }
        };

        Some(Self {
            layer_name: String::from(layer_name),
            tile_page_id: String::from(tile_page_id),
            large_image: Some(true),
            offset: Dimensions::from_xy(x1, y1),
            offset_2: Some(Dimensions::from_xy(x2, y2)),
            ..Self::default()
        })
    }
}
