//! A module for the Caste struct and its implementations.

use std::{collections::HashSet, mem::discriminant, str::FromStr as _};

use dfraw_parser_proc_macros::{Cleanable, IsEmpty};
use itertools::Itertools;
use tracing::warn;

use crate::{
    Gait,
    custom_types::{BodySize, Name, Tile},
    tokens::CasteToken,
    traits::TagOperations,
};

/// A struct representing a creature caste.
///
/// Castes are specific subgroups within a creature species, often representing
/// biological sexes, specialized roles, or unique variations specified in the raw files.
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    Default,
    specta::Type,
    Eq,
    PartialEq,
    IsEmpty,
    Cleanable,
)]
#[serde(rename_all = "camelCase")]
pub struct Caste {
    /// The unique name used in raw files for this caste (e.g., "MALE", "FEMALE").
    pub identifier: String,
    /// A collection of tags assigned to this caste.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    #[cleanable(ignore)]
    pub tokens: Vec<CasteToken>,
    /// Character and color data for map representation.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    pub tile: Tile,
    /// The gaits by which the creature can move.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    pub gaits: Vec<Gait>,
}

impl Caste {
    /// Creates a new [`Caste`] with the specified identifier.
    ///
    /// * `identifier` - The unique name used in raw files for this caste (e.g., "MALE", "FEMALE").
    ///
    /// Returns a default [`Caste`] instance with the provided identifier.
    ///
    /// # Examples
    ///
    /// ```
    /// use dfraw_parser::Caste;
    /// let caste = Caste::new("MALE");
    /// ```
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            identifier: String::from(name),
            ..Self::default()
        }
    }

    #[must_use]
    pub fn get_grazer(&self) -> Option<u32> {
        self.tokens.iter().find_map(|token| match token {
            CasteToken::Grazer { grazer } => Some(*grazer),
            _ => None,
        })
    }

    #[must_use]
    pub fn get_grass_trample(&self) -> Option<u32> {
        self.tokens.iter().find_map(|token| match token {
            CasteToken::GrassTrample { trample } => Some(*trample),
            _ => None,
        })
    }

    #[must_use]
    pub fn get_low_light_vision(&self) -> Option<u32> {
        self.tokens.iter().find_map(|token| match token {
            CasteToken::LowLightVision { vision } => Some(*vision),
            _ => None,
        })
    }

    /// Returns the age at which creatures of this caste are considered babies.
    ///
    /// This value is specified in ticks (game time units).
    #[must_use]
    pub fn get_baby_age(&self) -> Option<u32> {
        self.tokens.iter().find_map(|token| match token {
            CasteToken::Baby { age } => Some(*age),
            _ => None,
        })
    }

    /// Returns the name of the creature when it is in its baby stage.
    ///
    /// This value is specified in the raw file using the `[BABY_NAME]` token.
    #[must_use]
    pub fn get_baby_name(&self) -> Option<&Name> {
        self.tokens.iter().find_map(|token| match token {
            CasteToken::BabyName { name } => Some(name),
            _ => None,
        })
    }

    /// Returns the body size measurements for this caste at different ages.
    ///
    /// Measured in cubic centimeters. This list represents the growth stages
    /// specified by `[BODY_SIZE]` tokens in the raw files.
    #[must_use]
    pub fn get_body_sizes(&self) -> Vec<BodySize> {
        self.tokens
            .iter()
            .filter_map(|token| match token {
                CasteToken::BodySize { size } => Some(size.clone()),
                _ => None,
            })
            .unique()
            .collect()
    }

    /// Returns the specific name for this caste.
    ///
    /// This value is specified in the raw file using the `[CASTE_NAME]` token.
    #[must_use]
    pub fn get_caste_name(&self) -> Option<&Name> {
        self.tokens.iter().find_map(|token| match token {
            CasteToken::Name { name } => Some(name),
            _ => None,
        })
    }

    /// Returns the age at which creatures of this caste are considered children.
    ///
    /// This value is specified in ticks (game time units).
    #[must_use]
    pub fn get_child_age(&self) -> Option<u32> {
        self.tokens.iter().find_map(|token| match token {
            CasteToken::Child { age } => Some(*age),
            _ => None,
        })
    }

    /// Returns the name of the creature when it is in its child stage.
    ///
    /// This value is specified in the raw file using the `[CHILD_NAME]` token.
    #[must_use]
    pub fn get_child_name(&self) -> Option<&Name> {
        self.tokens.iter().find_map(|token| match token {
            CasteToken::ChildName { name } => Some(name),
            _ => None,
        })
    }

    /// Returns the clutch size range for this caste, if it lays eggs.
    ///
    /// Returns a tuple of `[min, max]` eggs per clutch.
    #[must_use]
    pub fn get_clutch_size(&self) -> Option<[u32; 2]> {
        self.tokens.iter().find_map(|token| match token {
            CasteToken::ClutchSize { min, max } => Some([*min, *max]),
            _ => None,
        })
    }

    /// Returns a slice of creature classes this caste belongs to.
    ///
    /// Creature classes are used for targeting by interactions, syndromes, and other effects.
    #[must_use]
    pub fn get_creature_classes(&self) -> Vec<String> {
        self.tokens
            .iter()
            .filter_map(|token| match token {
                CasteToken::CreatureClass { class } => Some(class.as_str()),
                _ => None,
            })
            .unique()
            .map(String::from)
            .collect()
    }

    /// Returns the difficulty rating for this caste.
    ///
    /// Higher values indicate more challenging creatures in arena mode or similar contexts.
    #[must_use]
    pub fn get_difficulty(&self) -> Option<u32> {
        self.tokens.iter().find_map(|token| match token {
            CasteToken::Difficulty { difficulty } => Some(*difficulty),
            _ => None,
        })
    }

    /// Returns the description of this caste, if available.
    ///
    /// The description is the flavor text shown in-game when examining a creature of this caste.
    ///
    /// This will find and return the first description listed in the tokens. See [`get_all_descriptions`] for
    /// complete results.
    #[must_use]
    pub fn get_description(&self) -> Option<&str> {
        self.tokens.iter().find_map(|token| match token {
            CasteToken::Description { description } => Some(description.as_str()),
            _ => None,
        })
    }

    #[must_use]
    pub fn get_all_descriptions(&self) -> Vec<&str> {
        self.tokens
            .iter()
            .filter_map(|token| match token {
                CasteToken::Description { description } => Some(description.as_str()),
                _ => None,
            })
            .unique()
            .collect()
    }

    /// Returns the size of eggs laid by this caste, if applicable.
    ///
    /// Measured in cubic centimeters (cm³).
    #[must_use]
    pub fn get_egg_size(&self) -> Option<u32> {
        self.tokens.iter().find_map(|token| match token {
            CasteToken::EggSize { size } => Some(*size),
            _ => None,
        })
    }

    /// Returns a slice of gaits (movement modes) available to this caste.
    ///
    /// Examples include walking, crawling, flying, and swimming.
    #[must_use]
    pub fn get_gaits(&self) -> &[Gait] {
        self.gaits.as_slice()
    }

    /// Returns the unique identifier of this caste.
    ///
    /// The identifier is the unique name used in raw files to distinguish this caste
    /// from others within the same creature definition.
    #[must_use]
    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }

    /// Returns the litter size range for this caste, if it gives live birth.
    ///
    /// Returns a tuple of `[min, max]` offspring per litter.
    #[must_use]
    pub fn get_litter_size(&self) -> Option<[u32; 2]> {
        self.tokens.iter().find_map(|token| match token {
            CasteToken::MaxAge { min, max } => Some([*min, *max]),
            _ => None,
        })
    }

    /// Returns the material and frequency for milking
    #[must_use]
    pub fn get_milkable(&self) -> Option<(String, u32)> {
        self.tokens.iter().find_map(|t| match t {
            CasteToken::Milkable {
                material,
                frequency,
            } => Some((material.join(":"), *frequency)),
            _ => None,
        })
    }

    /// Returns the maximum age range for this caste.
    ///
    /// Returns a tuple of `[min, max]` age in game ticks. Creatures die of old age
    /// within this range.
    #[must_use]
    pub fn get_max_age(&self) -> Option<[u32; 2]> {
        self.tokens.iter().find_map(|token| match token {
            CasteToken::MaxAge { min, max } => Some([*min, *max]),
            _ => None,
        })
    }

    /// Returns the pet value of this caste, if specified.
    ///
    /// The pet value affects how desirable this creature is as a pet and influences
    /// its trade value.
    #[must_use]
    pub fn get_pet_value(&self) -> Option<u32> {
        self.tokens.iter().find_map(|token| match token {
            CasteToken::PetValue { pet_value } => Some(*pet_value),
            _ => None,
        })
    }

    /// Returns the population ratio for this caste.
    ///
    /// This determines the relative frequency of this caste in wild populations.
    /// For example, a pop_ratio of 50 means this caste appears 50% of the time.
    #[must_use]
    pub fn get_pop_ratio(&self) -> Option<u32> {
        self.tokens.iter().find_map(|token| match token {
            CasteToken::PopulationRatio { pop_ratio } => Some(*pop_ratio),
            _ => None,
        })
    }

    #[must_use]
    pub fn get_change_body_size_percentage(&self) -> Option<u32> {
        self.tokens.iter().find_map(|token| match token {
            CasteToken::ChangeBodySizePercent { percent } => Some(*percent),
            _ => None,
        })
    }

    /// Function to get the tags of the creature caste.
    ///
    /// # Returns
    ///
    /// * `&[CasteToken]` - The tokens of the creature caste.
    #[must_use]
    pub fn get_tokens(&self) -> &[CasteToken] {
        self.tokens.as_slice()
    }

    /// Returns the tiles used to represent this caste in-game.
    ///
    /// Includes graphical or character-based representations for different display modes.
    #[must_use]
    pub fn get_tile(&self) -> Tile {
        self.tile.clone()
    }

    /// Returns true if the caste has the given token, ignoring token values.
    ///
    /// * `token` - The [`CasteToken`] to check for.
    ///
    /// This check uses the variant discriminant to match tokens regardless of internal data.
    #[must_use]
    pub fn has_token(&self, token: &CasteToken) -> bool {
        for t in &self.tokens {
            if std::mem::discriminant(t) == std::mem::discriminant(token) {
                return true;
            }
        }
        false
    }

    /// Adds a token to the internal collection if it is not already present.
    ///
    /// * `token` - The [`CasteToken`] to add.
    pub fn add_token(&mut self, token: CasteToken) {
        if !self.tokens.contains(&token) {
            self.tokens.push(token);
        }
    }

    /// Remove all instances of the given token.
    pub fn remove_token(&mut self, token: &CasteToken) {
        let target_discriminant = std::mem::discriminant(token);
        self.tokens
            .retain(|token| std::mem::discriminant(token) != target_discriminant);
    }

    /// Remove any tokens exactly matching the given one (takes into account its value)
    pub fn remove_token_with_value(&mut self, token: &CasteToken) {
        self.tokens.retain(|t| t != token);
    }

    /// Returns true if the caste is an egg layer.
    ///
    /// Checks for the presence of the `[LAYS_EGGS]` token via [`CasteToken::LaysEggs`].
    #[must_use]
    pub fn is_egg_layer(&self) -> bool {
        self.has_token(&CasteToken::LaysEggs)
    }

    /// Returns true if the caste is milkable.
    ///
    /// Checks for the presence of the `[MILKABLE]` token via [`CasteToken::Milkable`].
    #[must_use]
    pub fn is_milkable(&self) -> bool {
        self.has_token(&CasteToken::Milkable {
            material: Vec::new(),
            frequency: 0,
        })
    }

    /// Get all names for this caste (general, baby and child names)
    ///
    /// This doesn't return duplicates in the result.
    pub fn get_all_names(&self) -> Vec<String> {
        self.tokens
            .iter()
            .flat_map(|token| match token {
                CasteToken::Name { name }
                | CasteToken::BabyName { name }
                | CasteToken::ChildName { name } => name.as_vec(),
                _ => Vec::new(),
            })
            .unique()
            .collect()
    }

    /// Parses a token key and value and updates the caste state.
    ///
    /// * `key` - The key of the token to parse (e.g., "NAME").
    /// * `value` - The string value associated with the token.
    ///
    /// This method maps raw file tokens directly to internal struct fields.
    #[allow(clippy::too_many_lines)]
    pub fn parse_token(&mut self, key: &str, value: &str) {
        let Some(token) = CasteToken::parse(key, value) else {
            warn!(
                "parse_token: called `Option::unwrap()` on a `None` value for presumed caste tag: '{}'",
                key
            );
            return;
        };

        self.tokens.push(token.clone());

        match token {
            CasteToken::Tile { .. } => {
                self.tile.set_character(value);
            }
            CasteToken::AltTile { .. } => {
                self.tile.set_alt_character(value);
            }
            CasteToken::Color { .. } => {
                self.tile.set_color(value);
            }
            CasteToken::GlowTile { .. } => {
                self.tile.set_glow_character(value);
            }
            CasteToken::GlowColor { .. } => {
                self.tile.set_glow_color(value);
            }
            CasteToken::Gait { .. } => {
                self.gaits.push(Gait::from_value(value));
            }
            _ => {}
        }
    }

    /// Removes a specific token and its associated value from the caste.
    ///
    /// * `key` - The key of the token to remove.
    /// * `value` - The value of the token to remove (relevant for multi-value tokens like `GAIT`).
    ///
    /// This is used when a creature variation or selection rule negates an existing definition.
    #[allow(clippy::too_many_lines)]
    pub fn remove_token_from_key_and_value(&mut self, key: &str, value: &str) {
        let token_text = format!("{key}:{value}");
        let Ok(token) = CasteToken::from_str(token_text.as_str()) else {
            tracing::warn!("Unable to remove given key_value '{key}' '{value}'");
            return;
        };

        self.remove_token_with_value(&token);
    }

    /// Overwrites the properties of this caste with non-default values from another.
    ///
    /// * `other` - The source [`Caste`] to copy values from.
    ///
    /// Any field that is considered "default" (e.g., zero or empty) in the `other`
    /// caste will not overwrite the current value.
    #[allow(clippy::cognitive_complexity)]
    pub fn overwrite_caste(&mut self, other: &Self) {
        // Identify which token types 'other' is providing using discriminants to get only the tokens (we will be
        // removing them no matter what, so value doesn't matter)
        let replacement_types: HashSet<_> =
            other.tokens.iter().map(std::mem::discriminant).collect();

        // Remove the "old" values for those specific tokens
        self.tokens
            .retain(|token| !replacement_types.contains(&discriminant(token)));

        // Append all the new tokens from 'other'.
        self.tokens.extend(other.tokens.iter().cloned());
    }
}
