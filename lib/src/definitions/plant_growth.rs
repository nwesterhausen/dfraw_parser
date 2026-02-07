//! Contains the struct for plant growths and its implementation.

use dfraw_parser_proc_macros::{Cleanable, IsEmpty};
use tracing::{error, warn};

use crate::{
    custom_types::Name,
    tokens::{
        PlantGrowthToken, PlantGrowthTypeToken, PlantPartToken,
        raw_definitions::{PLANT_GROWTH_TOKENS, PLANT_PART_TOKENS},
    },
};

/// A struct representing a plant growth
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
pub struct PlantGrowth {
    /// Plant growths are not given an identifier, since they are just supporting
    /// data for the plant definition. They are defined instead by the type of growth.
    growth_type: PlantGrowthTypeToken,
    /// The name of the growth. This is actually defined with `GROWTH_NAME` key in the raws.
    pub name: Name,
    /// The item grown by this growth. This is actually defined with `GROWTH_ITEM` key in the raws.
    /// This is a string until we make a proper item structure. Technically there are 2 arguments:
    /// 1. item token, 2: material token. Generally the item type should be `PLANT_GROWTH:NONE`.
    item: String,
    /// Specifies on which part of the plant this growth grows. This is defined with `GROWTH_HOST_TILE` key.
    /// This can be unused, like in the case of crops where the plant is the growth (I think?).
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    host_tiles: Option<Vec<PlantPartToken>>,
    /// Controls the height on the trunk above which the growth begins to appear.
    /// The first value is the percent of the trunk height where the growth begins appearing:
    /// 0 will cause it along the entire trunk (above the first tile), 100 will cause it to appear
    /// at the topmost trunk tile. Can be larger than 100 to cause it to appear above the trunk.
    /// The second value must be -1, but might be intended to control whether it starts height counting
    /// from the bottom or top.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    trunk_height_percentage: Option<[i32; 2]>,
    /// Currently has no effect.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    density: Option<u32>,
    /// Specifies the appearance of the growth. This is defined with `GROWTH_PRINT` key.
    /// This is a string until we make a proper print structure.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    print: Option<String>,
    /// Specifies at which part of the year the growth appears. Default is all year round.
    /// Minimum: 0, Maximum: `402_200`. This is defined with `GROWTH_TIMING` key.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    #[is_empty(value = [0, 402_200])]
    timing: Option<[u32; 2]>,
    /// Where we gather some of the growth's tags.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    tags: Option<Vec<PlantGrowthToken>>,
}

impl PlantGrowth {
    /// Create a new plant growth based on a growth type
    ///
    /// # Arguments
    ///
    /// * `growth_type` - The type of growth
    ///
    /// # Returns
    ///
    /// A new plant growth
    #[must_use]
    pub fn new(growth_type: PlantGrowthTypeToken) -> Self {
        Self {
            growth_type,
            ..Self::default()
        }
    }
    /// Returns the type of growth this is
    pub fn get_growth_type(&self) -> &PlantGrowthTypeToken {
        &self.growth_type
    }
    /// Returns true if tag exists on this plant growth
    pub fn has_tag(&self, tag: &PlantGrowthToken) -> bool {
        if let Some(tags) = &self.tags {
            for t in tags {
                if std::mem::discriminant(t) == std::mem::discriminant(tag) {
                    return true;
                }
            }
        }
        false
    }
    /// Parses a tag and value into the plant growth
    ///
    /// # Arguments
    ///
    /// * `key` - The tag of the growth
    /// * `value` - The value of the growth
    #[allow(clippy::too_many_lines)]
    pub fn parse_tag(&mut self, key: &str, value: &str) {
        let Some(tag) = PLANT_GROWTH_TOKENS.get(key) else {
            warn!(
                "PlantGrowthParsing: called `Option::unwrap()` on a `None` value for presumed caste tag: {}",
                key
            );
            return;
        };

        if value.is_empty() {
            if self.tags.is_none() {
                self.tags = Some(Vec::new());
            }
            // If there is no value, we just add the tag to the list.
            if let Some(tags) = &mut self.tags {
                tags.push(*tag);
            }
            return;
        }

        match tag {
            PlantGrowthToken::GrowthName => {
                self.name = Name::from_value(value);
            }
            PlantGrowthToken::GrowthItem => {
                self.item = value.to_string();
            }
            PlantGrowthToken::GrowthHostTile => {
                if self.host_tiles.is_none() {
                    self.host_tiles = Some(Vec::new());
                }
                let Some(part) = PLANT_PART_TOKENS.get(value) else {
                    warn!(
                        "PlantGrowthParsing: called `Option::unwrap()` on a `None` value for presumed plant part: {}",
                        value
                    );
                    return;
                };
                if let Some(host_tiles) = &mut self.host_tiles {
                    host_tiles.push(*part);
                }
            }
            PlantGrowthToken::GrowthTrunkHeightPercent => {
                let split: Vec<&str> = value.split(':').collect::<Vec<&str>>();
                if split.len() != 2 {
                    warn!(
                        "PlantGrowthParsing: called `Option::unwrap()` on a `None` value for presumed plant part: {}",
                        value
                    );
                    return;
                }
                let percentage: i32 = match split.first().unwrap_or(&"").parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("min_value parsing error\n{:?}", e);
                        return;
                    }
                };
                let dir: i32 = match split.get(1).unwrap_or(&"").parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("max_value parsing error\n{:?}", e);
                        return;
                    }
                };
                self.trunk_height_percentage = Some([percentage, dir]);
            }
            PlantGrowthToken::GrowthDensity => {
                self.density = Some(value.parse().unwrap_or_default());
            }
            PlantGrowthToken::GrowthTiming => {
                let split: Vec<&str> = value.split(':').collect::<Vec<&str>>();
                if split.len() != 2 {
                    warn!(
                        "PlantGrowthParsing: called `Option::unwrap()` on a `None` value for presumed plant part: {}",
                        value
                    );
                    return;
                }
                let start: u32 = match split.first().unwrap_or(&"").parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("min_value parsing error\n{:?}", e);
                        return;
                    }
                };
                let end: u32 = match split.get(1).unwrap_or(&"").parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("max_value parsing error\n{:?}", e);
                        return;
                    }
                };
                self.timing = Some([start, end]);
            }
            PlantGrowthToken::GrowthPrint => {
                self.print = Some(value.to_string());
            }
            _ => {
                // If we don't recognize the tag, we just add it to the list.
                if self.tags.is_none() {
                    self.tags = Some(Vec::new());
                }
                if let Some(tags) = &mut self.tags {
                    tags.push(*tag);
                }
            }
        }
    }
}
