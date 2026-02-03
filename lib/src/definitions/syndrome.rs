//! Syndrome struct and implementation

use dfraw_parser_proc_macros::IsEmpty;
use tracing::{debug, warn};

use crate::tokens::{
    SyndromeToken,
    raw_definitions::{CREATURE_EFFECT_TOKENS, SYNDROME_TOKENS},
};

/// A struct representing a syndrome
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
pub struct Syndrome {
    /// Seen the `[SYN_IDENTIFIER:INEBRIATION]` tag in `material_templates.txt`
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    identifier: Option<String>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    name: Option<String>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    affected_classes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    immune_classes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    affected_creatures: Option<Vec<(String, String)>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    immune_creatures: Option<Vec<(String, String)>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    classes: Option<Vec<String>>,

    /// Seen the `[SYN_CONCENTRATION_ADDED:100:1000]` tag in `material_templates.txt`
    /// default is 0:0
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    concentration_added: Option<[u32; 2]>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    tags: Option<Vec<SyndromeToken>>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    conditions: Option<Vec<String>>,
}

impl Syndrome {
    /// Creates a new Syndrome struct
    ///
    /// # Returns
    ///
    /// * `Syndrome` - The new Syndrome struct
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Creates a new Syndrome struct with the given name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the syndrome
    ///
    /// # Returns
    ///
    /// * `Syndrome` - The new Syndrome struct
    #[must_use]
    pub fn from_name(name: &str) -> Self {
        Self {
            name: Some(String::from(name)),
            ..Self::default()
        }
    }
    /// Parses a tag into the Syndrome struct
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the tag
    /// * `value` - The value of the tag
    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    pub fn parse_tag(&mut self, key: &str, value: &str) {
        if CREATURE_EFFECT_TOKENS.contains_key(key) {
            if self.conditions.is_none() {
                self.conditions = Some(Vec::new());
            }

            if let Some(conditions) = self.conditions.as_mut() {
                conditions.push(String::from(value));
            }
            return;
        }
        if key == "CE" {
            debug!("Manual handling of CE tag: {}:{}", key, value);
            if self.conditions.is_none() {
                self.conditions = Some(Vec::new());
            }

            if let Some(conditions) = self.conditions.as_mut() {
                conditions.push(String::from(value));
            }
            return;
        }

        let token = SYNDROME_TOKENS.get(key).unwrap_or(&SyndromeToken::Unknown);
        match token {
            SyndromeToken::Name => self.name = Some(String::from(value)),
            SyndromeToken::Identifier => self.identifier = Some(String::from(value)),
            SyndromeToken::AffectedClass => {
                if self.affected_classes.is_none() {
                    self.affected_classes = Some(Vec::new());
                }
                if let Some(affected_classes) = self.affected_classes.as_mut() {
                    affected_classes.push(String::from(value));
                }
            }
            SyndromeToken::ImmuneClass => {
                if self.immune_classes.is_none() {
                    self.immune_classes = Some(Vec::new());
                }
                if let Some(immune_classes) = self.immune_classes.as_mut() {
                    immune_classes.push(String::from(value));
                }
            }
            SyndromeToken::AffectedCreature => {
                if self.affected_creatures.is_none() {
                    self.affected_creatures = Some(Vec::new());
                }

                let mut split = value.split(':');
                let creature = split.next().unwrap_or_default().trim();
                let caste = split.next().unwrap_or_default().trim();

                if let Some(affected_creatures) = self.affected_creatures.as_mut() {
                    affected_creatures.push((String::from(creature), String::from(caste)));
                }
            }
            SyndromeToken::ImmuneCreature => {
                if self.immune_creatures.is_none() {
                    self.immune_creatures = Some(Vec::new());
                }

                let mut split = value.split(':');
                let creature = split.next().unwrap_or_default().trim();
                let caste = split.next().unwrap_or_default().trim();

                if let Some(immune_creatures) = self.immune_creatures.as_mut() {
                    immune_creatures.push((String::from(creature), String::from(caste)));
                }
            }
            SyndromeToken::ConcentrationAdded => {
                let mut split = value.split(':');
                let min = split.next().unwrap_or_default().trim();
                let max = split.next().unwrap_or_default().trim();
                self.concentration_added = Some([
                    min.parse::<u32>().unwrap_or_default(),
                    max.parse::<u32>().unwrap_or_default(),
                ]);
            }
            SyndromeToken::Injected => {
                if self.tags.is_none() {
                    self.tags = Some(Vec::new());
                }
                if let Some(tags) = self.tags.as_mut() {
                    tags.push(SyndromeToken::Injected);
                }
            }
            SyndromeToken::Contact => {
                if self.tags.is_none() {
                    self.tags = Some(Vec::new());
                }
                if let Some(tags) = self.tags.as_mut() {
                    tags.push(SyndromeToken::Contact);
                }
            }
            SyndromeToken::Inhaled => {
                if self.tags.is_none() {
                    self.tags = Some(Vec::new());
                }
                if let Some(tags) = self.tags.as_mut() {
                    tags.push(SyndromeToken::Inhaled);
                }
            }
            SyndromeToken::Ingested => {
                if self.tags.is_none() {
                    self.tags = Some(Vec::new());
                }
                if let Some(tags) = self.tags.as_mut() {
                    tags.push(SyndromeToken::Ingested);
                }
            }
            SyndromeToken::Unknown => {
                warn!("Unknown syndrome token: {}", key);
            }
            SyndromeToken::Class => {
                if self.classes.is_none() {
                    self.classes = Some(Vec::new());
                }
                if let Some(classes) = self.classes.as_mut() {
                    classes.push(String::from(value));
                }
            }
            SyndromeToken::NoHospital => {
                if self.tags.is_none() {
                    self.tags = Some(Vec::new());
                }
                if let Some(tags) = self.tags.as_mut() {
                    tags.push(SyndromeToken::NoHospital);
                }
            }
        }
    }
}
