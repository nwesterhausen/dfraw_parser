//! Parsed Inorganic object definition.
use dfraw_parser_proc_macros::{Cleanable, IsEmpty};
use uuid::Uuid;

use crate::{
    Material,
    metadata::RawMetadata,
    raw_definitions::{ENVIRONMENT_CLASS_TOKENS, INCLUSION_TYPE_TOKENS, INORGANIC_TOKENS},
    tokens::{EnvironmentClassToken, InclusionTypeToken, InorganicToken, ObjectType},
    traits::RawObject,
    utilities::generate_object_id_using_raw_metadata,
};

/// The raw representation of an inorganic object.
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
pub struct Inorganic {
    identifier: String,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    metadata: Option<RawMetadata>,
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
    material: Material,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    metal_ore_chance: Option<Vec<(String, u8)>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    thread_metal_chance: Option<Vec<(String, u8)>>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    environment_class: Option<EnvironmentClassToken>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    environment_inclusion_type: Option<InclusionTypeToken>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    environment_inclusion_frequency: Option<u32>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    environment_class_specific: Option<Vec<String>>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    tags: Option<Vec<InorganicToken>>,
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
            object_id: generate_object_id_using_raw_metadata(
                identifier,
                ObjectType::Inorganic,
                metadata,
            ),
            ..Self::default()
        }
    }
    #[must_use]
    pub fn get_tags(&self) -> Vec<InorganicToken> {
        if self.tags.is_none() {
            return Vec::new();
        }

        let mut ret_tags = Vec::new();
        if let Some(tags) = &self.tags {
            for tag in tags {
                ret_tags.push(*tag);
            }
        }
        ret_tags
    }

    /// Add a tag to the inorganic raw.
    ///
    /// This handles making sure the tags vector is initialized.
    ///
    /// # Arguments
    ///
    /// * `tag` - The tag to add to the inorganic raw.
    pub fn add_tag(&mut self, tag: InorganicToken) {
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
    pub fn has_tag(&self, tag: &InorganicToken) -> bool {
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
    fn get_type(&self) -> ObjectType {
        ObjectType::Inorganic
    }

    fn parse_tag(&mut self, key: &str, value: &str) {
        if INORGANIC_TOKENS.contains_key(key) {
            // For the inorganic tokens, we need to check for (and parse) the MetalOre, ThreadMetal, Environment, and EnvironmentSpecific tokens.
            let token = INORGANIC_TOKENS
                .get(key)
                .unwrap_or(&InorganicToken::Unknown);

            match token {
                InorganicToken::Environment => {
                    // Environment values are like this: "class:type:frequency"
                    let mut split = value.split(':');
                    // Determine class
                    self.environment_class = Some(
                        *ENVIRONMENT_CLASS_TOKENS
                            .get(split.next().unwrap_or(""))
                            .unwrap_or(&EnvironmentClassToken::None),
                    );
                    // Determine type
                    self.environment_inclusion_type = Some(
                        *INCLUSION_TYPE_TOKENS
                            .get(split.next().unwrap_or(""))
                            .unwrap_or(&InclusionTypeToken::None),
                    );
                    // Determine frequency
                    self.environment_inclusion_frequency =
                        Some(split.next().unwrap_or("0").parse::<u32>().unwrap_or(0));
                }
                InorganicToken::EnvironmentSpecific => {
                    if self.environment_class_specific.is_none() {
                        self.environment_class_specific = Some(Vec::new());
                    }
                    if let Some(environment_class_specific) = &mut self.environment_class_specific {
                        // Environment specific values are like this: "value"
                        environment_class_specific.push(String::from(value));
                    }
                }
                InorganicToken::MetalOre => {
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
                InorganicToken::ThreadMetal => {
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

    fn get_object_id(&self) -> Uuid {
        self.object_id
    }
    fn get_module_object_id(&self) -> Uuid {
        match &self.metadata {
            Some(meta) => meta.get_module_object_id(),
            None => Uuid::nil(),
        }
    }
}
