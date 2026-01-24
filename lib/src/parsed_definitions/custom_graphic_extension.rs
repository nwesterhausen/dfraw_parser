//! Custom graphic extension definition.

use dfraw_parser_proc_macros::IsEmpty;
use tracing::warn;

use crate::tokens::GraphicTypeToken;

/// A custom graphic extension.
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
pub struct CustomGraphicExtension {
    extension_type: GraphicTypeToken,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    tile_page_id: Option<String>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    value_1: Option<u32>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    value_2: Option<u32>,
}

impl CustomGraphicExtension {
    /// Create a new custom graphic extension.
    ///
    /// # Arguments
    ///
    /// * `extension_type` - The type of the extension.
    /// * `value` - The value of the extension.
    ///
    /// # Returns
    ///
    /// A new custom graphic extension.
    #[must_use]
    pub fn from_value(extension_type: GraphicTypeToken, value: &str) -> Option<Self> {
        // 2 Options:
        // [CUSTOM_EDGING:          4]
        // [CUSTOM_RAMP:            6]
        // ( key )              value_1
        // [CUSTOM_EDGE_W:      GRASS_OTHER:    4:        2]
        // ( key )              tile_page_id  value_1  value_2
        let mut split = value.split(':');
        let possible_value_1 = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };

        // If we can parse u32 from the first value, we have the first option
        if let Ok(value_1) = possible_value_1.parse::<u32>() {
            return Some(Self {
                extension_type,
                value_1: Some(value_1),
                ..Self::default()
            });
        }

        // Otherwise, we have the second option
        let tile_page_id = possible_value_1;
        let possible_value_1 = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };
        let possible_value_2 = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };

        if let (Ok(value_1), Ok(value_2)) = (
            possible_value_1.parse::<u32>(),
            possible_value_2.parse::<u32>(),
        ) {
            Some(Self {
                extension_type,
                tile_page_id: Some(tile_page_id),
                value_1: Some(value_1),
                value_2: Some(value_2),
            })
        } else {
            warn!(
                "CustomGraphicExtension::from_value: Failed to parse {} OR {} as u32",
                possible_value_1, possible_value_2
            );
            None
        }
    }
}
