//! Contains the Position struct and implementation (for government positions)

use crate::{color::Color, default_checks, name::Name, tags::PositionTag};

/// Represents a position in the government of an entity
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    identifier: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_classes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_creatures: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    appointed_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commander: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    demand_max: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_skill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    land_holder: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    land_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mandate_max: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Name>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_male: Option<Name>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_female: Option<Name>,

    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<i32>, //set -1 for AS_NEEDED
    #[serde(skip_serializing_if = "Option::is_none")]
    precedence: Option<i32>, //set -1 for NONE
    #[serde(skip_serializing_if = "Option::is_none")]
    rejected_classes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rejected_creatures: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replaced_by: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    required_bedroom: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_boxes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_cabinets: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_dining: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_office: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_racks: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_stands: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_tomb: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requires_population: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    responsibilities: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spouse: Option<Name>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spouse_female: Option<Name>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spouse_male: Option<Name>,
    #[serde(skip_serializing_if = "Option::is_none")]
    squad: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    succession: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
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
    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    #[must_use]
    pub fn cleaned(&self) -> Self {
        let mut cleaned = self.clone();

        if let Some(allowed_classes) = &cleaned.allowed_classes
            && allowed_classes.is_empty()
        {
            cleaned.allowed_classes = None;
        }

        if let Some(allowed_creatures) = &cleaned.allowed_creatures
            && allowed_creatures.is_empty()
        {
            cleaned.allowed_creatures = None;
        }
        if let Some(rejected_classes) = &cleaned.rejected_classes
            && rejected_classes.is_empty()
        {
            cleaned.rejected_classes = None;
        }
        if let Some(appointed_by) = &cleaned.appointed_by
            && appointed_by.is_empty()
        {
            cleaned.appointed_by = None;
        }
        if let Some(color) = &cleaned.color
            && color.is_default()
        {
            cleaned.color = None;
        }
        if let Some(commander) = &cleaned.commander
            && commander.is_empty()
        {
            cleaned.commander = None;
        }
        if default_checks::is_zero_u32(cleaned.demand_max) {
            cleaned.demand_max = None;
        }
        if let Some(execution_skill) = &cleaned.execution_skill
            && execution_skill.is_empty()
        {
            cleaned.execution_skill = None;
        }
        if let Some(gender) = &cleaned.gender
            && gender.is_empty()
        {
            cleaned.gender = None;
        }
        if default_checks::is_zero_u32(cleaned.land_holder) {
            cleaned.land_holder = None;
        }
        if let Some(land_name) = &cleaned.land_name
            && land_name.is_empty()
        {
            cleaned.land_name = None;
        }
        if default_checks::is_zero_u32(cleaned.mandate_max) {
            cleaned.mandate_max = None;
        }
        if let Some(name) = &cleaned.name
            && name.is_empty()
        {
            cleaned.name = None;
        }
        if let Some(name_male) = &cleaned.name_male
            && name_male.is_empty()
        {
            cleaned.name_male = None;
        }
        if let Some(name_female) = &cleaned.name_female
            && name_female.is_empty()
        {
            cleaned.name_female = None;
        }
        if default_checks::is_zero_i32(cleaned.number) {
            cleaned.number = None;
        }
        if default_checks::is_zero_i32(cleaned.precedence) {
            cleaned.precedence = None;
        }
        if let Some(rejected_creatures) = &cleaned.rejected_creatures
            && rejected_creatures.is_empty()
        {
            cleaned.rejected_creatures = None;
        }
        if let Some(replaced_by) = &cleaned.replaced_by
            && replaced_by.is_empty()
        {
            cleaned.replaced_by = None;
        }
        if default_checks::is_zero_u32(cleaned.required_bedroom) {
            cleaned.required_bedroom = None;
        }
        if default_checks::is_zero_u32(cleaned.required_boxes) {
            cleaned.required_boxes = None;
        }
        if default_checks::is_zero_u32(cleaned.required_cabinets) {
            cleaned.required_cabinets = None;
        }
        if default_checks::is_zero_u32(cleaned.required_dining) {
            cleaned.required_dining = None;
        }
        if default_checks::is_zero_u32(cleaned.required_office) {
            cleaned.required_office = None;
        }
        if default_checks::is_zero_u32(cleaned.required_racks) {
            cleaned.required_racks = None;
        }
        if default_checks::is_zero_u32(cleaned.required_stands) {
            cleaned.required_stands = None;
        }
        if default_checks::is_zero_u32(cleaned.required_tomb) {
            cleaned.required_tomb = None;
        }
        if default_checks::is_zero_u32(cleaned.requires_population) {
            cleaned.requires_population = None;
        }
        if let Some(responsibilities) = &cleaned.responsibilities
            && responsibilities.is_empty()
        {
            cleaned.responsibilities = None;
        }
        if let Some(spouse) = &cleaned.spouse
            && spouse.is_empty()
        {
            cleaned.spouse = None;
        }
        if let Some(spouse_female) = &cleaned.spouse_female
            && spouse_female.is_empty()
        {
            cleaned.spouse_female = None;
        }
        if let Some(spouse_male) = &cleaned.spouse_male
            && spouse_male.is_empty()
        {
            cleaned.spouse_male = None;
        }
        if let Some(squad) = &cleaned.squad
            && squad.is_empty()
        {
            cleaned.squad = None;
        }
        if let Some(succession) = &cleaned.succession
            && succession.is_empty()
        {
            cleaned.succession = None;
        }

        cleaned
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
