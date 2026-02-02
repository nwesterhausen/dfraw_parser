//! Plant definition and parsing

use std::collections::HashSet;

use dfraw_parser_proc_macros::{Cleanable, IsEmpty};
use tracing::{debug, warn};
use uuid::Uuid;

use crate::{
    Material, PlantGrowth, Shrub, Tree,
    custom_types::Name,
    metadata::RawMetadata,
    raw_definitions::{
        BIOME_TOKENS, MATERIAL_PROPERTY_TOKENS, MATERIAL_USAGE_TOKENS, PLANT_GROWTH_TOKENS,
        PLANT_GROWTH_TYPE_TOKENS, PLANT_TOKENS, SHRUB_TOKENS, TREE_TOKENS,
    },
    tokens::{BiomeToken, ObjectType, PlantGrowthToken, PlantGrowthTypeToken, PlantToken},
    traits::{RawObject, RawToken},
    utilities::{generate_object_id_using_raw_metadata, parse_min_max_range},
};

/// A struct representing a plant
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
pub struct Plant {
    /// Common Raw file Things
    metadata: RawMetadata,
    identifier: String,
    /// A generated id that is used to uniquely identify this object.
    ///
    /// This is deterministic based on the following:
    /// * The raw's `identifier`
    /// * The raw's [`ObjectType`]
    /// * [`RawModuleLocation`] where the raw was found
    /// * The containing module's `numeric_version`
    ///
    /// See [`crate::utilities::generate_object_id`]
    object_id: Uuid,

    // Basic Tokens
    name: Name,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pref_strings: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pub tokens: Vec<PlantToken>,

    // Environment Tokens
    /// Default [0, 0] (aboveground)
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    underground_depth: Option<[u32; 2]>,
    /// Default frequency is 50
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    frequency: Option<u32>,
    /// List of biomes this plant can grow in
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    biomes: Option<Vec<BiomeToken>>,

    /// Growth Tokens define the growths of the plant (leaves, fruit, etc.)
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    growths: Option<Vec<PlantGrowth>>,
    /// If plant is a tree, it will have details about the tree.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    tree_details: Option<Tree>,
    /// If plant is a shrub, it will have details about the shrub.
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    shrub_details: Option<Shrub>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    materials: Option<Vec<Material>>,
}

impl Plant {
    /// Create a new empty plant
    ///
    /// # Returns
    ///
    /// A new empty plant
    #[must_use]
    pub fn empty() -> Self {
        Self {
            metadata: RawMetadata::default()
                .with_object_type(ObjectType::Plant)
                .with_hidden(true),
            frequency: Some(50),
            ..Self::default()
        }
    }
    /// Create a new plant based on an identifier and metadata
    ///
    /// # Arguments
    ///
    /// * `identifier` - The identifier of the plant
    /// * `metadata` - The metadata of the plant
    ///
    /// # Returns
    ///
    /// A new plant
    #[must_use]
    pub fn new(identifier: &str, metadata: &RawMetadata) -> Self {
        Self {
            identifier: String::from(identifier),
            metadata: metadata.clone(),
            frequency: Some(50),
            object_id: generate_object_id_using_raw_metadata(
                identifier,
                ObjectType::Plant,
                metadata,
            ),
            ..Self::default()
        }
    }
    /// Get the biomes the plant can grow in
    ///
    /// # Returns
    ///
    /// A vector of biomes the plant can grow in
    #[must_use]
    pub fn get_biomes(&self) -> Vec<BiomeToken> {
        self.biomes
            .as_ref()
            .map_or_else(Vec::new, std::clone::Clone::clone)
    }

    #[must_use]
    pub fn get_tags(&self) -> Vec<PlantToken> {
        self.tokens.clone()
    }
    pub fn get_names(&self) -> Vec<&str> {
        let mut names = HashSet::new();

        names.insert(self.name.get_singular());
        names.insert(self.name.get_plural());
        names.insert(self.name.get_adjective());

        if let Some(growths) = &self.growths {
            for growth in growths {
                names.insert(growth.name.get_singular());
                names.insert(growth.name.get_plural());
                names.insert(growth.name.get_adjective());
            }
        }

        names.into_iter().collect()
    }
    pub fn get_pref_strings(&self) -> Vec<&str> {
        if let Some(prefs) = self.pref_strings.as_ref() {
            return prefs.iter().map(String::as_str).collect();
        };
        Vec::new()
    }

    /// Add a tag to the plant.
    ///
    /// This handles making sure the tags vector is initialized.
    ///
    /// # Arguments
    ///
    /// * `tag` - The tag to add to the plant
    pub fn add_tag(&mut self, tag: PlantToken) {
        self.tokens.push(tag)
    }

    /// Check whether the plant has a specific tag
    ///
    /// # Arguments
    ///
    /// * `tag` - The tag to check for
    ///
    /// # Returns
    ///
    /// Whether the plant has the tag
    #[must_use]
    pub fn has_tag(&self, tag: &PlantToken) -> bool {
        for t in &self.tokens {
            if std::mem::discriminant(t) == std::mem::discriminant(tag) {
                return true;
            }
        }
        false
    }

    /// Check whether the plant has a specific biome
    ///
    /// # Arguments
    ///
    /// * `biome` - The biome to check for
    ///
    /// # Returns
    ///
    /// Whether the plant can grow in the biome
    #[must_use]
    pub fn has_biome(&self, biome: &BiomeToken) -> bool {
        if let Some(biomes) = &self.biomes {
            for b in biomes {
                if b == biome {
                    return true;
                }
            }
        }
        false
    }
}

#[typetag::serde]
impl RawObject for Plant {
    fn get_searchable_tokens(&self) -> Vec<&str> {
        let mut tokens = HashSet::new();

        for token in PlantToken::FLAG_TOKENS {
            if self.has_tag(token) {
                tokens.insert(RawToken::get_key(token).unwrap_or_default());
            }
        }

        if let Some(growths) = &self.growths {
            for growth in growths {
                if PlantGrowthTypeToken::FLAG_TOKENS.contains(&growth.get_growth_type()) {
                    tokens.insert(RawToken::get_key(growth.get_growth_type()).unwrap_or_default());
                }
            }
        }

        tokens.into_iter().collect()
    }
    fn get_metadata(&self) -> RawMetadata {
        self.metadata.clone()
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
    fn get_name(&self) -> &str {
        self.name.get_singular()
    }

    fn get_type(&self) -> ObjectType {
        ObjectType::Plant
    }
    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn parse_tag(&mut self, key: &str, value: &str) {
        if (MATERIAL_PROPERTY_TOKENS.contains_key(key) || MATERIAL_USAGE_TOKENS.contains_key(key))
            && !key.eq("USE_MATERIAL_TEMPLATE")
        {
            // have our latest material parse the tag
            if let Some(materials) = self.materials.as_mut() {
                if let Some(material) = materials.last_mut() {
                    material.parse_tag(key, value);
                } else {
                    warn!(
                        "PlantParsing: Failed to find material to add tag {} with value {}",
                        key, value
                    );
                }
            }
            return;
        }

        if TREE_TOKENS.contains_key(key) {
            if self.tree_details.is_none() {
                self.tree_details = Some(Tree::new(value));
            }
            #[allow(clippy::unwrap_used)]
            let tree = self.tree_details.as_mut().unwrap();
            tree.parse_tag(key, value);
            return;
        }

        if PLANT_GROWTH_TOKENS.contains_key(key) {
            if self.growths.is_none() {
                self.growths = Some(Vec::new());
            }
            let token = PLANT_GROWTH_TOKENS
                .get(key)
                .unwrap_or(&PlantGrowthToken::Unknown);
            if token == &PlantGrowthToken::Growth {
                // If we are defining a new growth, we need to create a new PlantGrowth
                let growth_type = *PLANT_GROWTH_TYPE_TOKENS
                    .get(value)
                    .unwrap_or(&PlantGrowthTypeToken::None);
                let growth = PlantGrowth::new(growth_type);
                if let Some(growths) = self.growths.as_mut() {
                    growths.push(growth);
                }
                return;
            }
            // Otherwise, we are defining a tag for the current growth (most recently added)
            if let Some(growths) = self.growths.as_mut() {
                if let Some(growth) = growths.last_mut() {
                    growth.parse_tag(key, value);
                } else {
                    warn!(
                        "PlantParsing: Failed to find growth to add tag {} with value {}",
                        key, value
                    );
                }
            }
            return;
        }

        if SHRUB_TOKENS.contains_key(key) {
            if self.shrub_details.is_none() {
                self.shrub_details = Some(Shrub::new());
            }
            self.shrub_details
                .as_mut()
                .unwrap_or(&mut Shrub::default())
                .parse_tag(key, value);
            return;
        }

        if !PLANT_TOKENS.contains_key(key) {
            debug!("PlantParsing: Unknown tag {} with value {}", key, value);
            return;
        }

        let Some(tag) = PLANT_TOKENS.get(key) else {
            warn!(
                "PlantParsing: called `Option::unwrap()` on a `None` value for presumed plant tag: {}",
                key
            );
            return;
        };

        match tag {
            PlantToken::NameSingular => {
                self.name.update_singular(value);
            }
            PlantToken::NamePlural => {
                self.name.update_plural(value);
            }
            PlantToken::NameAdjective => {
                self.name.update_adjective(value);
            }
            PlantToken::AllNames => {
                self.name = Name::from_value(value);
            }
            PlantToken::PrefString => {
                if self.pref_strings.is_none() {
                    self.pref_strings = Some(Vec::new());
                }
                if let Some(pref_strings) = &mut self.pref_strings {
                    pref_strings.push(String::from(value));
                }
            }
            PlantToken::Biome => {
                let Some(biome) = BIOME_TOKENS.get(value) else {
                    warn!(
                        "PlantParsing: called `Option::unwrap()` on a `None` value for presumed biome: {}",
                        value
                    );
                    return;
                };
                if self.biomes.is_none() {
                    self.biomes = Some(Vec::new());
                }
                if let Some(biomes) = &mut self.biomes {
                    biomes.push(*biome);
                }
            }
            PlantToken::UndergroundDepth => {
                self.underground_depth = Some(parse_min_max_range(value).unwrap_or([0, 0]));
            }
            PlantToken::Frequency => {
                self.frequency = Some(value.parse::<u32>().unwrap_or(50));
            }
            PlantToken::UseMaterialTemplate => {
                if self.materials.is_none() {
                    self.materials = Some(Vec::new());
                }
                if let Some(materials) = self.materials.as_mut() {
                    materials.push(Material::use_material_template_from_value(value));
                }
            }
            PlantToken::UseMaterial => {
                if self.materials.is_none() {
                    self.materials = Some(Vec::new());
                }
                if let Some(materials) = self.materials.as_mut() {
                    materials.push(Material::use_material_from_value(value));
                }
            }
            PlantToken::BasicMaterial => {
                if self.materials.is_none() {
                    self.materials = Some(Vec::new());
                }
                if let Some(materials) = self.materials.as_mut() {
                    materials.push(Material::basic_material_from_value(value));
                }
            }
            PlantToken::Material => {
                if self.materials.is_none() {
                    self.materials = Some(Vec::new());
                }
                if let Some(materials) = self.materials.as_mut() {
                    materials.push(Material::from_value(value));
                }
            }
            _ => {
                self.add_tag(*tag);
            }
        }
    }

    fn get_object_id(&self) -> Uuid {
        self.object_id
    }
}
