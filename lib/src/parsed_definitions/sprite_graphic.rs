//! A module that contains the `SpriteGraphic` struct and its implementation.

use itertools::Itertools;
use tracing::warn;

use crate::{
    default_checks,
    dimensions::Dimensions,
    raw_definitions::{CONDITION_TOKENS, GRAPHIC_TYPE_TOKENS},
    tags::{ColorModificationTag, ConditionTag, GraphicTypeTag},
};

/// A struct representing a sprite graphic.
#[allow(clippy::module_name_repetitions)]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SpriteGraphic {
    primary_condition: ConditionTag,
    tile_page_id: String,
    offset: Dimensions,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<ColorModificationTag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    large_image: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset2: Option<Dimensions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_condition: Option<ConditionTag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color_pallet_swap: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra_descriptor: Option<String>,
}

impl SpriteGraphic {
    /// Get the tile page ID.
    ///
    /// # Returns
    ///
    /// A string slice containing the tile page ID.
    #[must_use]
    pub fn get_tile_page_id(&self) -> &str {
        self.tile_page_id.as_str()
    }
    /// Create a new sprite graphic by parsing a token.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice containing the key.
    /// * `value` - A string slice containing the value.
    /// * `graphic_type` - The graphic type.
    ///
    /// # Returns
    ///
    /// An option containing the sprite graphic.
    #[must_use]
    pub fn from_token(key: &str, value: &str, graphic_type: GraphicTypeTag) -> Option<Self> {
        // Recombine token for parsing
        let token = format!("{key}:{value}");
        let specific_graphic_type = GRAPHIC_TYPE_TOKENS
            .get(key)
            .copied()
            .unwrap_or(graphic_type);

        match specific_graphic_type {
            GraphicTypeTag::Creature | GraphicTypeTag::CreatureCaste => {
                // parse creature
                Self::parse_creature_from_token(&token)
            }
            GraphicTypeTag::Plant => {
                // parse plant
                Self::parse_plant_from_token(&token)
            }
            GraphicTypeTag::ToolWood
            | GraphicTypeTag::ToolGlass
            | GraphicTypeTag::ToolMetal
            | GraphicTypeTag::ToolStone
            | GraphicTypeTag::ToolWoodVariant
            | GraphicTypeTag::ToolGlassVariant
            | GraphicTypeTag::ToolMetalVariant
            | GraphicTypeTag::ToolStoneVariant
            | GraphicTypeTag::ToolDamage => Self::parse_tile_with_color_pallet_from_value(value),
            GraphicTypeTag::ToolShape
            | GraphicTypeTag::ShapeLargeGem
            | GraphicTypeTag::ShapeSmallGem => {
                Self::parse_tile_with_extra_descriptor_from_value(value)
            }
            GraphicTypeTag::StatueCreature
            | GraphicTypeTag::StatueCreatureCaste
            | GraphicTypeTag::StatuesSurfaceGiant => Self::parse_creature_statue_from_token(&token),
            GraphicTypeTag::Template
            | GraphicTypeTag::CustomWorkshop
            | GraphicTypeTag::AddTool
            | GraphicTypeTag::Ammo
            | GraphicTypeTag::SiegeAmmo
            | GraphicTypeTag::Weapon => {
                // parse template ""
                Some(Self {
                    primary_condition: ConditionTag::CopyOfTemplate,
                    tile_page_id: format!("{key}:{value}"),
                    ..Self::default()
                })
            }
            _ => {
                // Assume most are tiles
                if let Some(v) = Self::parse_tile_from_value(value) {
                    return Some(v);
                }
                warn!(
                    "Failed to parse {} as SpriteGraphic, unknown key {}",
                    value, key
                );
                None
            }
        }
    }
    fn parse_plant_from_token(token: &str) -> Option<Self> {
        // [SHRUB:PLANT_STANDARD:0:0]
        // [PICKED:PLANT_STANDARD:1:0]
        // [Condition, TilePageId, OffsetX, OffsetY]
        let mut split = token.split(':');

        let sprite_condition = match split.next() {
            Some(v) => *CONDITION_TOKENS.get(v).unwrap_or(&ConditionTag::None),
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
        let tile_offset_x = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };
        let tile_offset_y = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };

        let offset_x: i32 = match tile_offset_x.parse() {
            Ok(n) => n,
            Err(_e) => {
                warn!(
                    "parse_plant_from_token: Failed to parse {} as offset_x, {}",
                    tile_offset_x, token
                );
                return None;
            }
        };

        let offset_y: i32 = match tile_offset_y.parse() {
            Ok(n) => n,
            Err(_e) => {
                warn!(
                    "parse_plant_from_token: Failed to parse {} as offset_y, {}",
                    tile_offset_y, token
                );
                return None;
            }
        };

        Some(Self {
            primary_condition: sprite_condition,
            tile_page_id,
            offset: Dimensions::from_xy(offset_x, offset_y),
            ..Self::default()
        })
    }
    fn parse_tile_with_color_pallet_from_value(value: &str) -> Option<Self> {
        // .[TOOL_GRAPHICS_WOOD:        1:      ITEM_BOOKCASE:      0:      0]
        // (     key                color_id    tile_page_id    offset_x   offset_y)
        let mut split = value.split(':');

        let color_id: u32 = match split.next() {
            Some(v) => match v.parse() {
                Ok(n) => n,
                Err(_e) => {
                    warn!(
                        "parse_tile_with_color_pallet_from_value: Failed to parse {} as color_id {}",
                        v,
                        value
                    );
                    return None;
                }
            },
            _ => {
                return None;
            }
        };

        let tile_sheet = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };
        let tile_offset_x = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };
        let tile_offset_y = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };

        let offset_x: i32 = match tile_offset_x.parse() {
            Ok(n) => n,
            Err(_e) => {
                warn!(
                    "parse_tile_with_color_pallet_from_value: Failed to parse {} as offset_x {}",
                    tile_offset_x, value
                );
                return None;
            }
        };

        let offset_y: i32 = match tile_offset_y.parse() {
            Ok(n) => n,
            Err(_e) => {
                warn!(
                    "parse_tile_with_color_pallet_from_value: Failed to parse {} as offset_y {}",
                    tile_offset_y, value
                );
                return None;
            }
        };

        Some(Self {
            tile_page_id: tile_sheet,
            offset: Dimensions::from_xy(offset_x, offset_y),
            color_pallet_swap: Some(color_id),
            ..Self::default()
        })
    }
    fn parse_tile_from_value(value: &str) -> Option<Self> {
        // .[TOY_GRAPHICS:              ITEM_TOY:           1:     4:          ITEM_TOY_MINIFORGE:GLASS]
        // .[ARMOR_GRAPHICS:            ITEMS4:             1:     4:          ITEM_ARMOR_CAPE]
        // .[TOOL_GRAPHICS:             TOOLS:              0:     14:         ITEM_TOOL_HONEYCOMB]
        // .[WEAPON_GRAPHICS_DEFAULT:   WEAPONS:            2:     20]               (none)
        // .[WEAPON_GRAPHICS_UPRIGHT_1T:UPRIGHT_WEAPONS:    0:     5]                (none)
        // .[ITEMS2:                    1:          20:             ITEM_SLAB_ENGRAVED]
        // (     key                    tile_page_id    offset_x   offset_y    Option<tile_target_identifier>)
        let mut split = value.split(':');

        let tile_page_id = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };

        let tile_offset_x = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };

        let tile_offset_y = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };

        // Target identifier is optional
        let target_identifier = split.join(":");

        let offset_x: i32 = match tile_offset_x.parse() {
            Ok(n) => n,
            Err(_e) => {
                warn!(
                    "parse_tile_from_value: Failed to parse {} as offset_x {}",
                    tile_offset_x, value
                );
                return None;
            }
        };

        let offset_y: i32 = match tile_offset_y.parse() {
            Ok(n) => n,
            Err(_e) => {
                warn!(
                    "parse_tile_from_value: Failed to parse {} as offset_y {}",
                    tile_offset_y, value
                );
                return None;
            }
        };

        Some(Self {
            tile_page_id,
            offset: Dimensions::from_xy(offset_x, offset_y),
            target_identifier: Some(target_identifier),
            ..Self::default()
        })
    }
    fn parse_tile_with_extra_descriptor_from_value(value: &str) -> Option<Self> {
        // .[TOOL_GRAPHICS_SHAPE:       LONG_DIE_8:             ITEMS4:         2:          0]
        // .[SHAPE_GRAPHICS_LARGE_GEM:  BAGUETTE_CUT_GEM:       GEMS:           1:          0]
        // .[SHAPE_GRAPHICS_SMALL_GEM:  BAGUETTE_CUT_GEM:       SMALLGEMS:      0:          0]
        // (     key                    extra_descriptor      tile_page_id    offset_x   offset_y)
        let mut split = value.split(':');

        let extra_descriptor = match split.next() {
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

        let tile_offset_x = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };

        let tile_offset_y = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };

        // Target identifier is optional
        let target_identifier = split.join(":");

        let offset_x: i32 = match tile_offset_x.parse() {
            Ok(n) => n,
            Err(_e) => {
                warn!(
                    "parse_tile_with_extra_descriptor_from_value: Failed to parse {} as offset_x {}",
                    tile_offset_x,
                    value
                );
                return None;
            }
        };

        let offset_y: i32 = match tile_offset_y.parse() {
            Ok(n) => n,
            Err(_e) => {
                warn!(
                    "parse_tile_with_extra_descriptor_from_value: Failed to parse {} as offset_y {}",
                    tile_offset_y,
                    value
                );
                return None;
            }
        };

        Some(Self {
            tile_page_id,
            offset: Dimensions::from_xy(offset_x, offset_y),
            target_identifier: Some(target_identifier),
            extra_descriptor: Some(extra_descriptor),
            ..Self::default()
        })
    }
    fn parse_creature_statue_from_token(token: &str) -> Option<Self> {
        // [DEFAULT:    STATUES_LAYERED:        0:  0:  0:  1]
        // [DEFAULT:    STATUES_SURFACE_LARGE:  1:  0:  1:  1]
        //  condition   tile_page_id            x1  y1  x2  y2
        let mut split = token.split(':');

        let condition = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };

        let x1: i32 = match split.next() {
            Some(v) => match v.parse() {
                Ok(n) => n,
                Err(_e) => {
                    warn!(
                        "parse_creature_statue_from_token: Failed to parse {} as x1 {}",
                        v, token
                    );
                    return None;
                }
            },
            _ => {
                return None;
            }
        };
        let y1: i32 = match split.next() {
            Some(v) => match v.parse() {
                Ok(n) => n,
                Err(_e) => {
                    warn!(
                        "parse_creature_statue_from_token: Failed to parse {} as y1 {}",
                        v, token
                    );
                    return None;
                }
            },
            _ => {
                return None;
            }
        };
        let x2: i32 = match split.next() {
            Some(v) => match v.parse() {
                Ok(n) => n,
                Err(_e) => {
                    warn!(
                        "parse_creature_statue_from_token: Failed to parse {} as x2 {}",
                        v, token
                    );
                    return None;
                }
            },
            _ => {
                return None;
            }
        };
        let y2: i32 = match split.next() {
            Some(v) => match v.parse() {
                Ok(n) => n,
                Err(_e) => {
                    warn!(
                        "parse_creature_statue_from_token: Failed to parse {} as y2 {}",
                        v, token
                    );
                    return None;
                }
            },
            _ => {
                return None;
            }
        };

        let primary_condition = ConditionTag::from_token(condition.as_str()).unwrap_or_else(|| {
            warn!(
                "parse_creature_statue_from_token: Failed to parse {} as primary_condition in {}",
                condition, token
            );
            ConditionTag::None
        });

        Some(Self {
            primary_condition,
            tile_page_id: String::from("STATUES"),
            offset: Dimensions::from_xy(x1, y1),
            offset2: Some(Dimensions::from_xy(x2, y2)),
            ..Self::default()
        })
    }
    fn parse_creature_from_token(token: &str) -> Option<Self> {
        // [<condition>:<tile page identifier>:<x position>:<y position>:<color type>:<secondary condition>]
        //   0           1                      2            3             4            5
        // [<condition>:<tile page identifier>:LARGE_IMAGE:<x1>:<y1>:<x2>:<y2>:<color type>:<secondary condition>]
        //   0           1                      2          3    4     5    6    7            8

        // Based on the length of the split, we can determine if this is a large image or not
        let mut split = token.split(':');

        let condition = match split.next() {
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
            return Self::parse_large_creature_with_split(
                condition.as_str(),
                tile_page_id.as_str(),
                split.collect::<Vec<&str>>().as_slice(),
            );
        }

        // x1 actually is parsed from fourth_position_token
        let x1: i32 = match fourth_position_token.parse() {
            Ok(n) => n,
            Err(_e) => {
                warn!(
                    "parse_creature_from_token: Failed to parse {} as x1 {}",
                    fourth_position_token, token
                );
                return None;
            }
        };

        let y1: i32 = match split.next() {
            Some(v) => match v.parse() {
                Ok(n) => n,
                Err(_e) => {
                    warn!(
                        "parse_creature_from_token: Failed to parse {} as y1 {}",
                        v, token
                    );
                    return None;
                }
            },
            _ => {
                return None;
            }
        };

        let color = split.next().map_or(ColorModificationTag::AsIs, |v| {
            ColorModificationTag::from_token(v)
        });

        let primary_condition = ConditionTag::from_token(condition.as_str()).unwrap_or_else(|| {
            warn!(
                "Failed to parse {} as primary_condition in {}",
                condition, token
            );
            ConditionTag::None
        });

        let secondary_condition = split.next().map_or(ConditionTag::None, |v| {
            ConditionTag::from_token(v).unwrap_or_else(|| {
                warn!("Failed to parse {} as secondary_condition in {}", v, token);
                ConditionTag::None
            })
        });

        if primary_condition == ConditionTag::None {
            warn!(
                "Failed to parse {} as primary_condition large_animal_sprite {}",
                condition, tile_page_id
            );
            return None;
        }

        Some(Self {
            primary_condition,
            tile_page_id,
            offset: Dimensions::from_xy(x1, y1),
            color: Some(color),
            secondary_condition: Some(secondary_condition),
            ..Self::default()
        })
    }
    #[allow(clippy::too_many_lines)]
    fn parse_large_creature_with_split(
        condition: &str,
        tile_page_id: &str,
        split: &[&str],
    ) -> Option<Self> {
        // [<condition>:<tile page identifier>:LARGE_IMAGE:<x1>:<y1>:<x2>:<y2>:<color type>:<secondary condition>]
        //   0           1                      2          3    4     5    6    7            8
        let x1: i32 = match split.first() {
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

        let y1: i32 = match split.get(1) {
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

        let x2: i32 = match split.get(2) {
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

        let y2: i32 = match split.get(3) {
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

        let color = split.get(4).map_or(ColorModificationTag::AsIs, |v| {
            ColorModificationTag::from_token(v)
        });

        let primary_condition = ConditionTag::from_token(condition).unwrap_or_else(|| {
            warn!(
                "Failed to parse {} as primary_condition in {}",
                condition,
                split.join(":")
            );
            ConditionTag::None
        });

        let secondary_condition = split.get(5).map_or(ConditionTag::None, |v| {
            ConditionTag::from_token(v).unwrap_or_else(|| {
                warn!(
                    "Failed to parse {} as secondary_condition in {}",
                    v,
                    split.join(":")
                );
                ConditionTag::None
            })
        });

        if primary_condition == ConditionTag::None {
            warn!(
                "Failed to parse {} as primary_condition large_animal_sprite {}",
                condition, tile_page_id
            );
            return None;
        }

        Some(Self {
            primary_condition,
            tile_page_id: String::from(tile_page_id),
            offset: Dimensions::from_xy(x1, y1),
            color: Some(color),
            large_image: Some(true),
            offset2: Some(Dimensions::from_xy(x2, y2)),
            secondary_condition: Some(secondary_condition),
            ..Self::default()
        })
    }

    /// Function to "clean" the creature. This is used to remove any empty list or strings,
    /// and to remove any default values. By "removing" it means setting the value to None.
    ///
    /// This also will remove the metadata if `is_metadata_hidden` is true.
    ///
    /// Steps:
    /// - Set any metadata to None if `is_metadata_hidden` is true.
    /// - Set any empty string to None.
    /// - Set any empty list to None.
    /// - Set any default values to None.
    ///
    /// # Returns
    ///
    /// A new sprite graphic with the cleaned values.
    #[must_use]
    pub fn cleaned(&self) -> Self {
        let mut cleaned = self.clone();

        // Set any empty string to None.
        if let Some(extra_descriptor) = cleaned.extra_descriptor.as_ref() {
            if extra_descriptor.is_empty() {
                cleaned.extra_descriptor = None;
            }
        }

        // Set any empty string to None.
        if let Some(target_identifier) = cleaned.target_identifier.as_ref() {
            if target_identifier.is_empty() {
                cleaned.target_identifier = None;
            }
        }

        // Set any default values to None.
        if let Some(color) = cleaned.color.as_ref() {
            if color.is_default() {
                cleaned.color = None;
            }
        }

        // Set any default values to None.
        if let Some(offset2) = cleaned.offset2.as_ref() {
            if offset2.is_empty() {
                cleaned.offset2 = None;
            }
        }

        // Set any default values to None.
        if let Some(secondary_condition) = cleaned.secondary_condition.as_ref() {
            if secondary_condition.is_none() {
                cleaned.secondary_condition = None;
            }
        }

        // Set any default values to None.
        if default_checks::is_zero(cleaned.color_pallet_swap) {
            cleaned.color_pallet_swap = None;
        }

        cleaned
    }
}
