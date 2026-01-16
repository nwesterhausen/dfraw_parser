//! Contains the Position struct and implementation (for government positions)
use dfraw_parser_proc_macros::{Cleanable, IsEmpty};

use crate::{color::Color, name::Name, tags::PositionTag};

/// Represents a position in the government of an entity
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
pub struct Position {
    identifier: String,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    allowed_classes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    allowed_creatures: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    appointed_by: Option<String>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    color: Option<Color>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    commander: Option<String>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    demand_max: Option<u32>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    execution_skill: Option<String>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    gender: Option<String>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    land_holder: Option<u32>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    land_name: Option<String>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    mandate_max: Option<u32>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    name: Option<Name>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    name_male: Option<Name>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    name_female: Option<Name>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    number: Option<i32>, //set -1 for AS_NEEDED
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    precedence: Option<i32>, //set -1 for NONE
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    rejected_classes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    rejected_creatures: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    replaced_by: Option<String>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    required_bedroom: Option<u32>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    required_boxes: Option<u32>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    required_cabinets: Option<u32>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    required_dining: Option<u32>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    required_office: Option<u32>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    required_racks: Option<u32>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    required_stands: Option<u32>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    required_tomb: Option<u32>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    requires_population: Option<u32>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    responsibilities: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    spouse: Option<Name>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    spouse_female: Option<Name>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    spouse_male: Option<Name>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    squad: Option<String>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    succession: Option<String>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    tags: Vec<PositionTag>,
}

impl Position {
    /// Creates a new Position struct with the given identifier
    ///
    /// # Arguments
    ///
    /// * `identifier` - The identifier of the position
    #[must_use]
    pub fn new(identifier: String) -> Self {
        Self {
            identifier,
            ..Default::default()
        }
    }
    /// Parses a tag and value into the position
    ///
    /// # Arguments
    ///
    /// * `key` - The tag to parse
    /// * `value` - The value to parse
    pub fn parse_tag(&mut self, key: &PositionTag, value: &str) {
        match key {
            PositionTag::AllowedClass => {
                if self.allowed_classes.is_none() {
                    self.allowed_classes = Some(Vec::new());
                }
                if let Some(allowed_classes) = self.allowed_classes.as_mut() {
                    allowed_classes.push(value.to_string());
                }
            }
            PositionTag::AllowedCreature => {
                if self.allowed_creatures.is_none() {
                    self.allowed_creatures = Some(Vec::new());
                }
                if let Some(allowed_creatures) = self.allowed_creatures.as_mut() {
                    allowed_creatures.push(value.to_string());
                }
            }
            PositionTag::AppointedBy => self.appointed_by = Some(value.to_string()),
            PositionTag::Color => self.color = Some(Color::from_value(value)),
            PositionTag::Commander => self.commander = Some(value.to_string()),
            PositionTag::DemandMax => self.demand_max = Some(value.parse().unwrap_or_default()),
            PositionTag::ExecutionSkill => self.execution_skill = Some(value.to_string()),
            PositionTag::Gender => self.gender = Some(value.to_string()),
            PositionTag::LandHolder => self.land_holder = Some(value.parse().unwrap_or_default()),
            PositionTag::LandName => self.land_name = Some(value.to_string()),
            PositionTag::MandateMax => self.mandate_max = Some(value.parse().unwrap_or_default()),
            PositionTag::Name => self.name = Some(Name::from_value(value)),
            PositionTag::Spouse => self.spouse = Some(Name::from_value(value)),
            PositionTag::NameFemale => self.name_female = Some(Name::from_value(value)),
            PositionTag::SpouseFemale => {
                self.spouse_female = Some(Name::from_value(value));
            }
            PositionTag::NameMale => self.name_male = Some(Name::from_value(value)),
            PositionTag::SpouseMale => self.spouse_male = Some(Name::from_value(value)),
            PositionTag::Number => self.number = Some(value.parse().unwrap_or_default()),
            PositionTag::Precedence => self.precedence = Some(value.parse().unwrap_or_default()),
            PositionTag::RejectedClass => {
                if self.rejected_classes.is_none() {
                    self.rejected_classes = Some(Vec::new());
                }
                if let Some(rejected_classes) = self.rejected_classes.as_mut() {
                    rejected_classes.push(value.to_string());
                }
            }
            PositionTag::RejectedCreature => {
                if self.rejected_creatures.is_none() {
                    self.rejected_creatures = Some(Vec::new());
                }
                if let Some(rejected_creatures) = self.rejected_creatures.as_mut() {
                    rejected_creatures.push(value.to_string());
                }
            }
            PositionTag::ReplacedBy => self.replaced_by = Some(value.to_string()),
            PositionTag::RequiredBedroom => {
                self.required_bedroom = Some(value.parse().unwrap_or_default());
            }
            PositionTag::RequiredBoxes => {
                self.required_boxes = Some(value.parse().unwrap_or_default());
            }
            PositionTag::RequiredCabinets => {
                self.required_cabinets = Some(value.parse().unwrap_or_default());
            }
            PositionTag::RequiredDining => {
                self.required_dining = Some(value.parse().unwrap_or_default());
            }
            PositionTag::RequiredOffice => {
                self.required_office = Some(value.parse().unwrap_or_default());
            }
            PositionTag::RequiredRacks => {
                self.required_racks = Some(value.parse().unwrap_or_default());
            }
            PositionTag::RequiredStands => {
                self.required_stands = Some(value.parse().unwrap_or_default());
            }
            PositionTag::RequiredTomb => {
                self.required_tomb = Some(value.parse().unwrap_or_default());
            }
            PositionTag::RequiresPopulation => {
                self.requires_population = Some(value.parse().unwrap_or_default());
            }
            PositionTag::Responsibility => {
                if self.responsibilities.is_none() {
                    self.responsibilities = Some(Vec::new());
                }
                if let Some(responsibilities) = self.responsibilities.as_mut() {
                    responsibilities.push(value.to_string());
                }
            }
            PositionTag::Squad => self.squad = Some(value.to_string()),
            PositionTag::Succession => self.succession = Some(value.to_string()),
            _ => self.tags.push(*key),
        }
    }
}
