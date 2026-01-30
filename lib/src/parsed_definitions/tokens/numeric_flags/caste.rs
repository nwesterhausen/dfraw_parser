use crate::{
    metadata::NumericToken,
    tokens::CasteToken,
    traits::{NumericTokenTransform, RawToken},
};

impl NumericTokenTransform for CasteToken {
    fn as_numeric_tokens(&self) -> Vec<NumericToken> {
        let mut tokens = Vec::new();
        let prefix = self.get_key().unwrap_or_default();
        match self {
            // Single Value Tags
            CasteToken::Child { age } | CasteToken::Baby { age } => {
                tokens.push(NumericToken::new(String::from(prefix), *age));
            }
            CasteToken::BeachFrequency { frequency } => {
                tokens.push(NumericToken::new(String::from(prefix), *frequency));
            }
            CasteToken::Difficulty { difficulty } => {
                tokens.push(NumericToken::new(String::from(prefix), *difficulty));
            }
            CasteToken::EggSize { size } => {
                tokens.push(NumericToken::new(String::from(prefix), *size));
            }
            CasteToken::FixedTemp { temperature } => {
                tokens.push(NumericToken::new(String::from(prefix), *temperature));
            }
            CasteToken::GrassTrample { trample } => {
                tokens.push(NumericToken::new(String::from(prefix), *trample));
            }
            CasteToken::GravitateBodySize { target } => {
                tokens.push(NumericToken::new(String::from(prefix), *target));
            }
            CasteToken::Grazer { grazer } => {
                tokens.push(NumericToken::new(String::from(prefix), *grazer));
            }
            CasteToken::Homeotherm { temperature } => {
                tokens.push(NumericToken::new(String::from(prefix), *temperature));
            }
            CasteToken::ItemCorpseQuality { quality } => {
                tokens.push(NumericToken::new(String::from(prefix), *quality));
            }
            CasteToken::LowLightVision { vision } => {
                tokens.push(NumericToken::new(String::from(prefix), *vision));
            }
            CasteToken::PenetratePower { penetrate_power } => {
                tokens.push(NumericToken::new(String::from(prefix), *penetrate_power));
            }
            CasteToken::PetValue { pet_value } => {
                tokens.push(NumericToken::new(String::from(prefix), *pet_value));
            }
            CasteToken::PopulationRatio { pop_ratio } => {
                tokens.push(NumericToken::new(String::from(prefix), *pop_ratio));
            }
            CasteToken::ProneToRage { rage_chance } => {
                tokens.push(NumericToken::new(String::from(prefix), *rage_chance));
            }
            CasteToken::TradeCapacity { capacity } => {
                tokens.push(NumericToken::new(String::from(prefix), *capacity));
            }
            CasteToken::ViewRange { view_range } => {
                tokens.push(NumericToken::new(String::from(prefix), *view_range));
            }

            // Combo-key simple value
            CasteToken::NaturalSkill { skill, level } => {
                tokens.push(NumericToken::new(format!("{prefix}_{skill}"), *level));
            }
            CasteToken::SkillLearnRate { skill, rate } => {
                tokens.push(NumericToken::new(format!("{prefix}_{skill}"), *rate));
            }
            CasteToken::SyndromeDilutionFactor {
                syndrome,
                percentage,
            } => {
                tokens.push(NumericToken::new(
                    format!("{prefix}_{syndrome}"),
                    *percentage,
                ));
            }

            // Range Tags (Min/Max)
            CasteToken::ClutchSize { min, max } => {
                tokens.push(NumericToken::new(format!("{prefix}_MIN"), *min));
                tokens.push(NumericToken::new(format!("{prefix}_MAX"), *max));
            }
            CasteToken::LitterSize { min, max } => {
                tokens.push(NumericToken::new(format!("{prefix}_MIN"), *min));
                tokens.push(NumericToken::new(format!("{prefix}_MAX"), *max));
            }
            CasteToken::MaxAge { min, max } => {
                tokens.push(NumericToken::new(format!("{prefix}_MIN"), *min));
                tokens.push(NumericToken::new(format!("{prefix}_MAX"), *max));
            }
            CasteToken::VisionArc {
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
            CasteToken::AttackTrigger {
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
            CasteToken::BodySize { year, days, size } => {
                tokens.push(NumericToken::new(format!("{prefix}_YEAR"), *year));
                tokens.push(NumericToken::new(format!("{prefix}_DAY"), *days));
                tokens.push(NumericToken::new(format!("{prefix}_SIZE"), *size));
            }
            _ => {}
        }
        tokens
    }
}
