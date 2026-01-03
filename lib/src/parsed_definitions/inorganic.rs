//! Parsed Inorganic object definition.

use crate::{
    default_checks,
    material::Material,
    metadata::{ObjectType, RawMetadata},
    raw_definitions::{ENVIRONMENT_CLASS_TOKENS, INCLUSION_TYPE_TOKENS, INORGANIC_TOKENS},
    tags::{EnvironmentClassTag, InclusionTypeTag, InorganicTag},
    traits::{searchable::clean_search_vec, RawObject, Searchable},
    utilities::build_object_id_from_pieces,
};

/// The raw representation of an inorganic object.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Inorganic {
    identifier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RawMetadata>,
    object_id: String,
    material: Material,

    #[serde(skip_serializing_if = "Option::is_none")]
    metal_ore_chance: Option<Vec<(String, u8)>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thread_metal_chance: Option<Vec<(String, u8)>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    environment_class: Option<EnvironmentClassTag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_inclusion_type: Option<InclusionTypeTag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_inclusion_frequency: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_class_specific: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<InorganicTag>>,
}

impl Inorganic {
    /// Create a new empty Inorganic object.
    ///
    /// This is used for creating a new Inorganic object with the metadata set to hidden.
    ///
    /// # Returns
    ///
    /// A new Inorganic object with the metadata set to hidden.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            metadata: Some(
                RawMetadata::default()
                    .with_object_type(ObjectType::Inorganic)
                    .with_hidden(true),
            ),
            ..Self::default()
        }
    }
    /// Create a new Inorganic object with the given identifier and metadata.
    ///
    /// The `object_id` is generated from the metadata's raw identifier and the identifier.
    ///
    /// # Arguments
    ///
    /// * `identifier` - The identifier for the inorganic object.
    /// * `metadata` - The metadata for the inorganic object.
    ///
    /// # Returns
    ///
    /// A new Inorganic object with the given identifier and metadata.
    #[must_use]
    pub fn new(identifier: &str, metadata: &RawMetadata) -> Self {
        Self {
            identifier: String::from(identifier),
            metadata: Some(metadata.clone()),
            object_id: build_object_id_from_pieces(metadata, identifier, &ObjectType::Inorganic),
            ..Self::default()
        }
    }
    /// Function to "clean" the creature. This is used to remove any empty list or strings,
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
    /// A new Inorganic object with all empty or default values set to None.
    #[must_use]
    pub fn cleaned(&self) -> Self {
        let mut cleaned = self.clone();

        if let Some(metadata) = &cleaned.metadata {
            if metadata.is_hidden() {
                cleaned.metadata = None;
            }
        }

        if let Some(metal_ore_chance) = &cleaned.metal_ore_chance {
            if metal_ore_chance.is_empty() {
                cleaned.metal_ore_chance = None;
            }
        }
        if let Some(thread_metal_chance) = &cleaned.thread_metal_chance {
            if thread_metal_chance.is_empty() {
                cleaned.thread_metal_chance = None;
            }
        }
        if let Some(environment_class) = &cleaned.environment_class {
            if environment_class.is_default() {
                cleaned.environment_class = None;
            }
        }
        if let Some(environment_inclusion_type) = &cleaned.environment_inclusion_type {
            if environment_inclusion_type.is_default() {
                cleaned.environment_inclusion_type = None;
            }
        }
        if default_checks::is_zero(cleaned.environment_inclusion_frequency) {
            cleaned.environment_inclusion_frequency = None;
        }
        if let Some(environment_class_specific) = &cleaned.environment_class_specific {
            if environment_class_specific.is_empty() {
                cleaned.environment_class_specific = None;
            }
        }

        cleaned
    }
    /// Add a tag to the inorganic raw.
    ///
    /// This handles making sure the tags vector is initialized.
    ///
    /// # Arguments
    ///
    /// * `tag` - The tag to add to the inorganic raw.
    pub fn add_tag(&mut self, tag: InorganicTag) {
        if self.tags.is_none() {
            self.tags = Some(Vec::new());
        }
        if let Some(tags) = self.tags.as_mut() {
            tags.push(tag);
        } else {
            tracing::warn!(
                "Inorganic::add_tag: ({}) Failed to add tag {:?}",
                self.identifier,
                tag
            );
        }
    }

    /// Check whether the inorganic has the specified inorganic tag (found in the `tags` field).
    ///
    /// # Arguments
    ///
    /// * `tag`: The tag to check for.
    ///
    /// # Returns
    ///
    /// Returns true if the inorganic has the specified tag, and false otherwise.
    #[must_use]
    pub fn has_tag(&self, tag: &InorganicTag) -> bool {
        if let Some(tags) = &self.tags {
            for t in tags {
                if std::mem::discriminant(t) == std::mem::discriminant(tag) {
                    return true;
                }
            }
        }
        false
    }
}

#[typetag::serde]
impl RawObject for Inorganic {
    fn get_searchable_tokens(&self) -> Vec<&str> {
        Vec::new()
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
    fn get_name(&self) -> &str {
        &self.identifier
    }
    fn get_metadata(&self) -> RawMetadata {
        self.metadata.as_ref().map_or_else(
            || {
                tracing::warn!("Metadata is missing for Inorganic {}", self.get_object_id());
                RawMetadata::default()
                    .with_object_type(ObjectType::Inorganic)
                    .with_hidden(true)
            },
            std::clone::Clone::clone,
        )
    }
    fn is_empty(&self) -> bool {
        self.identifier.is_empty()
    }
    fn get_type(&self) -> &ObjectType {
        &ObjectType::Inorganic
    }
    fn clean_self(&mut self) {
        *self = self.cleaned();
    }

    fn parse_tag(&mut self, key: &str, value: &str) {
        if INORGANIC_TOKENS.contains_key(key) {
            // For the inorganic tokens, we need to check for (and parse) the MetalOre, ThreadMetal, Environment, and EnvironmentSpecific tokens.
            let token = INORGANIC_TOKENS.get(key).unwrap_or(&InorganicTag::Unknown);

            match token {
                InorganicTag::Environment => {
                    // Environment values are like this: "class:type:frequency"
                    let mut split = value.split(':');
                    // Determine class
                    self.environment_class = Some(
                        *ENVIRONMENT_CLASS_TOKENS
                            .get(split.next().unwrap_or(""))
                            .unwrap_or(&EnvironmentClassTag::None),
                    );
                    // Determine type
                    self.environment_inclusion_type = Some(
                        *INCLUSION_TYPE_TOKENS
                            .get(split.next().unwrap_or(""))
                            .unwrap_or(&InclusionTypeTag::None),
                    );
                    // Determine frequency
                    self.environment_inclusion_frequency =
                        Some(split.next().unwrap_or("0").parse::<u32>().unwrap_or(0));
                }
                InorganicTag::EnvironmentSpecific => {
                    if self.environment_class_specific.is_none() {
                        self.environment_class_specific = Some(Vec::new());
                    }
                    if let Some(environment_class_specific) = &mut self.environment_class_specific {
                        // Environment specific values are like this: "value"
                        environment_class_specific.push(String::from(value));
                    }
                }
                InorganicTag::MetalOre => {
                    if self.metal_ore_chance.is_none() {
                        self.metal_ore_chance = Some(Vec::new());
                    }

                    // Metal ore token values are like this: "metal:d100chance"
                    let mut split = value.split(':');
                    let metal = String::from(split.next().unwrap_or(""));
                    let chance = split.next().unwrap_or("0").parse::<u8>().unwrap_or(0);

                    if let Some(metal_ore_chance) = self.metal_ore_chance.as_mut() {
                        metal_ore_chance.push((metal, chance));
                    }
                }
                InorganicTag::ThreadMetal => {
                    if self.thread_metal_chance.is_none() {
                        self.thread_metal_chance = Some(Vec::new());
                    }

                    // Thread metal token values are like this: "metal:d100chance"
                    let mut split = value.split(':');
                    let metal = String::from(split.next().unwrap_or(""));
                    let chance = split.next().unwrap_or("0").parse::<u8>().unwrap_or(0);

                    if let Some(thread_metal_chance) = self.thread_metal_chance.as_mut() {
                        thread_metal_chance.push((metal, chance));
                    }
                }
                _ => {
                    self.add_tag(*token);
                }
            }

            return;
        }

        // Fall through any remaining tags to the material
        self.material.parse_tag(key, value);
    }

    fn get_object_id(&self) -> &str {
        &self.object_id
    }
}

impl Searchable for Inorganic {
    fn get_search_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();

        // Identifier
        vec.push(self.identifier.clone());
        // Material (if any)
        vec.extend(self.material.get_search_vec());
        // Tags
        if let Some(tags) = &self.tags {
            vec.extend(tags.iter().map(std::string::ToString::to_string));
        }
        // Environment information
        if let Some(environment_class) = &self.environment_class {
            vec.push(environment_class.to_string());
        }
        if let Some(environment_inclusion_type) = &self.environment_inclusion_type {
            vec.push(environment_inclusion_type.to_string());
        }
        if let Some(environment_inclusion_frequency) = &self.environment_inclusion_frequency {
            vec.push(environment_inclusion_frequency.to_string());
        }
        if let Some(environment_class_specific) = &self.environment_class_specific {
            vec.extend(environment_class_specific.iter().cloned());
        }

        clean_search_vec(vec.as_slice())
    }
}
