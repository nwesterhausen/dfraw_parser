//! Plant definition and parsing

use std::collections::HashSet;

use tracing::{debug, warn};

use crate::{
    default_checks,
    material::Material,
    metadata::{ObjectType, RawMetadata},
    name::Name,
    plant_growth::PlantGrowth,
    raw_definitions::{
        BIOME_TOKENS, MATERIAL_PROPERTY_TOKENS, MATERIAL_USAGE_TOKENS, PLANT_GROWTH_TOKENS,
        PLANT_GROWTH_TYPE_TOKENS, PLANT_TOKENS, SHRUB_TOKENS, TREE_TOKENS,
    },
    shrub::Shrub,
    tags::{BiomeTag, PlantGrowthTag, PlantGrowthTypeTag, PlantTag},
    traits::{RawObject, Searchable, searchable::clean_search_vec},
    tree::Tree,
    utilities::{build_object_id_from_pieces, parse_min_max_range},
};

/// A struct representing a plant
#[allow(clippy::module_name_repetitions)]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Plant {
    /// Common Raw file Things
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RawMetadata>,
    identifier: String,
    object_id: String,

    // Basic Tokens
    name: Name,
    #[serde(skip_serializing_if = "Option::is_none")]
    pref_strings: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<PlantTag>>,

    // Environment Tokens
    /// Default [0, 0] (aboveground)
    #[serde(skip_serializing_if = "Option::is_none")]
    underground_depth: Option<[u32; 2]>,
    /// Default frequency is 50
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency: Option<u32>,
    /// List of biomes this plant can grow in
    #[serde(skip_serializing_if = "Option::is_none")]
    biomes: Option<Vec<BiomeTag>>,

    /// Growth Tokens define the growths of the plant (leaves, fruit, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    growths: Option<Vec<PlantGrowth>>,
    /// If plant is a tree, it will have details about the tree.
    #[serde(skip_serializing_if = "Option::is_none")]
    tree_details: Option<Tree>,
    /// If plant is a shrub, it will have details about the shrub.
    #[serde(skip_serializing_if = "Option::is_none")]
    shrub_details: Option<Shrub>,

    #[serde(skip_serializing_if = "Option::is_none")]
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
            metadata: Some(
                RawMetadata::default()
                    .with_object_type(ObjectType::Plant)
                    .with_hidden(true),
            ),
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
            metadata: Some(metadata.clone()),
            frequency: Some(50),
            object_id: build_object_id_from_pieces(metadata, identifier, &ObjectType::Plant),
            ..Self::default()
        }
    }
    /// Get the biomes the plant can grow in
    ///
    /// # Returns
    ///
    /// A vector of biomes the plant can grow in
    #[must_use]
    pub fn get_biomes(&self) -> Vec<BiomeTag> {
        self.biomes
            .as_ref()
            .map_or_else(Vec::new, std::clone::Clone::clone)
    }

    pub fn get_all_names(&self) -> Vec<&str> {
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
    /// A new plant with all empty or default values removed.
    #[must_use]
    pub fn cleaned(&self) -> Self {
        let mut cleaned = self.clone();

        if let Some(metadata) = &cleaned.metadata
            && metadata.is_hidden()
        {
            cleaned.metadata = None;
        }

        if let Some(pref_strings) = &cleaned.pref_strings
            && pref_strings.is_empty()
        {
            cleaned.pref_strings = None;
        }

        if let Some(tags) = &cleaned.tags
            && tags.is_empty()
        {
            cleaned.tags = None;
        }

        if default_checks::min_max_is_zeroes(&cleaned.underground_depth) {
            cleaned.underground_depth = None;
        }

        if default_checks::is_default_frequency(cleaned.frequency) {
            cleaned.frequency = None;
        }

        if let Some(biomes) = &cleaned.biomes
            && biomes.is_empty()
        {
            cleaned.biomes = None;
        }

        if let Some(growths) = &cleaned.growths {
            let mut cleaned_growths = Vec::new();
            for growth in growths {
                cleaned_growths.push(growth.cleaned());
            }
            cleaned.growths = Some(cleaned_growths);
        }

        if let Some(materials) = &cleaned.materials {
            let mut cleaned_materials = Vec::new();
            for material in materials {
                cleaned_materials.push(material.cleaned());
            }
            if cleaned_materials.is_empty() {
                cleaned.materials = None;
            }
            cleaned.materials = Some(cleaned_materials);
        }

        cleaned
    }
    /// Add a tag to the plant.
    ///
    /// This handles making sure the tags vector is initialized.
    ///
    /// # Arguments
    ///
    /// * `tag` - The tag to add to the plant
    pub fn add_tag(&mut self, tag: PlantTag) {
        if self.tags.is_none() {
            self.tags = Some(Vec::new());
        }
        if let Some(tags) = self.tags.as_mut() {
            tags.push(tag);
        } else {
            warn!(
                "Plant::add_tag: ({}) Failed to add tag {:?}",
                self.identifier, tag
            );
        }
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
    pub fn has_tag(&self, tag: &PlantTag) -> bool {
        if let Some(tags) = &self.tags {
            for t in tags {
                if std::mem::discriminant(t) == std::mem::discriminant(tag) {
                    return true;
                }
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
    pub fn has_biome(&self, biome: &BiomeTag) -> bool {
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

        for token in PlantTag::FLAG_TOKENS {
            if self.has_tag(token) {
                tokens.insert(PlantTag::get_key(token).unwrap_or_default());
            }
        }

        if let Some(growths) = &self.growths {
            for growth in growths {
                if PlantGrowthTypeTag::FLAG_TOKENS.contains(&growth.get_growth_type()) {
                    tokens.insert(
                        PlantGrowthTypeTag::get_key(growth.get_growth_type()).unwrap_or_default(),
                    );
                }
            }
        }

        tokens.into_iter().collect()
    }
    fn get_metadata(&self) -> RawMetadata {
        self.metadata.as_ref().map_or_else(
            || {
                warn!(
                    "PlantParsing: Failed to get metadata for plant {}",
                    self.identifier
                );
                RawMetadata::default()
                    .with_object_type(ObjectType::Plant)
                    .with_hidden(true)
            },
            std::clone::Clone::clone,
        )
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
    fn get_name(&self) -> &str {
        self.name.get_singular()
    }
    fn is_empty(&self) -> bool {
        self.identifier.is_empty()
    }

    fn clean_self(&mut self) {
        *self = self.cleaned();
    }
    fn get_type(&self) -> &ObjectType {
        &ObjectType::Plant
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
                .unwrap_or(&PlantGrowthTag::Unknown);
            if token == &PlantGrowthTag::Growth {
                // If we are defining a new growth, we need to create a new PlantGrowth
                let growth_type = *PLANT_GROWTH_TYPE_TOKENS
                    .get(value)
                    .unwrap_or(&PlantGrowthTypeTag::None);
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
            PlantTag::NameSingular => {
                self.name.update_singular(value);
            }
            PlantTag::NamePlural => {
                self.name.update_plural(value);
            }
            PlantTag::NameAdjective => {
                self.name.update_adjective(value);
            }
            PlantTag::AllNames => {
                self.name = Name::from_value(value);
            }
            PlantTag::PrefString => {
                if self.pref_strings.is_none() {
                    self.pref_strings = Some(Vec::new());
                }
                if let Some(pref_strings) = &mut self.pref_strings {
                    pref_strings.push(String::from(value));
                }
            }
            PlantTag::Biome => {
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
            PlantTag::UndergroundDepth => {
                self.underground_depth = Some(parse_min_max_range(value).unwrap_or([0, 0]));
            }
            PlantTag::Frequency => {
                self.frequency = Some(value.parse::<u32>().unwrap_or(50));
            }
            PlantTag::UseMaterialTemplate => {
                if self.materials.is_none() {
                    self.materials = Some(Vec::new());
                }
                if let Some(materials) = self.materials.as_mut() {
                    materials.push(Material::use_material_template_from_value(value));
                }
            }
            PlantTag::UseMaterial => {
                if self.materials.is_none() {
                    self.materials = Some(Vec::new());
                }
                if let Some(materials) = self.materials.as_mut() {
                    materials.push(Material::use_material_from_value(value));
                }
            }
            PlantTag::BasicMaterial => {
                if self.materials.is_none() {
                    self.materials = Some(Vec::new());
                }
                if let Some(materials) = self.materials.as_mut() {
                    materials.push(Material::basic_material_from_value(value));
                }
            }
            PlantTag::Material => {
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

    fn get_object_id(&self) -> &str {
        &self.object_id
    }
}

impl Searchable for Plant {
    fn get_search_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();

        vec.push(self.get_identifier().to_string());
        vec.extend(self.name.as_vec());
        if let Some(pref_strings) = &self.pref_strings {
            vec.extend(pref_strings.clone());
        }
        if let Some(biomes) = &self.biomes {
            vec.extend(biomes.iter().map(std::string::ToString::to_string));
        }
        if let Some(tags) = &self.tags {
            vec.extend(tags.iter().map(std::string::ToString::to_string));
        }
        if let Some(growths) = &self.growths {
            vec.extend(growths.iter().flat_map(Searchable::get_search_vec));
        }
        if let Some(materials) = &self.materials {
            vec.extend(materials.iter().flat_map(Searchable::get_search_vec));
        }

        clean_search_vec(vec.as_slice())
    }
}
