use crate::{
    metadata::NumericToken,
    tags::CasteTag,
    traits::{NumericTokenTransform, RawObjectToken},
};

impl NumericTokenTransform for CasteTag {
    fn as_numeric_tokens(&self) -> Vec<NumericToken> {
        let mut tokens = Vec::new();
        let prefix = self.get_key().unwrap_or_default();
        match self {
            // Single Value Tags
            CasteTag::Child { age } | CasteTag::Baby { age } => {
                tokens.push(NumericToken::new(String::from(prefix), *age));
            }
            CasteTag::BeachFrequency { frequency } => {
                tokens.push(NumericToken::new(String::from(prefix), *frequency));
            }
            CasteTag::Difficulty { difficulty } => {
                tokens.push(NumericToken::new(String::from(prefix), *difficulty));
            }
            CasteTag::EggSize { size } => {
                tokens.push(NumericToken::new(String::from(prefix), *size));
            }
            CasteTag::FixedTemp { temperature } => {
                tokens.push(NumericToken::new(String::from(prefix), *temperature));
            }
            CasteTag::GrassTrample { trample } => {
                tokens.push(NumericToken::new(String::from(prefix), *trample));
            }
            CasteTag::GravitateBodySize { target } => {
                tokens.push(NumericToken::new(String::from(prefix), *target));
            }
            CasteTag::Grazer { grazer } => {
                tokens.push(NumericToken::new(String::from(prefix), *grazer));
            }
            CasteTag::Homeotherm { temperature } => {
                tokens.push(NumericToken::new(String::from(prefix), *temperature));
            }
            CasteTag::ItemCorpseQuality { quality } => {
                tokens.push(NumericToken::new(String::from(prefix), *quality));
            }
            CasteTag::LowLightVision { vision } => {
                tokens.push(NumericToken::new(String::from(prefix), *vision));
            }
            CasteTag::PenetratePower { penetrate_power } => {
                tokens.push(NumericToken::new(String::from(prefix), *penetrate_power));
            }
            CasteTag::PetValue { pet_value } => {
                tokens.push(NumericToken::new(String::from(prefix), *pet_value));
            }
            CasteTag::PopulationRatio { pop_ratio } => {
                tokens.push(NumericToken::new(String::from(prefix), *pop_ratio));
            }
            CasteTag::ProneToRage { rage_chance } => {
                tokens.push(NumericToken::new(String::from(prefix), *rage_chance));
            }
            CasteTag::TradeCapacity { capacity } => {
                tokens.push(NumericToken::new(String::from(prefix), *capacity));
            }
            CasteTag::ViewRange { view_range } => {
                tokens.push(NumericToken::new(String::from(prefix), *view_range));
            }

            // Combo-key simple value
            CasteTag::NaturalSkill { skill, level } => {
                tokens.push(NumericToken::new(format!("{prefix}_{skill}"), *level));
            }
            CasteTag::SkillLearnRate { skill, rate } => {
                tokens.push(NumericToken::new(format!("{prefix}_{skill}"), *rate));
            }
            CasteTag::SyndromeDilutionFactor {
                syndrome,
                percentage,
            } => {
                tokens.push(NumericToken::new(
                    format!("{prefix}_{syndrome}"),
                    *percentage,
                ));
            }

            // Range Tags (Min/Max)
            CasteTag::ClutchSize { min, max } => {
                tokens.push(NumericToken::new(format!("{prefix}_MIN"), *min));
                tokens.push(NumericToken::new(format!("{prefix}_MAX"), *max));
            }
            CasteTag::LitterSize { min, max } => {
                tokens.push(NumericToken::new(format!("{prefix}_MIN"), *min));
                tokens.push(NumericToken::new(format!("{prefix}_MAX"), *max));
            }
            CasteTag::MaxAge { min, max } => {
                tokens.push(NumericToken::new(format!("{prefix}_MIN"), *min));
                tokens.push(NumericToken::new(format!("{prefix}_MAX"), *max));
            }
            CasteTag::VisionArc {
                binocular,
                non_binocular,
            } => {
                tokens.push(NumericToken::new(format!("{prefix}_BINOCULAR"), *binocular));
                tokens.push(NumericToken::new(
                    format!("{prefix}_NON_BINOCULAR"),
                    *non_binocular,
                ));
            }

            // Complex Tags
            CasteTag::AttackTrigger {
                population,
                exported_wealth,
                created_wealth,
            } => {
                tokens.push(NumericToken::new(
                    format!("{prefix}_POPULATION"),
                    *population,
                ));
                tokens.push(NumericToken::new(
                    format!("{prefix}_EXPORTED"),
                    *exported_wealth,
                ));
                tokens.push(NumericToken::new(
                    format!("{prefix}_CREATED"),
                    *created_wealth,
                ));
            }
            CasteTag::BodySize { year, days, size } => {
                tokens.push(NumericToken::new(format!("{prefix}_YEAR"), *year));
                tokens.push(NumericToken::new(format!("{prefix}_DAY"), *days));
                tokens.push(NumericToken::new(format!("{prefix}_SIZE"), *size));
            }
            _ => {}
        }
        tokens
    }
}
