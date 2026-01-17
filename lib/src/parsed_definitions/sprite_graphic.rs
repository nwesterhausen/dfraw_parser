//! A module that contains the `SpriteGraphic` struct and its implementation.

use dfraw_parser_proc_macros::IsEmpty;
use itertools::Itertools;
use tracing::warn;

use crate::{
    Dimensions,
    raw_definitions::{CONDITION_TOKENS, GRAPHIC_TYPE_TOKENS},
    tags::{ColorModificationTag, ConditionTag, GraphicTypeTag},
};

/// A struct representing a sprite graphic.
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
pub struct SpriteGraphic {
    primary_condition: ConditionTag,
    tile_page_id: String,
    offset: Dimensions,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[is_empty(only_if_none)]
    color: Option<ColorModificationTag>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    large_image: Option<bool>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    offset2: Option<Dimensions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[is_empty(only_if_none)]
    secondary_condition: Option<ConditionTag>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    color_pallet_swap: Option<u32>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    target_identifier: Option<String>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    extra_descriptor: Option<String>,
}

impl SpriteGraphic {
    #[must_use]
    pub fn get_offset(&self) -> Dimensions {
        self.offset
    }
    #[must_use]
    pub fn get_offset2(&self) -> Option<Dimensions> {
        self.offset2
    }
    #[must_use]
    pub fn get_primary_condition(&self) -> ConditionTag {
        self.primary_condition
    }
    #[must_use]
    pub fn get_secondary_condition(&self) -> ConditionTag {
        match self.secondary_condition {
            Some(condition) => condition,
            None => ConditionTag::None,
        }
    }
    #[must_use]
    pub fn get_target_identifier(&self) -> &str {
        match self.target_identifier.as_ref() {
            Some(target) => target.as_str(),
            None => "",
        }
    }
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
            GraphicTypeTag::Creature
            | GraphicTypeTag::CreatureCaste
            | GraphicTypeTag::StatueCreature
            | GraphicTypeTag::StatueCreatureCaste
            | GraphicTypeTag::StatuesSurfaceGiant => {
                // parse creature
                Self::parse_creature_sprite_from_token(&token)
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

        let offset_x: u32 = match tile_offset_x.parse() {
            Ok(n) => n,
            Err(_e) => {
                warn!(
                    "parse_plant_from_token: Failed to parse {} as offset_x, {}",
                    tile_offset_x, token
                );
                return None;
            }
        };

        let offset_y: u32 = match tile_offset_y.parse() {
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
    #[tracing::instrument]
    fn parse_tile_with_color_pallet_from_value(value: &str) -> Option<Self> {
        // .[TOOL_GRAPHICS_WOOD:        1:      ITEM_BOOKCASE:      0:      0]
        // (     key                color_id    tile_page_id    offset_x   offset_y)
        let mut split = value.split(':');

        let color_id: u32 = match split.next() {
            Some(v) => match v.parse() {
                Ok(n) => n,
                Err(_e) => {
                    if v == "ALL" {
                        0
                    } else {
                        warn!("Failed to parse {v}");
                        return None;
                    }
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

        let offset_x: u32 = match tile_offset_x.parse() {
            Ok(n) => n,
            Err(_e) => {
                warn!(
                    "parse_tile_with_color_pallet_from_value: Failed to parse {} as offset_x {}",
                    tile_offset_x, value
                );
                return None;
            }
        };

        let offset_y: u32 = match tile_offset_y.parse() {
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

        let offset_x: u32 = match tile_offset_x.parse() {
            Ok(n) => n,
            Err(_e) => {
                warn!(
                    "parse_tile_from_value: Failed to parse {} as offset_x {}",
                    tile_offset_x, value
                );
                return None;
            }
        };

        let offset_y: u32 = match tile_offset_y.parse() {
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

        let offset_x: u32 = match tile_offset_x.parse() {
            Ok(n) => n,
            Err(_e) => {
                warn!(
                    "parse_tile_with_extra_descriptor_from_value: Failed to parse {} as offset_x {}",
                    tile_offset_x, value
                );
                return None;
            }
        };

        let offset_y: u32 = match tile_offset_y.parse() {
            Ok(n) => n,
            Err(_e) => {
                warn!(
                    "parse_tile_with_extra_descriptor_from_value: Failed to parse {} as offset_y {}",
                    tile_offset_y, value
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

    /// Parses a sprite graphic for anything about a creature.
    ///
    /// Takes the key condition and uses it to confirm we know how to parse it.
    #[tracing::instrument]
    fn parse_creature_sprite_from_token(token: &str) -> Option<Self> {
        let mut tokens: Vec<&str> = token.split(':').rev().collect();
        // reversed token list, so pop will remove the first token
        let first_token = tokens.pop().unwrap_or_default();
        let Some(key_condition) = CONDITION_TOKENS.get(first_token) else {
            warn!("no condition token found '{first_token}'");
            return None;
        };
        tokens.reverse();
        let token_str = tokens.as_slice().join(":");

        Self::parse_creature_sprite_from_keyed_token(key_condition, &token_str)
    }

    /// Parses a sprite graphic that has one of the 6 accepted basic condition tags:
    /// `DEFAULT`,`CHILD`,`ANIMATED`,`CORPSE`,`LIST_ICON`, or `CDI_LIST_ICON`
    ///
    /// ## Default Sprite Example:
    ///
    /// ```txt
    /// [CREATURE_GRAPHICS:GORLAK]
    ///     [DEFAULT:CREATURES_UNDERGROUND:0:11:AS_IS]
    ///     [key-----0---------------------1-2--3----] len: 4
    ///     [DEFAULT:CREATURES_UNDERGROUND:0:11:AS_IS:CORPSE]
    ///     [key-----0---------------------1-2--3-----4-----] len: 5
    /// ```
    ///
    /// ## Large Sprite Example:
    ///
    /// ```txt
    /// [CREATURE_GRAPHICS:CAVE_DRAGON]
    ///     [DEFAULT:CREATURES_UNDERGROUND_LARGE:LARGE_IMAGE:0:0:2:1:AS_IS]
    ///     [key-----0---------------------------1-----------2-3-4-5-6----] len: 7
    ///     [DEFAULT:CREATURES_UNDERGROUND_LARGE:LARGE_IMAGE:0:0:2:1:AS_IS:CHILD]
    ///     [key-----0---------------------------1-----------2-3-4-5-6-----7----] len: 8
    /// ```
    ///
    /// Alternatively, statues are tall and so define it as a larger size without the indicator
    /// flag `LARGE_IMAGE`.
    ///
    /// ```txt
    /// [STATUE_CREATURE_GRAPHICS:SERPENT_MAN]
    ///     [DEFAULT:STATUES_UNDERGROUND_CIV:0:8:0:9]
    ///     [key-----0-----------------------1-2-3-4] len: 5
    /// ```
    ///
    /// ## List Icon Example:
    ///
    /// Typically only exists if it is a giant/oddly sized normal sprite that wouldn't fit.
    ///
    /// ```txt
    /// [LIST_ICON:CREATURES_SURFACE_GIANT:10:54]
    /// [LIST_ICON:CREATURES_ANIMAL_PEOPLE_TALL:2:76]
    /// [key-------0----------------------------1-2-] len: 3
    /// ```
    ///
    /// ## CDI List Icon Example:
    ///
    /// An interaction list icon. Supercedes any list icon for the interaction list. Also replaces the normal
    /// sprite if a CDI_LIST_ICON exists for specific interaction.
    ///
    /// ```txt
    /// [CREATURE_GRAPHICS:ELEMENTMAN_FIRE]
    ///      [CDI_LIST_ICON:HURL_FIREBALL:CREATURE_ABILITY_LIST_ICONS:20:0:4:3]
    ///      [CDI_LIST_ICON:SPRAY_FIRE_JET:CREATURE_ABILITY_LIST_ICONS:24:0:4:3]
    ///      [key-----------0--------------1---------------------------2--3-4-5] len: 6
    /// ```
    #[tracing::instrument]
    fn parse_creature_sprite_from_keyed_token(
        key_condition: &ConditionTag,
        token: &str,
    ) -> Option<Self> {
        let primary_condition = *key_condition;
        let tokens: Vec<&str> = token.split(':').collect();
        let num_args = tokens.len();

        match num_args {
            // List Icon or Creature without color
            3 => {
                if key_condition == &ConditionTag::ListIcon {
                    return Some(Self {
                        primary_condition,
                        // pos 1 (idx 0)
                        tile_page_id: String::from(*tokens.first().unwrap_or(&"UNKNOWN")),
                        // pos 2-3 (idx 1-2)
                        offset: Dimensions::from_two_tokens(
                            tokens.get(1).unwrap_or(&"0"),
                            tokens.get(2).unwrap_or(&"0"),
                        ),
                        ..Default::default()
                    });
                }

                Some(Self {
                    primary_condition,
                    // pos 1 (idx 0)
                    tile_page_id: String::from(*tokens.first().unwrap_or(&"UNKNOWN")),
                    // pos 2-3 (idx 1-2)
                    offset: Dimensions::from_two_tokens(
                        tokens.get(1).unwrap_or(&"0"),
                        tokens.get(2).unwrap_or(&"0"),
                    ),
                    ..Default::default()
                })
            }
            // Only creature
            4 => {
                Some(Self {
                    primary_condition,
                    // pos 1 (idx 0)
                    tile_page_id: String::from(*tokens.first().unwrap_or(&"UNKNOWN")),
                    // pos 2-3 (idx 1-2)
                    offset: Dimensions::from_two_tokens(
                        tokens.get(1).unwrap_or(&"0"),
                        tokens.get(2).unwrap_or(&"0"),
                    ),
                    // pos 4 (idx 3)
                    // always is AS_IS (as of df53.08)
                    color: Some(ColorModificationTag::AsIs),
                    ..Default::default()
                })
            }
            // Can be creature sprite + 2nd condition or statue
            5 => {
                // If the final token is not `u32` then we have a secondary condition.
                if tokens.get(4).unwrap_or(&"!").parse::<u32>().is_err() {
                    // pos 5 (idx 4) (only sometimes present)
                    let secondary_condition: Option<ConditionTag> =
                        CONDITION_TOKENS.get(tokens.get(4).unwrap_or(&"")).copied();
                    return Some(Self {
                        primary_condition,
                        // pos 1 (idx 0)
                        tile_page_id: String::from(*tokens.first().unwrap_or(&"UNKNOWN")),
                        // pos 2-3 (idx 1-2)
                        offset: Dimensions::from_two_tokens(
                            tokens.get(1).unwrap_or(&"0"),
                            tokens.get(2).unwrap_or(&"0"),
                        ),
                        // pos 4 (idx 3)
                        // always is AS_IS (as of df53.08)
                        color: Some(ColorModificationTag::AsIs),
                        // pos 5 (idx 4) (only sometimes present)
                        secondary_condition,
                        ..Default::default()
                    });
                }

                // Tall/wide Sprite
                // Used for statues (tall)
                Some(Self {
                    primary_condition,
                    // pos 1 (idx 0)
                    tile_page_id: String::from(*tokens.first().unwrap_or(&"UNKNOWN")),
                    // pos 2-3 (idx 1-2)
                    offset: Dimensions::from_two_tokens(
                        tokens.get(1).unwrap_or(&"0"),
                        tokens.get(2).unwrap_or(&"0"),
                    ),
                    // pos 4 (idx 3)
                    // always is AS_IS (as of df53.08)
                    color: Some(ColorModificationTag::AsIs),
                    ..Default::default()
                })
            }
            // Only cdi list icon
            6 => {
                Some(Self {
                    primary_condition,
                    // pos 1 (idx 0)
                    extra_descriptor: Some(String::from(*tokens.first().unwrap_or(&"UNKNOWN"))),
                    // pos 2 (idx 1)
                    tile_page_id: String::from(*tokens.get(1).unwrap_or(&"UNKNOWN")),
                    // pos 3-4 (idx 2-3)
                    offset: Dimensions::from_two_tokens(
                        tokens.get(2).unwrap_or(&"0"),
                        tokens.get(3).unwrap_or(&"0"),
                    ),
                    // pos 5-6 (idx 4-5)
                    offset2: Some(Dimensions::from_two_tokens(
                        tokens.get(4).unwrap_or(&"0"),
                        tokens.get(5).unwrap_or(&"0"),
                    )),
                    ..Default::default()
                })
            }
            // Only large creature (and lrg + 2nd cond)
            7 | 8 => {
                // pos 8 (idx 7) (only sometimes present)
                let secondary_condition: Option<ConditionTag> = if num_args == 8 {
                    CONDITION_TOKENS.get(tokens.get(7).unwrap_or(&"")).copied()
                } else {
                    None
                };
                // Large Sprite
                Some(Self {
                    primary_condition,
                    // pos 1 (idx 0)
                    tile_page_id: String::from(*tokens.first().unwrap_or(&"UNKNOWN")),
                    // pos 2 (idx 1)
                    // literally: LARGE_IMAGE (discarded)
                    large_image: Some(true),
                    // pos 3-4 (idx 2-3)
                    offset: Dimensions::from_two_tokens(
                        tokens.get(2).unwrap_or(&"0"),
                        tokens.get(3).unwrap_or(&"0"),
                    ),
                    // pos 5-6 (idx 4-5)
                    offset2: Some(Dimensions::from_two_tokens(
                        tokens.get(4).unwrap_or(&"0"),
                        tokens.get(5).unwrap_or(&"0"),
                    )),
                    // pos 7 (idx 6)
                    // always is AS_IS (as of df53.08)
                    color: Some(ColorModificationTag::AsIs),
                    secondary_condition,
                    ..Default::default()
                })
            }
            _ => {
                warn!("Fell through to none");
                None
            }
        }
    }
}
