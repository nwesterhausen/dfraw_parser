use crate::{
    metadata::NumericToken,
    tags::CreatureTag,
    traits::{NumericTokenTransform, RawObjectToken},
};

impl NumericTokenTransform for CreatureTag {
    fn as_numeric_tokens(&self) -> Vec<NumericToken> {
        let mut tokens = Vec::new();
        let prefix = self.get_key().unwrap_or_default();
        match self {
            // Single Value Tags
            // CasteTag::Child { age } | CasteTag::Baby { age } => {
            //     tokens.push(NumericToken::new(prefix, *age));
            // }
            CreatureTag::Frequency { frequency } => {
                tokens.push(NumericToken::new(prefix, *frequency));
            }

            // Combo-key simple value
            // CasteTag::NaturalSkill { skill, level } => {
            //     tokens.push(NumericToken::new(format!("{prefix}_{skill}"), *level));
            // }

            // Range Tags (Min/Max)
            // CasteTag::ClutchSize { min, max } => {
            //     tokens.push(NumericToken::new(format!("{prefix}_MIN"), *min));
            //     tokens.push(NumericToken::new(format!("{prefix}_MAX"), *max));
            // }
            CreatureTag::ClusterNumber { min, max }
            | CreatureTag::PopulationNumber { min, max }
            | CreatureTag::UndergroundDepth { min, max } => {
                tokens.push(NumericToken::new(format!("{prefix}_MIN"), *min));
                tokens.push(NumericToken::new(format!("{prefix}_MAX"), *max));
            }

            // Complex Tags
            // CasteTag::BodySize { year, days, size } => {
            //     tokens.push(NumericToken::new(format!("{prefix}_YEAR"), *year));
            //     tokens.push(NumericToken::new(format!("{prefix}_DAY"), *days));
            //     tokens.push(NumericToken::new(format!("{prefix}_SIZE"), *size));
            // }
            _ => {}
        }
        tokens
    }
}
