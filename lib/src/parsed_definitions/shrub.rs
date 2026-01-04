//! Shrub definition and parsing.

use tracing::{error, warn};

use crate::{
    color::Color,
    default_checks,
    raw_definitions::SHRUB_TOKENS,
    seed_material::SeedMaterial,
    tags::{SeasonTag, ShrubTag},
};

/// A shrub in the raws.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Shrub {
    /// Allows the plant to grow in farm plots during the given season.
    /// If the plant is a surface plant, allows it to grow in the wild during this season; wild surface plants without
    /// this token will disappear at the beginning of the season. Underground plants grow wild in all seasons, regardless
    /// of their season tokens.
    /// Default: empty (plant will not grow in farm plots)
    #[serde(skip_serializing_if = "Option::is_none")]
    growing_season: Option<Vec<SeasonTag>>,
    /// How long the plant takes to grow to harvest in a farm plot. Unit hundreds of ticks.
    /// There are 1008 GROWDUR units in a season. Defaults to 300.
    #[serde(skip_serializing_if = "Option::is_none")]
    grow_duration: Option<u32>,
    /// Has no known effect. Previously set the value of the harvested plant.
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<u32>,
    /// The tile used when the plant is harvested whole, or is ready to be picked from a farm plot. May either be a cp437
    /// tile number, or a character between single quotes. See character table. Defaults to 231 (τ).
    #[serde(skip_serializing_if = "Option::is_none")]
    picked_tile: Option<u8>,
    /// The tile used when a plant harvested whole has wilted. Defaults to 169 (⌐).
    #[serde(skip_serializing_if = "Option::is_none")]
    dead_picked_tile: Option<u8>,
    /// The tile used to represent this plant when it is wild, alive, and has no growths. Defaults to 34 (").
    #[serde(skip_serializing_if = "Option::is_none")]
    shrub_tile: Option<u8>,
    /// The tile used to represent this plant when it is dead in the wild. Defaults to 34 (").
    #[serde(skip_serializing_if = "Option::is_none")]
    dead_shrub_tile: Option<u8>,
    /// The maximum stack size collected when gathered via herbalism (possibly also from farm plots?). Defaults to 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_size: Option<u32>,
    /// The color of the plant when it has been picked whole, or when it is ready for harvest in a farm plot. Defaults to 2:0:0 (dark green).
    #[serde(skip_serializing_if = "Option::is_none")]
    picked_color: Option<Color>,
    /// The color of the plant when it has been picked whole, but has wilted. Defaults to 0:0:1 (dark gray).
    #[serde(skip_serializing_if = "Option::is_none")]
    dead_picked_color: Option<Color>,
    /// The color of the plant when it is alive, wild, and has no growths. Defaults to 2:0:0 (dark green).
    #[serde(skip_serializing_if = "Option::is_none")]
    shrub_color: Option<Color>,
    /// The color of the plant when it is dead in the wild. Defaults to 6:0:0 (brown).
    #[serde(skip_serializing_if = "Option::is_none")]
    dead_shrub_color: Option<Color>,
    /// The shrub will drown once the water on its tile reaches this level. Defaults to 4.
    #[serde(skip_serializing_if = "Option::is_none")]
    shrub_drown_level: Option<u8>,

    // Todo: fix these with actual values (materials and seed)
    /// Names a drink made from the plant, allowing it to be used in entity resources.
    /// Previously also permitted brewing the plant into alcohol made of this material.
    /// Now, a `MATERIAL_REACTION_PRODUCT` of type `DRINK_MAT` should be used on the proper plant material.
    #[serde(skip_serializing_if = "Option::is_none")]
    drink: Option<String>,
    /// Permits milling the plant at a quern or millstone into a powder made of this material and allows its use in entity resources.
    /// Said material should have `[POWDER_MISC_PLANT]` to permit proper stockpiling. This token makes the whole plant harvestable regardless
    /// of which material is designated for milling.
    /// For plants with millable growths, use only `MATERIAL_REACTION_PRODUCT` or `ITEM_REACTION_PRODUCT` tokens to define the milling products.
    #[serde(skip_serializing_if = "Option::is_none")]
    mill: Option<String>,
    /// Permits processing the plant at a farmer's workshop to yield threads made of this material and allows its use in entity resources.
    /// Said material should have `[THREAD_PLANT]` to permit proper stockpiling.
    #[serde(skip_serializing_if = "Option::is_none")]
    thread: Option<String>,
    /// Causes the plant to yield plantable seeds made of this material and having these properties.
    /// Said material should have `[SEED_MAT]` to permit proper stockpiling.
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<SeedMaterial>,
    /// Permits processing the plant into a vial at a still to yield extract made of this material.
    /// Said material should have `[EXTRACT_STORAGE:FLASK]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    extract_still_vial: Option<String>,
    /// Permits processing the plant into a vial at a farmer's workshop to yield extract made of this material.
    /// Said material should have `[EXTRACT_STORAGE:VIAL]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    extract_vial: Option<String>,
    /// Permits processing the plant into a barrel at a farmer's workshop to yield extract made of this material.
    /// Said material should have `[EXTRACT_STORAGE:BARREL]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    extract_barrel: Option<String>,
}

impl Shrub {
    /// Creates a new Shrub with default values.
    ///
    /// # Returns
    ///
    /// * `Shrub` - The default Shrub
    #[must_use]
    pub fn new() -> Self {
        Self {
            grow_duration: Some(300),
            picked_tile: Some(231),
            dead_picked_tile: Some(169),
            shrub_tile: Some(34),
            dead_shrub_tile: Some(34),
            cluster_size: Some(5),
            shrub_drown_level: Some(4),
            ..Self::default()
        }
    }

    /// Function to "clean" the raw. This is used to remove any empty list or strings,
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
    /// * `Shrub` - The cleaned Shrub
    #[must_use]
    #[allow(clippy::cognitive_complexity)]
    pub fn cleaned(&self) -> Self {
        let mut cleaned = self.clone();

        if let Some(growing_season) = &cleaned.growing_season
            && growing_season.is_empty()
        {
            cleaned.growing_season = None;
        }
        if default_checks::is_default_grow_duration(cleaned.grow_duration) {
            cleaned.grow_duration = None;
        }
        if default_checks::is_zero(cleaned.value) {
            cleaned.value = None;
        }
        if default_checks::is_default_picked_tile(cleaned.picked_tile) {
            cleaned.picked_tile = None;
        }
        if default_checks::is_default_dead_picked_tile(cleaned.dead_picked_tile) {
            cleaned.dead_picked_tile = None;
        }
        if default_checks::is_default_shrub_tile(cleaned.shrub_tile) {
            cleaned.shrub_tile = None;
        }
        if default_checks::is_default_dead_shrub_tile(cleaned.dead_shrub_tile) {
            cleaned.dead_shrub_tile = None;
        }
        if default_checks::is_default_cluster_size(cleaned.cluster_size) {
            cleaned.cluster_size = None;
        }
        if let Some(picked_color) = &cleaned.picked_color
            && picked_color.is_default()
        {
            cleaned.picked_color = None;
        }
        if let Some(dead_picked_color) = &cleaned.dead_picked_color
            && dead_picked_color.is_default()
        {
            cleaned.dead_picked_color = None;
        }
        if let Some(shrub_color) = &cleaned.shrub_color
            && shrub_color.is_default()
        {
            cleaned.shrub_color = None;
        }
        if let Some(dead_shrub_color) = &cleaned.dead_shrub_color
            && dead_shrub_color.is_default()
        {
            cleaned.dead_shrub_color = None;
        }
        if default_checks::is_default_shrub_drown_level(cleaned.shrub_drown_level) {
            cleaned.shrub_drown_level = None;
        }
        if let Some(drink) = &cleaned.drink
            && drink.is_empty()
        {
            cleaned.drink = None;
        }
        if let Some(mill) = &cleaned.mill
            && mill.is_empty()
        {
            cleaned.mill = None;
        }
        if let Some(thread) = &cleaned.thread
            && thread.is_empty()
        {
            cleaned.thread = None;
        }
        if let Some(seed) = &cleaned.seed
            && seed.is_empty()
        {
            cleaned.seed = None;
        }
        if let Some(extract_still_vial) = &cleaned.extract_still_vial
            && extract_still_vial.is_empty()
        {
            cleaned.extract_still_vial = None;
        }
        if let Some(extract_vial) = &cleaned.extract_vial
            && extract_vial.is_empty()
        {
            cleaned.extract_vial = None;
        }
        if let Some(extract_barrel) = &cleaned.extract_barrel
            && extract_barrel.is_empty()
        {
            cleaned.extract_barrel = None;
        }

        cleaned
    }
    /// Parses a tag and sets the appropriate field.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the tag
    /// * `value` - The value of the tag
    #[allow(clippy::too_many_lines)]
    pub fn parse_tag(&mut self, key: &str, value: &str) {
        let Some(tag) = SHRUB_TOKENS.get(key) else {
            warn!("Unknown shrub token: {}", key);
            return;
        };

        match tag {
            ShrubTag::Spring => {
                if self.growing_season.is_none() {
                    self.growing_season = Some(Vec::new());
                }

                if let Some(growing_season) = self.growing_season.as_mut() {
                    growing_season.push(SeasonTag::Spring);
                }
            }
            ShrubTag::Summer => {
                if self.growing_season.is_none() {
                    self.growing_season = Some(Vec::new());
                }

                if let Some(growing_season) = self.growing_season.as_mut() {
                    growing_season.push(SeasonTag::Summer);
                }
            }
            ShrubTag::Autumn => {
                if self.growing_season.is_none() {
                    self.growing_season = Some(Vec::new());
                }

                if let Some(growing_season) = self.growing_season.as_mut() {
                    growing_season.push(SeasonTag::Autumn);
                }
            }
            ShrubTag::Winter => {
                if self.growing_season.is_none() {
                    self.growing_season = Some(Vec::new());
                }

                if let Some(growing_season) = self.growing_season.as_mut() {
                    growing_season.push(SeasonTag::Winter);
                }
            }
            ShrubTag::GrowDuration => {
                self.grow_duration = Some(match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("grow_duration parsing error\n{:?}", e);
                        return;
                    }
                });
            }
            ShrubTag::Value => {
                self.value = Some(match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("value parsing error\n{:?}", e);
                        return;
                    }
                });
            }
            ShrubTag::PickedTile => {
                self.picked_tile = Some(match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("picked_tile parsing error\n{:?}", e);
                        return;
                    }
                });
            }
            ShrubTag::DeadPickedTile => {
                self.dead_picked_tile = Some(match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("dead_picked_tile parsing error\n{:?}", e);
                        return;
                    }
                });
            }
            ShrubTag::ShrubTile => {
                self.shrub_tile = Some(match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("shrub_tile parsing error\n{:?}", e);
                        return;
                    }
                });
            }
            ShrubTag::DeadShrubTile => {
                self.dead_shrub_tile = Some(match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("dead_shrub_tile parsing error\n{:?}", e);
                        return;
                    }
                });
            }
            ShrubTag::ClusterSize => {
                self.cluster_size = Some(match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("cluster_size parsing error\n{:?}", e);
                        return;
                    }
                });
            }
            ShrubTag::PickedColor => {
                self.picked_color = Some(Color::from_value(value));
            }
            ShrubTag::DeadPickedColor => {
                self.dead_picked_color = Some(Color::from_value(value));
            }
            ShrubTag::ShrubColor => {
                self.shrub_color = Some(Color::from_value(value));
            }
            ShrubTag::DeadShrubColor => {
                self.dead_shrub_color = Some(Color::from_value(value));
            }
            ShrubTag::ShrubDrownLevel => {
                self.shrub_drown_level = Some(match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("shrub_drown_level parsing error\n{:?}", e);
                        return;
                    }
                });
            }
            ShrubTag::Drink => {
                self.drink = Some(String::from(value));
            }
            ShrubTag::Mill => {
                self.mill = Some(String::from(value));
            }
            ShrubTag::Thread => {
                self.thread = Some(String::from(value));
            }
            ShrubTag::Seed => {
                self.seed = Some(SeedMaterial::from_value(value));
            }
            ShrubTag::ExtractStillVial => {
                self.extract_still_vial = Some(String::from(value));
            }
            ShrubTag::ExtractVial => {
                self.extract_vial = Some(String::from(value));
            }
            ShrubTag::ExtractBarrel => {
                self.extract_barrel = Some(String::from(value));
            }
            ShrubTag::Unknown => {
                warn!("Unknown shrub token: {}", key);
            }
        }
    }
}
