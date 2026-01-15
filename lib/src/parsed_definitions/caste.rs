//! A module for the Caste struct and its implementations.

use tracing::warn;

use crate::{
    body_size::BodySize,
    default_checks,
    gait::Gait,
    milkable::Milkable,
    name::Name,
    raw_definitions::CASTE_TOKENS,
    tags::CasteTag,
    tile::Tile,
    traits::{TagOperations, searchable::Searchable},
};

/// A struct representing a creature caste.
#[allow(clippy::module_name_repetitions)]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Caste {
    identifier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<CasteTag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    // String Tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    baby_name: Option<Name>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caste_name: Option<Name>,
    #[serde(skip_serializing_if = "Option::is_none")]
    child_name: Option<Name>,
    // [min, max] ranges
    /// Default \[0,0\]
    #[serde(skip_serializing_if = "Option::is_none")]
    clutch_size: Option<[u32; 2]>,
    /// Default \[0,0\]
    #[serde(skip_serializing_if = "Option::is_none")]
    litter_size: Option<[u32; 2]>,
    /// Default \[0,0\]
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age: Option<[u32; 2]>,
    // Integer tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    baby: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    child: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    difficulty: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egg_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grass_trample: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grazer: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    low_light_vision: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pet_value: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pop_ratio: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    change_body_size_percentage: Option<u32>,
    // Arrays
    #[serde(skip_serializing_if = "Option::is_none")]
    creature_class: Option<Vec<String>>,
    // Special Tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    body_size: Option<Vec<BodySize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    milkable: Option<Milkable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tile: Option<Tile>,
    /// The gaits by which the creature can move.
    #[serde(skip_serializing_if = "Option::is_none")]
    gaits: Option<Vec<Gait>>,
}

impl Caste {
    /// Function to create a new Caste.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the caste.
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            identifier: String::from(name),
            ..Self::default()
        }
    }
    /// Function to get the tags of the creature caste.
    ///
    /// # Returns
    ///
    /// * `&[CasteTag]` - The tags of the creature caste.
    #[must_use]
    pub fn get_tags(&self) -> &[CasteTag] {
        self.tags.as_ref().map_or(&[], |tags| tags.as_slice())
    }
    /// Function to get the milkable of the creature caste.
    ///
    /// # Returns
    ///
    /// * `Milkable` - The milkable of the creature caste.
    #[must_use]
    pub fn get_milkable(&self) -> Milkable {
        self.milkable
            .as_ref()
            .map_or_else(Milkable::default, std::clone::Clone::clone)
    }
    /// Parse a tag and value into the creature caste
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the tag to parse.
    /// * `value` - The value of the tag to parse.
    #[allow(clippy::too_many_lines)]
    pub fn parse_tag(&mut self, key: &str, value: &str) {
        let Some(tag) = CasteTag::parse(key, value) else {
            warn!(
                "parse_tag: called `Option::unwrap()` on a `None` value for presumed caste tag: '{}'",
                key
            );
            return;
        };

        if let Some(tags) = self.tags.as_mut() {
            tags.push(tag.clone());
        } else {
            self.tags = Some(vec![tag.clone()]);
        }

        match tag {
            CasteTag::Description { description } => self.description = Some(description),
            CasteTag::EggSize { size } => self.egg_size = Some(size),
            CasteTag::Baby { age } => self.baby = Some(age),
            CasteTag::Child { age } => self.child = Some(age),
            CasteTag::Difficulty { difficulty } => self.difficulty = Some(difficulty),
            CasteTag::Grazer { grazer } => self.grazer = Some(grazer),
            CasteTag::GrassTrample { trample } => self.grass_trample = Some(trample),
            CasteTag::LowLightVision { vision } => self.low_light_vision = Some(vision),
            CasteTag::PopulationRatio { pop_ratio } => self.pop_ratio = Some(pop_ratio),
            CasteTag::PetValue { pet_value } => self.pet_value = Some(pet_value),
            CasteTag::ClutchSize { min, max } => self.clutch_size = Some([min, max]),
            CasteTag::LitterSize { min, max } => self.litter_size = Some([min, max]),
            CasteTag::MaxAge { min, max } => self.max_age = Some([min, max]),
            CasteTag::CreatureClass { class } => {
                if let Some(creature_classes) = self.creature_class.as_mut() {
                    creature_classes.push(class);
                } else {
                    self.creature_class = Some(vec![class]);
                }
            }
            CasteTag::BodySize { .. } => {
                if let Some(body_sizes) = self.body_size.as_mut() {
                    body_sizes.push(BodySize::from_value(value));
                } else {
                    self.body_size = Some(vec![BodySize::from_value(value)]);
                }
            }
            CasteTag::Milkable { .. } => self.milkable = Some(Milkable::from_value(value)),
            CasteTag::BabyName { .. } => self.baby_name = Some(Name::from_value(value)),
            CasteTag::Name { .. } => self.caste_name = Some(Name::from_value(value)),
            CasteTag::ChildName { .. } => self.child_name = Some(Name::from_value(value)),
            CasteTag::Tile { .. } => {
                if let Some(tile) = self.tile.as_mut() {
                    tile.set_character(value);
                } else {
                    self.tile = Some(Tile::default().with_character(value));
                }
            }
            CasteTag::AltTile { .. } => {
                if let Some(tile) = self.tile.as_mut() {
                    tile.set_alt_character(value);
                } else {
                    self.tile = Some(Tile::default().with_alt_character(value));
                }
            }
            CasteTag::Color { .. } => {
                if let Some(tile) = self.tile.as_mut() {
                    tile.set_color(value);
                } else {
                    self.tile = Some(Tile::default().with_color(value));
                }
            }
            CasteTag::GlowTile { .. } => {
                if let Some(tile) = self.tile.as_mut() {
                    tile.set_glow_character(value);
                } else {
                    self.tile = Some(Tile::default().with_glow_character(value));
                }
            }
            CasteTag::GlowColor { .. } => {
                if let Some(tile) = self.tile.as_mut() {
                    tile.set_glow_color(value);
                } else {
                    self.tile = Some(Tile::default().with_glow_color(value));
                }
            }
            CasteTag::ChangeBodySizePercent { .. } => {
                self.change_body_size_percentage = Some(value.parse::<u32>().unwrap_or_default());
            }
            CasteTag::Gait { .. } => {
                if let Some(gaits) = self.gaits.as_mut() {
                    gaits.push(Gait::from_value(value));
                } else {
                    self.gaits = Some(vec![Gait::from_value(value)]);
                }
            }
            _ => {}
        }
    }
    /// Returns the identifier of this caste.
    ///
    /// The identifier is the unique name used in the raw files to reference this caste.
    #[must_use]
    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }

    /// Returns the description of this caste, if available.
    ///
    /// This is the descriptive text shown in-game for creatures of this caste.
    #[must_use]
    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the pet value of this caste, if specified.
    ///
    /// The pet value affects how desirable this creature is as a pet and influences
    /// its trade value.
    #[must_use]
    pub fn get_pet_value(&self) -> Option<u32> {
        self.pet_value
    }

    /// Returns the age at which creatures of this caste are considered babies.
    ///
    /// This value is specified in ticks (game time units).
    #[must_use]
    pub fn get_baby_age(&self) -> Option<u32> {
        self.baby
    }

    /// Returns the age at which creatures of this caste are considered children.
    ///
    /// This value is specified in ticks (game time units).
    #[must_use]
    pub fn get_child_age(&self) -> Option<u32> {
        self.child
    }

    /// Returns the difficulty rating for this caste.
    ///
    /// Higher values indicate more challenging creatures in arena mode or similar contexts.
    #[must_use]
    pub fn get_difficulty(&self) -> Option<u32> {
        self.difficulty
    }

    /// Returns the size of eggs laid by this caste, if applicable.
    ///
    /// Measured in cubic centimeters (cm³).
    #[must_use]
    pub fn get_egg_size(&self) -> Option<u32> {
        self.egg_size
    }

    /// Returns the population ratio for this caste.
    ///
    /// This determines the relative frequency of this caste in wild populations.
    /// For example, a pop_ratio of 50 means this caste appears 50% of the time.
    #[must_use]
    pub fn get_pop_ratio(&self) -> Option<u32> {
        self.pop_ratio
    }

    /// Returns the clutch size range for this caste, if it lays eggs.
    ///
    /// Returns a tuple of `[min, max]` eggs per clutch.
    #[must_use]
    pub fn get_clutch_size(&self) -> Option<[u32; 2]> {
        self.clutch_size
    }

    /// Returns the litter size range for this caste, if it gives live birth.
    ///
    /// Returns a tuple of `[min, max]` offspring per litter.
    #[must_use]
    pub fn get_litter_size(&self) -> Option<[u32; 2]> {
        self.litter_size
    }

    /// Returns the maximum age range for this caste.
    ///
    /// Returns a tuple of `[min, max]` age in game ticks. Creatures die of old age
    /// within this range.
    #[must_use]
    pub fn get_max_age(&self) -> Option<[u32; 2]> {
        self.max_age
    }

    /// Returns the name used for baby creatures of this caste.
    #[must_use]
    pub fn get_baby_name(&self) -> Option<&Name> {
        self.baby_name.as_ref()
    }

    /// Returns the name used for child creatures of this caste.
    #[must_use]
    pub fn get_child_name(&self) -> Option<&Name> {
        self.child_name.as_ref()
    }

    /// Returns the name used for adult creatures of this caste.
    #[must_use]
    pub fn get_caste_name(&self) -> Option<&Name> {
        self.caste_name.as_ref()
    }

    /// Returns the tile graphics information for this caste.
    ///
    /// This defines the character and colors used to display this caste in ASCII mode.
    #[must_use]
    pub fn get_tile(&self) -> Option<&Tile> {
        self.tile.as_ref()
    }

    /// Returns a slice of body sizes for this caste at different life stages.
    ///
    /// Body size affects combat, carrying capacity, and butchering yields.
    #[must_use]
    pub fn get_body_sizes(&self) -> &[BodySize] {
        self.body_size.as_deref().unwrap_or(&[])
    }

    /// Returns a slice of creature classes this caste belongs to.
    ///
    /// Creature classes are used for targeting by interactions, syndromes, and other effects.
    #[must_use]
    pub fn get_creature_classes(&self) -> &[String] {
        self.creature_class.as_deref().unwrap_or(&[])
    }

    /// Returns a slice of gaits (movement modes) available to this caste.
    ///
    /// Examples include walking, crawling, flying, and swimming.
    #[must_use]
    pub fn get_gaits(&self) -> &[Gait] {
        self.gaits.as_deref().unwrap_or(&[])
    }
    /// Function to remove a tag from the creature.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the tag to remove.
    /// * `value` - The value of the tag to remove.
    #[allow(clippy::too_many_lines)]
    pub fn remove_tag_and_value(&mut self, key: &str, value: &str) {
        let Some(tag) = CASTE_TOKENS.get(key) else {
            warn!(
                "remove_tag_and_value: called `Option::unwrap()` on a `None` value for presumed caste tag: {key}"
            );
            return;
        };

        // Complex tags won't parse if we are removing them, (only the KEY is set)
        match key {
                "DESCRIPTION" => self.description = None,
                "EGG_SIZE" => self.egg_size = None,
                "BABY" => self.baby = None,
                "CHILD" => self.child = None,
                "DIFFICULTY" => self.difficulty = None,
                "GRAZER" => self.grazer = None,
                "GRASS_TRAMPLE" => self.grass_trample = None,
                "LOW_LIGHT_VISION" => self.low_light_vision = None,
                "POP_RATIO" => self.pop_ratio = None,
                "PET_VALUE" => self.pet_value = None,
                "CLUTCH_SIZE" => self.clutch_size = None,
                "LITTER_SIZE" => self.litter_size = None,
                "MAX_AGE" => self.max_age = None,
                "CREATURE_CLASS" => {
                    if let Some(creature_classes) = self.creature_class.as_mut() {
                        creature_classes.retain(|class| class != value);
                    }
                }
                "BODY_SIZE" => {
                    if let Some(body_sizes) = self.body_size.as_mut() {
                        body_sizes.retain(|body_size| body_size != &BodySize::from_value(value));
                    }
                }
                "MILKABLE" => self.milkable = None,
                "BABY_NAME" => self.baby_name = None,
                "NAME" => self.caste_name = None,
                "CHILD_NAME" => self.child_name = None,
                "TILE" | //=> self.tile = Tile::default(),
                "ALTTILE" | //=> self.tile = Tile::default(),
                "COLOR" | //=> self.tile = Tile::default(),
                "GLOWTILE" | //=> self.tile = Tile::default(),
                "GLOWCOLOR" => self.tile = None,
                "CHANGE_BODY_SIZE_PERCENT" => {
                    self.change_body_size_percentage = None;
                }
                "GAIT" => {
                    // Remove the specific gait from the gaits vector
                    if let Some(gaits) = self.gaits.as_mut() {
                        gaits.retain(|gait| gait != &Gait::from_value(value));
                    }
                }
                _ => {
                }
            }

        if let Some(tags) = self.tags.as_mut() {
            tags.retain(|t| t != tag);
        }
    }
    /// Overwrites the values of self with the values of other.
    ///
    /// # Arguments
    ///
    /// * `other` - The other caste to overwrite self with.
    ///
    /// # Notes
    ///
    /// This function will overwrite any values in self with the values from other. If a value is default in other, it will not be overwritten.
    #[allow(clippy::cognitive_complexity)]
    pub fn overwrite_caste(&mut self, other: &Self) {
        // Include any tags from other that aren't in self
        if let Some(tags) = &other.tags {
            for tag in tags {
                if !self.has_tag(tag) {
                    self.add_tag(tag.clone());
                }
            }
        }
        // For any of the other's values that are not default, overwrite self's values
        if let Some(other_description) = &other.description
            && !other_description.is_empty()
        {
            self.description = Some(other_description.clone());
        }
        if let Some(other_baby_name) = &other.baby_name
            && !other_baby_name.is_empty()
        {
            self.baby_name = Some(other_baby_name.clone());
        }
        if let Some(other_caste_name) = &other.caste_name
            && !other_caste_name.is_empty()
        {
            self.caste_name = Some(other_caste_name.clone());
        }
        if let Some(other_child_name) = &other.child_name
            && !other_child_name.is_empty()
        {
            self.child_name = Some(other_child_name.clone());
        }
        if !default_checks::min_max_is_zeroes(&other.clutch_size) {
            self.clutch_size = other.clutch_size;
        }
        if !default_checks::min_max_is_zeroes(&other.litter_size) {
            self.litter_size = other.litter_size;
        }
        if !default_checks::min_max_is_zeroes(&other.max_age) {
            self.max_age = other.max_age;
        }
        if !default_checks::is_zero(other.baby) {
            self.baby = other.baby;
        }
        if !default_checks::is_zero(other.child) {
            self.child = other.child;
        }
        if !default_checks::is_zero(other.difficulty) {
            self.difficulty = other.difficulty;
        }
        if !default_checks::is_zero(other.egg_size) {
            self.egg_size = other.egg_size;
        }
        if !default_checks::is_zero(other.grass_trample) {
            self.grass_trample = other.grass_trample;
        }
        if !default_checks::is_zero(other.grazer) {
            self.grazer = other.grazer;
        }
        if !default_checks::is_zero(other.low_light_vision) {
            self.low_light_vision = other.low_light_vision;
        }
        if !default_checks::is_zero(other.pet_value) {
            self.pet_value = other.pet_value;
        }
        if !default_checks::is_zero(other.pop_ratio) {
            self.pop_ratio = other.pop_ratio;
        }
        if !default_checks::is_zero(other.change_body_size_percentage) {
            self.change_body_size_percentage = other.change_body_size_percentage;
        }
        if let Some(other_creature_class) = &other.creature_class
            && !other_creature_class.is_empty()
        {
            self.creature_class = Some(other_creature_class.clone());
        }
        if let Some(other_body_size) = &other.body_size
            && !other_body_size.is_empty()
        {
            self.body_size = Some(other_body_size.clone());
        }
        if let Some(other_milkable) = &other.milkable
            && !other_milkable.is_default()
        {
            self.milkable = Some(other_milkable.clone());
        }
        if let Some(other_tile) = &other.tile
            && !other_tile.is_default()
        {
            self.tile = Some(other_tile.clone());
        }
    }
    /// Returns true if the caste is an egg layer.
    ///
    /// # Returns
    ///
    /// True if the caste is an egg layer, false otherwise.
    #[must_use]
    pub fn is_egg_layer(&self) -> bool {
        self.has_tag(&CasteTag::LaysEggs)
    }
    /// Returns true if the caste is milkable.
    ///
    /// # Returns
    ///
    /// True if the caste is milkable, false otherwise.
    #[must_use]
    pub fn is_milkable(&self) -> bool {
        self.has_tag(&CasteTag::Milkable {
            material: Vec::new(),
            frequency: 0,
        })
    }
    /// Returns true if the caste has the given tag, no values are checked.
    ///
    /// ## Arguments
    ///
    /// * `tag` - The tag to check for (note that any values are ignored)
    ///
    /// ## Returns
    ///
    /// True if the caste has the given tag, false otherwise.
    #[must_use]
    pub fn has_tag(&self, tag: &CasteTag) -> bool {
        if let Some(tags) = &self.tags {
            for t in tags {
                if std::mem::discriminant(t) == std::mem::discriminant(tag) {
                    return true;
                }
            }
        }
        false
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
    #[must_use]
    #[allow(clippy::cognitive_complexity)]
    pub fn cleaned(&self) -> Self {
        let mut cleaned = self.clone();

        // Set any empty string to None.
        if cleaned.description.is_some() && cleaned.description.as_deref() == Some("") {
            cleaned.description = None;
        }

        // Set any empty list to None.
        if cleaned.creature_class.is_some() && cleaned.creature_class.as_deref() == Some(&[]) {
            cleaned.creature_class = None;
        }

        // Set any empty list to None.
        if cleaned.body_size.is_some() && cleaned.body_size.as_deref() == Some(&[]) {
            cleaned.body_size = None;
        }

        // Set any default values to None.
        if default_checks::is_zero(cleaned.baby) {
            cleaned.baby = None;
        }

        // Set any default values to None.
        if default_checks::is_zero(cleaned.child) {
            cleaned.child = None;
        }

        // Set any default values to None.
        if default_checks::is_zero(cleaned.difficulty) {
            cleaned.difficulty = None;
        }

        // Set any default values to None.
        if default_checks::is_zero(cleaned.egg_size) {
            cleaned.egg_size = None;
        }

        // Set any default values to None.
        if default_checks::is_zero(cleaned.grass_trample) {
            cleaned.grass_trample = None;
        }

        // Set any default values to None.
        if default_checks::is_zero(cleaned.grazer) {
            cleaned.grazer = None;
        }

        // Set any default values to None.
        if default_checks::is_zero(cleaned.low_light_vision) {
            cleaned.low_light_vision = None;
        }

        // Set any default values to None.
        if default_checks::is_zero(cleaned.pet_value) {
            cleaned.pet_value = None;
        }

        // Set any default values to None.
        if default_checks::is_zero(cleaned.pop_ratio) {
            cleaned.pop_ratio = None;
        }

        // Set any default values to None.
        if default_checks::is_zero(cleaned.change_body_size_percentage) {
            cleaned.change_body_size_percentage = None;
        }

        // Set any default values to None.
        if default_checks::min_max_is_zeroes(&cleaned.clutch_size) {
            cleaned.clutch_size = None;
        }

        // Set any default values to None.
        if default_checks::min_max_is_zeroes(&cleaned.litter_size) {
            cleaned.litter_size = None;
        }

        // Set any default values to None.
        if default_checks::min_max_is_zeroes(&cleaned.max_age) {
            cleaned.max_age = None;
        }

        // Set any default values to None.
        if let Some(baby_name) = cleaned.baby_name.clone()
            && baby_name.is_empty()
        {
            cleaned.baby_name = None;
        }

        // Set any default values to None.
        if let Some(caste_name) = cleaned.caste_name.clone()
            && caste_name.is_empty()
        {
            cleaned.caste_name = None;
        }

        // Set any default values to None.
        if let Some(child_name) = cleaned.child_name.clone()
            && child_name.is_empty()
        {
            cleaned.child_name = None;
        }

        // Set any default values to None.
        if let Some(milkable) = cleaned.milkable.clone()
            && milkable.is_default()
        {
            cleaned.milkable = None;
        }

        // Set any default values to None.
        if let Some(tile) = cleaned.tile.clone()
            && tile.is_default()
        {
            cleaned.tile = None;
        }

        cleaned
    }

    fn add_tag(&mut self, tag: CasteTag) {
        if let Some(tags) = self.tags.as_mut() {
            if !tags.contains(&tag) {
                tags.push(tag);
            }
        } else {
            self.tags = Some(vec![tag]);
        }
    }
}

impl Searchable for Caste {
    // Used to help extend things that own this caste
    fn get_search_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();

        // Identifier
        vec.push(self.identifier.clone());
        // Name (and child/baby names)
        if let Some(caste_name) = &self.caste_name {
            vec.extend(caste_name.as_vec());
        }
        if let Some(child_name) = &self.child_name {
            vec.extend(child_name.as_vec());
        }
        if let Some(baby_name) = &self.baby_name {
            vec.extend(baby_name.as_vec());
        }
        // Creature Class
        if let Some(creature_class) = &self.creature_class {
            vec.extend(creature_class.clone());
        }
        // Description
        if let Some(description) = &self.description {
            vec.push(description.clone());
        }
        // If egg layer, include egg information
        if self.is_egg_layer() {
            vec.push(String::from("eggs"));
            if let Some([clutch_size_0, clutch_size_1, ..]) = self.clutch_size {
                vec.push(format!("{clutch_size_0}-{clutch_size_1}"));
            }
            if let Some(egg_size) = self.egg_size {
                vec.push(format!("{egg_size}"));
            }
        }
        // If milkable, include milk information
        if self.is_milkable() {
            vec.push(String::from("milk"));
            if let Some(milkable) = &self.milkable {
                vec.extend(milkable.as_vec());
            }
        }
        if let Some(tags) = &self.tags {
            // If flier, include flyer information
            if tags.contains(&CasteTag::Flier) {
                vec.push(String::from("flying flies flier"));
            }
            // If playable/civilized, include playable information
            if tags.contains(&CasteTag::OutsiderControllable) {
                vec.push(String::from("playable civilized"));
            }
            // If speaks, include language information
            // If learns, include learn
            // If both, include "intelligent"
            if tags.contains(&CasteTag::Intelligent) || tags.contains(&CasteTag::CanSpeak) {
                vec.push(String::from("speaks language"));
            }
            if tags.contains(&CasteTag::Intelligent) || tags.contains(&CasteTag::CanLearn) {
                vec.push(String::from("learns"));
            }
            if tags.contains(&CasteTag::Intelligent)
                || (tags.contains(&CasteTag::CanSpeak) && tags.contains(&CasteTag::CanLearn))
            {
                vec.push(String::from("intelligent"));
            }
        }
        // Include difficulty if not 0
        if let Some(difficulty) = self.difficulty {
            vec.push(format!("{difficulty}"));
        }
        // Include pet value if not 0
        if let Some(pet_value) = self.pet_value {
            vec.push(format!("{pet_value}"));
        }

        vec
    }
}
