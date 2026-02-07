use std::str::FromStr;

use crate::{
    tokens::{PositionToken, raw_definitions::POSITION_TOKENS},
    traits::{TagOperations, TokenParser as _},
};

impl TagOperations for PositionToken {
    fn parse(key: &str, value: &str) -> Option<Self> {
        let Some(token) = POSITION_TOKENS.get(key) else {
            tracing::warn!("parse_token: unknown token: {key}");
            return None;
        };

        // Split values into an array if possible, using an empty one if no values exist.
        let values: Vec<&str> = if value.is_empty() {
            Vec::new()
        } else {
            value.split(':').collect()
        };

        match token {
            PositionToken::AccountExempt
            | PositionToken::BragOnKill
            | PositionToken::ChatWorthy
            | PositionToken::ConqueredSite
            | PositionToken::DeterminesCoinDesign
            | PositionToken::DoNotCull
            | PositionToken::DutyBound
            | PositionToken::Elected
            | PositionToken::ExportedInLegends
            | PositionToken::Flashes
            | PositionToken::KillQuest
            | PositionToken::MenialWorkExemption
            | PositionToken::MenialWorkExemptionSpouse
            | PositionToken::MilitaryScreenOnly
            | PositionToken::PunishmentExemption
            | PositionToken::QuestGiver
            | PositionToken::RequiresMarket
            | PositionToken::RulesFromLocation
            | PositionToken::Site
            | PositionToken::SleepPretension
            | PositionToken::SpecialBurial
            | PositionToken::Unknown => token.parse_flag(&values, token.clone()),
            PositionToken::AllowedClass { .. } => {
                token.parse_single(&values, |class| PositionToken::AllowedClass { class })
            }
            PositionToken::AllowedCreature { .. } => {
                token.parse_key_value(&values, |creature, caste| PositionToken::AllowedCreature {
                    creature,
                    caste,
                })
            }
            PositionToken::AppointedBy { .. } => {
                token.parse_single(&values, |position| PositionToken::AppointedBy { position })
            }
            PositionToken::MandateMax { .. } => {
                token.parse_single(&values, |amount| PositionToken::MandateMax { amount })
            }
            PositionToken::Color { .. } => {
                token.parse_array(&values, |[foreground, background, brightness]| {
                    PositionToken::Color {
                        foreground,
                        background,
                        brightness,
                    }
                })
            }
            PositionToken::Commander { .. } => {
                token.parse_key_value(&values, |position, commanded_position| {
                    PositionToken::Commander {
                        position,
                        commanded_position,
                    }
                })
            }

            PositionToken::DemandMax { .. } => {
                token.parse_single(&values, |amount| PositionToken::DemandMax { amount })
            }

            PositionToken::ExecutionSkill { .. } => {
                token.parse_single(&values, |skill| PositionToken::ExecutionSkill { skill })
            }

            PositionToken::Gender { .. } => {
                token.parse_single(&values, |name| PositionToken::Gender { name })
            }

            PositionToken::LandHolder { .. } => token.parse_single(&values, |importance| {
                PositionToken::LandHolder { importance }
            }),
            PositionToken::LandName { .. } => {
                token.parse_single(&values, |name| PositionToken::LandName { name })
            }

            PositionToken::Name { .. } => token.parse_key_value(&values, |singular, plural| {
                PositionToken::Name { singular, plural }
            }),
            PositionToken::NameMale { .. } => token.parse_key_value(&values, |singular, plural| {
                PositionToken::NameMale { singular, plural }
            }),
            PositionToken::NameFemale { .. } => {
                token.parse_key_value(&values, |singular, plural| PositionToken::NameFemale {
                    singular,
                    plural,
                })
            }
            PositionToken::Description { .. } => token.parse_single(&values, |description| {
                PositionToken::Description { description }
            }),
            PositionToken::Number { .. } => {
                token.parse_single(&values, |number| PositionToken::Number { number })
            }
            PositionToken::Precedence { .. } => token.parse_single(&values, |importance| {
                PositionToken::Precedence { importance }
            }),

            PositionToken::RejectedClass { .. } => {
                token.parse_single(&values, |class| PositionToken::RejectedClass { class })
            }
            PositionToken::RejectedCreature { .. } => {
                token.parse_key_value(&values, |creature, caste| PositionToken::RejectedCreature {
                    creature,
                    caste,
                })
            }
            PositionToken::ReplacedBy { .. } => {
                token.parse_single(&values, |position| PositionToken::ReplacedBy { position })
            }
            PositionToken::RequiredBedroom { .. } => {
                token.parse_single(&values, |value| PositionToken::RequiredBedroom { value })
            }
            PositionToken::RequiredBoxes { .. } => {
                token.parse_single(&values, |amount| PositionToken::RequiredBoxes { amount })
            }
            PositionToken::RequiredCabinets { .. } => {
                token.parse_single(&values, |amount| PositionToken::RequiredCabinets { amount })
            }
            PositionToken::RequiredDining { .. } => {
                token.parse_single(&values, |value| PositionToken::RequiredDining { value })
            }
            PositionToken::RequiredOffice { .. } => {
                token.parse_single(&values, |value| PositionToken::RequiredOffice { value })
            }
            PositionToken::RequiredRacks { .. } => {
                token.parse_single(&values, |amount| PositionToken::RequiredRacks { amount })
            }
            PositionToken::RequiredStands { .. } => {
                token.parse_single(&values, |amount| PositionToken::RequiredStands { amount })
            }
            PositionToken::RequiredTomb { .. } => {
                token.parse_single(&values, |value| PositionToken::RequiredTomb { value })
            }

            PositionToken::RequiresPopulation { .. } => token.parse_single(&values, |population| {
                PositionToken::RequiresPopulation { population }
            }),
            PositionToken::Responsibility { .. } => token.parse_single(&values, |responsibility| {
                PositionToken::Responsibility { responsibility }
            }),

            PositionToken::Spouse { .. } => token.parse_key_value(&values, |singular, plural| {
                PositionToken::Spouse { singular, plural }
            }),
            PositionToken::SpouseFemale { .. } => {
                token.parse_key_value(&values, |singular, plural| PositionToken::SpouseFemale {
                    singular,
                    plural,
                })
            }
            PositionToken::SpouseMale { .. } => {
                token.parse_key_value(&values, |singular, plural| PositionToken::SpouseMale {
                    singular,
                    plural,
                })
            }
            PositionToken::Squad { .. } => {
                token.parse_labeled_array(&values, |headcount, [singular, plural]| {
                    PositionToken::Squad {
                        headcount,
                        singular,
                        plural,
                    }
                })
            }
            PositionToken::Succession { .. } => {
                token.parse_single(&values, |inheritor| PositionToken::Succession { inheritor })
            }
        }
    }
}

impl FromStr for PositionToken {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s
            .strip_prefix('[')
            .unwrap_or(s)
            .strip_suffix(']')
            .unwrap_or(s);

        if let Some((key, value)) = trimmed.split_once(':') {
            match Self::parse(key, value) {
                Some(token) => Ok(token),
                None => Err(format!("PositionToken unable to parse {s}")),
            }
        } else {
            match Self::parse(trimmed, "") {
                Some(token) => Ok(token),
                None => Err(format!("PositionToken unable to parse {s}")),
            }
        }
    }
}
