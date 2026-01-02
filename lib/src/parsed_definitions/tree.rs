//! Tree definition and parsing.

use tracing::{error, warn};

use crate::{
    color::Color,
    default_checks,
    name::Name,
    raw_definitions::TREE_TOKENS,
    tags::{TreeTag, TwigPlacementTag},
};

/// A struct representing a tree.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Tree {
    /// Tree will yield logs made of that material. Instead, if it's `[TREE:NONE]`, no logs will result.
    /// Materials are typically found in other raws..
    material: String,
    /// What the trunk of the tree is named
    #[serde(skip_serializing_if = "Option::is_none")]
    trunk_name: Option<Name>,
    /// The maximum z-level height of the trunk, starting from +2 z-levels above the ground.
    /// Valid values: 1-8
    /// Default: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    max_trunk_height: Option<u8>,
    /// Upper limit of trunk thickness, in tiles. Has a geometric effect on log yield.
    /// Valid values: 1-3
    /// Default: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    max_trunk_diameter: Option<u8>,
    /// The number of years the trunk takes to grow one z-level upward. Default: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    trunk_period: Option<u8>,
    /// The number of years the trunk takes to grow one tile wider. Default: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    trunk_width_period: Option<u8>,
    /// What thin branches of the tree are named.
    #[serde(skip_serializing_if = "Option::is_none")]
    branch_name: Option<Name>,
    /// How dense the branches grow on this tree.
    #[serde(skip_serializing_if = "Option::is_none")]
    branch_density: Option<u8>,
    /// The radius to which branches can reach. Appears to never reach further than seven tiles from the centre.
    /// Does not depend on the trunk branching amount or where trunks are.
    /// The values used in the game go from 0-3. Higher values than that can cause crashes.
    #[serde(skip_serializing_if = "Option::is_none")]
    branch_radius: Option<u8>,
    /// What thick branches of the tree are named.
    #[serde(skip_serializing_if = "Option::is_none")]
    heavy_branches_name: Option<Name>,
    /// Similar to `BRANCH_DENSITY` for thick branches. Default: 0
    #[serde(skip_serializing_if = "Option::is_none")]
    heavy_branch_density: Option<u8>,
    /// Similar as `BRANCH_DENSITY` for thick branches. Values outside 0-3 can cause crashes. Default: 0
    #[serde(skip_serializing_if = "Option::is_none")]
    heavy_branch_radius: Option<u8>,
    /// How much the trunk branches out. 0 makes the trunk straight (default)
    #[serde(skip_serializing_if = "Option::is_none")]
    trunk_branching: Option<u8>,
    /// What the roots of the tree are named.
    #[serde(skip_serializing_if = "Option::is_none")]
    root_name: Option<Name>,
    /// Density of the root growth. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    root_density: Option<u8>,
    /// How wide the roots reach out. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    root_radius: Option<u8>,
    /// What the twigs of the tree are named.
    #[serde(skip_serializing_if = "Option::is_none")]
    twigs_name: Option<Name>,
    /// Where twigs appear, defaults to `[SideBranches, AboveBranches]`
    #[serde(skip_serializing_if = "Option::is_none")]
    twigs_placement: Option<Vec<TwigPlacementTag>>,
    /// What this mushroom-cap is called. Only makes sense with `TREE_HAS_MUSHROOM_CAP`.
    #[serde(skip_serializing_if = "Option::is_none")]
    cap_name: Option<Name>,
    /// Similar to the other PERIOD tags, influences the rate of the mushroom cap growth. Only makes sense with `TREE_HAS_MUSHROOM_CAP`. Default: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    cap_period: Option<u8>,
    /// The radius of a mushroom cap. Only makes sense with `TREE_HAS_MUSHROOM_CAP`. Default: 0
    #[serde(skip_serializing_if = "Option::is_none")]
    cap_radius: Option<u8>,
    /// The tile used for trees of this type on the world map. Defaults to 24 (↑).
    #[serde(skip_serializing_if = "Option::is_none")]
    tree_tile: Option<String>,
    /// The tile used for (un)dead trees and deciduous trees (generally in winter) of this type. Defaults to 198 (╞).
    #[serde(skip_serializing_if = "Option::is_none")]
    dead_tree_tile: Option<String>,
    /// The tile used for saplings of this tree. Defaults to 231 (τ).
    #[serde(skip_serializing_if = "Option::is_none")]
    sapling_tile: Option<String>,
    /// The tile used for dead saplings of this tree. Defaults to 231 (τ).
    #[serde(skip_serializing_if = "Option::is_none")]
    dead_sapling_tile: Option<String>,
    /// The color of the tree on the map. Defaults to 2:0:0 (dark green).
    #[serde(skip_serializing_if = "Option::is_none")]
    tree_color: Option<Color>,
    /// The color of the tree on the map when (un)dead. Defaults to 0:0:1 (dark gray).
    #[serde(skip_serializing_if = "Option::is_none")]
    dead_tree_color: Option<Color>,
    /// The color of saplings of this tree. Defaults to 2:0:0 (dark green).
    #[serde(skip_serializing_if = "Option::is_none")]
    sapling_color: Option<Color>,
    /// The color of dead saplings of this tree. Defaults to 0:0:1 (dark gray).
    #[serde(skip_serializing_if = "Option::is_none")]
    dead_sapling_color: Option<Color>,
    /// The sapling of this tree will drown once the water on its tile reaches this level. Defaults to 4.
    #[serde(skip_serializing_if = "Option::is_none")]
    sapling_drown_level: Option<u8>,
    /// The water depth at which this tree will drown. Exact behavior is unknown. Defaults to 7.
    #[serde(skip_serializing_if = "Option::is_none")]
    tree_drown_level: Option<u8>,
    /// Token tags for the tree.
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<TreeTag>>,
}

impl Tree {
    /// Create a new `Tree` object with the given material.
    ///
    /// # Arguments
    ///
    /// * `material`: The material of the tree.
    ///
    /// # Returns
    ///
    /// A new `Tree` object with the given material.
    #[must_use]
    pub fn new(material: &str) -> Self {
        Self {
            material: material.to_string(),
            max_trunk_height: Some(1),
            max_trunk_diameter: Some(1),
            trunk_period: Some(1),
            trunk_width_period: Some(1),
            cap_period: Some(1),
            sapling_drown_level: Some(4),
            tree_drown_level: Some(7),
            twigs_placement: Some(vec![
                TwigPlacementTag::SideBranches,
                TwigPlacementTag::AboveBranches,
            ]),
            ..Default::default()
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
    /// A new `Tree` object with all the fields cleaned.
    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    #[must_use]
    pub fn cleaned(&self) -> Self {
        let mut cleaned = self.clone();

        if let Some(trunk_name) = &cleaned.trunk_name {
            if trunk_name.is_empty() {
                cleaned.trunk_name = None;
            }
        }
        if default_checks::is_one_u8(cleaned.max_trunk_height) {
            cleaned.max_trunk_height = None;
        }
        if default_checks::is_one_u8(cleaned.max_trunk_diameter) {
            cleaned.max_trunk_diameter = None;
        }
        if default_checks::is_one_u8(cleaned.trunk_period) {
            cleaned.trunk_period = None;
        }
        if default_checks::is_one_u8(cleaned.trunk_width_period) {
            cleaned.trunk_width_period = None;
        }
        if let Some(branch_name) = &cleaned.branch_name {
            if branch_name.is_empty() {
                cleaned.branch_name = None;
            }
        }
        if default_checks::is_zero_u8(cleaned.branch_density) {
            cleaned.branch_density = None;
        }
        if default_checks::is_zero_u8(cleaned.branch_radius) {
            cleaned.branch_radius = None;
        }
        if let Some(heavy_branches_name) = &cleaned.heavy_branches_name {
            if heavy_branches_name.is_empty() {
                cleaned.heavy_branches_name = None;
            }
        }
        if default_checks::is_zero_u8(cleaned.heavy_branch_density) {
            cleaned.heavy_branch_density = None;
        }
        if default_checks::is_zero_u8(cleaned.heavy_branch_radius) {
            cleaned.heavy_branch_radius = None;
        }
        if default_checks::is_zero_u8(cleaned.trunk_branching) {
            cleaned.trunk_branching = None;
        }
        if let Some(root_name) = &cleaned.root_name {
            if root_name.is_empty() {
                cleaned.root_name = None;
            }
        }
        if default_checks::is_zero_u8(cleaned.root_density) {
            cleaned.root_density = None;
        }
        if default_checks::is_zero_u8(cleaned.root_radius) {
            cleaned.root_radius = None;
        }
        if let Some(twigs_name) = &cleaned.twigs_name {
            if twigs_name.is_empty() {
                cleaned.twigs_name = None;
            }
        }
        if let Some(twigs_placement) = &cleaned.twigs_placement {
            if twigs_placement.is_empty() {
                cleaned.twigs_placement = None;
            }
        }
        if let Some(cap_name) = &cleaned.cap_name {
            if cap_name.is_empty() {
                cleaned.cap_name = None;
            }
        }
        if default_checks::is_one_u8(cleaned.cap_period) {
            cleaned.cap_period = None;
        }
        if default_checks::is_zero_u8(cleaned.cap_radius) {
            cleaned.cap_radius = None;
        }
        if let Some(tree_tile) = &cleaned.tree_tile {
            if tree_tile.is_empty() {
                cleaned.tree_tile = None;
            }
        }
        if let Some(dead_tree_tile) = &cleaned.dead_tree_tile {
            if dead_tree_tile.is_empty() {
                cleaned.dead_tree_tile = None;
            }
        }
        if let Some(sapling_tile) = &cleaned.sapling_tile {
            if sapling_tile.is_empty() {
                cleaned.sapling_tile = None;
            }
        }
        if let Some(dead_sapling_tile) = &cleaned.dead_sapling_tile {
            if dead_sapling_tile.is_empty() {
                cleaned.dead_sapling_tile = None;
            }
        }
        if let Some(tree_color) = &cleaned.tree_color {
            if tree_color.is_default() {
                cleaned.tree_color = None;
            }
        }
        if let Some(dead_tree_color) = &cleaned.dead_tree_color {
            if dead_tree_color.is_default() {
                cleaned.dead_tree_color = None;
            }
        }
        if let Some(sapling_color) = &cleaned.sapling_color {
            if sapling_color.is_default() {
                cleaned.sapling_color = None;
            }
        }
        if let Some(dead_sapling_color) = &cleaned.dead_sapling_color {
            if dead_sapling_color.is_default() {
                cleaned.dead_sapling_color = None;
            }
        }
        if default_checks::is_default_sapling_drown_level(cleaned.sapling_drown_level) {
            cleaned.sapling_drown_level = None;
        }
        if default_checks::is_default_tree_drown_level(cleaned.tree_drown_level) {
            cleaned.tree_drown_level = None;
        }
        if let Some(tags) = &cleaned.tags {
            if tags.is_empty() {
                cleaned.tags = None;
            }
        }

        cleaned
    }
    /// Parse a new tag from the raw file into this raw object.
    ///
    /// # Arguments
    ///
    /// * `key`: The key of the tag. The first part of a tag, before the colon.
    /// * `value`: The value of the tag. The second part of a tag, after the colon.
    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    pub fn parse_tag(&mut self, key: &str, value: &str) {
        let Some(tag) = TREE_TOKENS.get(key) else {
            warn!(
    "TreeParsing: called `Option::unwrap()` on a `None` value for presumed tree tag: {}",
                key
            );
            return;
        };

        if tag == &TreeTag::Tree {
            // Skip because it's the root tag
            return;
        }

        match tag {
            TreeTag::TrunkName => {
                self.trunk_name = Some(Name::from_value(value));
            }
            TreeTag::MaxTrunkHeight => {
                let height = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("max_trunk_height parsing error\n{:?}", e);
                        return;
                    }
                };
                if height > 8 {
                    warn!("max_trunk_height parsing error: value {height} is greater than 8");
                    self.max_trunk_height = Some(8);
                }
                if height == 0 {
                    warn!("max_trunk_height parsing error: value {height} is 0");
                    self.max_trunk_height = Some(1);
                }
                self.max_trunk_height = Some(height);
            }
            TreeTag::MaxTrunkDiameter => {
                let diameter = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("max_trunk_diameter parsing error\n{:?}", e);
                        return;
                    }
                };
                if diameter > 3 {
                    warn!("max_trunk_diameter parsing error: value {diameter} is greater than 3");
                    self.max_trunk_diameter = Some(3);
                }
                if diameter == 0 {
                    warn!("max_trunk_diameter parsing error: value {diameter} is 0");
                    self.max_trunk_diameter = Some(1);
                }
                self.max_trunk_diameter = Some(diameter);
            }
            TreeTag::TrunkPeriod => {
                self.trunk_period = Some(match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("trunk_period parsing error\n{:?}", e);
                        return;
                    }
                });
            }
            TreeTag::TrunkWidthPeriod => {
                self.trunk_width_period = Some(match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("trunk_width_period parsing error\n{:?}", e);
                        return;
                    }
                });
            }
            TreeTag::BranchName => {
                self.branch_name = Some(Name::from_value(value));
            }
            TreeTag::BranchDensity => {
                self.branch_density = Some(match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("branch_density parsing error\n{:?}", e);
                        return;
                    }
                });
            }
            TreeTag::BranchRadius => {
                let radius = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("branch_radius parsing error\n{:?}", e);
                        return;
                    }
                };
                if radius > 3 {
                    warn!("branch_radius parsing error: value {radius} is greater than 3");
                    self.branch_radius = Some(3);
                }
                self.branch_radius = Some(radius);
            }
            TreeTag::HeavyBranchesName => {
                self.heavy_branches_name = Some(Name::from_value(value));
            }
            TreeTag::HeavyBranchDensity => {
                self.heavy_branch_density = Some(match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("heavy_branch_density parsing error\n{:?}", e);
                        return;
                    }
                });
            }
            TreeTag::HeavyBranchRadius => {
                let radius = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("heavy_branch_radius parsing error\n{:?}", e);
                        return;
                    }
                };
                if radius > 3 {
                    warn!("heavy_branch_radius parsing error: value {radius} is greater than 3");
                    self.heavy_branch_radius = Some(3);
                }
                self.heavy_branch_radius = Some(radius);
            }
            TreeTag::TrunkBranching => {
                self.trunk_branching = Some(match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("trunk_branching parsing error\n{:?}", e);
                        return;
                    }
                });
            }
            TreeTag::RootName => {
                self.root_name = Some(Name::from_value(value));
            }
            TreeTag::RootDensity => {
                self.root_density = Some(match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("root_density parsing error\n{:?}", e);
                        return;
                    }
                });
            }
            TreeTag::RootRadius => {
                self.root_radius = Some(match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("root_radius parsing error\n{:?}", e);
                        return;
                    }
                });
            }
            TreeTag::TwigsName => {
                self.twigs_name = Some(Name::from_value(value));
            }
            TreeTag::TwigsSideBranches => {
                if self.twigs_placement.is_none() {
                    self.twigs_placement = Some(Vec::new());
                }

                if let Some(twigs_placement) = self.twigs_placement.as_mut() {
                    twigs_placement.push(TwigPlacementTag::SideBranches);
                }
            }
            TreeTag::TwigsAboveBranches => {
                if self.twigs_placement.is_none() {
                    self.twigs_placement = Some(Vec::new());
                }

                if let Some(twigs_placement) = self.twigs_placement.as_mut() {
                    twigs_placement.push(TwigPlacementTag::AboveBranches);
                }
            }
            TreeTag::TwigsBelowBranches => {
                if self.twigs_placement.is_none() {
                    self.twigs_placement = Some(Vec::new());
                }

                if let Some(twigs_placement) = self.twigs_placement.as_mut() {
                    twigs_placement.push(TwigPlacementTag::BelowBranches);
                }
            }
            TreeTag::TwigsSideHeavyBranches => {
                if self.twigs_placement.is_none() {
                    self.twigs_placement = Some(Vec::new());
                }

                if let Some(twigs_placement) = self.twigs_placement.as_mut() {
                    twigs_placement.push(TwigPlacementTag::SideHeavyBranches);
                }
            }
            TreeTag::TwigsAboveHeavyBranches => {
                if self.twigs_placement.is_none() {
                    self.twigs_placement = Some(Vec::new());
                }

                if let Some(twigs_placement) = self.twigs_placement.as_mut() {
                    twigs_placement.push(TwigPlacementTag::AboveHeavyBranches);
                }
            }
            TreeTag::TwigsBelowHeavyBranches => {
                if self.twigs_placement.is_none() {
                    self.twigs_placement = Some(Vec::new());
                }

                if let Some(twigs_placement) = self.twigs_placement.as_mut() {
                    twigs_placement.push(TwigPlacementTag::BelowHeavyBranches);
                }
            }
            TreeTag::TwigsSideTrunk => {
                if self.twigs_placement.is_none() {
                    self.twigs_placement = Some(Vec::new());
                }

                if let Some(twigs_placement) = self.twigs_placement.as_mut() {
                    twigs_placement.push(TwigPlacementTag::SideTrunk);
                }
            }
            TreeTag::TwigsAboveTrunk => {
                if self.twigs_placement.is_none() {
                    self.twigs_placement = Some(Vec::new());
                }

                if let Some(twigs_placement) = self.twigs_placement.as_mut() {
                    twigs_placement.push(TwigPlacementTag::AboveTrunk);
                }
            }
            TreeTag::TwigsBelowTrunk => {
                if self.twigs_placement.is_none() {
                    self.twigs_placement = Some(Vec::new());
                }

                if let Some(twigs_placement) = self.twigs_placement.as_mut() {
                    twigs_placement.push(TwigPlacementTag::BelowTrunk);
                }
            }
            TreeTag::CapName => {
                self.cap_name = Some(Name::from_value(value));
            }
            TreeTag::CapPeriod => {
                self.cap_period = Some(match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("cap_period parsing error\n{:?}", e);
                        return;
                    }
                });
            }
            TreeTag::CapRadius => {
                self.cap_radius = Some(match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("cap_radius parsing error\n{:?}", e);
                        return;
                    }
                });
            }
            TreeTag::TreeTile => {
                self.tree_tile = Some(String::from(value));
            }
            TreeTag::DeadTreeTile => {
                self.dead_tree_tile = Some(String::from(value));
            }
            TreeTag::SaplingTile => {
                self.sapling_tile = Some(String::from(value));
            }
            TreeTag::DeadSaplingTile => {
                self.dead_sapling_tile = Some(String::from(value));
            }
            TreeTag::TreeColor => {
                self.tree_color = Some(Color::from_value(value));
            }
            TreeTag::DeadTreeColor => {
                self.dead_tree_color = Some(Color::from_value(value));
            }
            TreeTag::SaplingColor => {
                self.sapling_color = Some(Color::from_value(value));
            }
            TreeTag::DeadSaplingColor => {
                self.dead_sapling_color = Some(Color::from_value(value));
            }
            TreeTag::SaplingDrownLevel => {
                self.sapling_drown_level = Some(match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("sapling_drown_level parsing error\n{:?}", e);
                        return;
                    }
                });
            }
            TreeTag::TreeDrownLevel => {
                self.tree_drown_level = Some(match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("tree_drown_level parsing error\n{:?}", e);
                        return;
                    }
                });
            }
            _ => {
                if self.tags.is_none() {
                    self.tags = Some(Vec::new());
                }

                if let Some(tags) = self.tags.as_mut() {
                    tags.push(*tag);
                }
            }
        }
    }
}
