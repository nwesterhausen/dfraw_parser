//! The parsing implementation for `CasteTag`

use crate::{
    metadata::{TagComplexity, OBJECT_TOKEN_MAP},
    raw_definitions::CASTE_TOKENS,
    tags::CasteTag,
    traits::TagOperations,
};

impl TagOperations for CasteTag {
    #[allow(clippy::too_many_lines)]
    fn get_complexity(&self) -> TagComplexity {
        match self {
          Self::ApplyCreatureVariation { .. } |
          Self::Attack { .. } |
          Self::AttackTrigger { .. } |
          Self::BabyName {.. } |
          Self::Body { .. } |
          Self::Blood {..}|
          Self::BodyAppearanceModifier { .. }|
          Self::BodyDetailPlan { .. } |
          Self::BodyPartAppearanceModifier {..}|
          Self::BodySize {..}|
            Self::ChildName { .. } |
            Self::ClutchSize { .. } |
            Self::Color { .. } |
            Self::EggMaterial { .. } |
            Self::ExtraButcherObjectItem { .. } |
            Self::ExtraButcherObject { .. } |
            Self::GeneralMaterialForceMultiplier { .. } |
            Self::GlowColor { .. } |
            Self::GobbleVerminCreature { .. } |
            Self::InteractionDetail { .. } |
            Self::ItemCorpse { .. } |
            Self::Lair { .. } |
            Self::LaysUnusualEggs {..}|
            Self::Ligaments { .. } |
            Self::LitterSize { .. } |
            Self::MannerismFingers { .. } |
            Self::MaxAge { .. } |
            Self::MentalAttributeCapPercentage { .. } |
            Self::MentalAttributeRange { .. } |
            Self::MentalAttributeRate { .. } |
            Self::Milkable { .. } |
            Self::Name { .. } |
            Self::NaturalSkill { .. } |
            Self::Orientation { .. } |
            Self::Personality { .. } |
            Self::PhysicalAttributeCapPercentage { .. } |
            Self::PhysicalAttributeRange { .. } |
            Self::PhysicalAttributeRate { .. } |
            Self::ProfessionName { .. } |
            Self::Pus { .. } |
            Self::RelativeSize { .. } |
            Self::Remains { .. } |
            Self::RetractIntoBodyPart { .. } |
            Self::RootAround { .. } |
            Self::Secretion { .. } |
            Self::SenseCreatureClass { .. } |
            Self::SetBodyPartGroup { .. } |
            Self::SkillLearnRate { .. } |
            Self::SkillRate { .. } |
            Self::SkillRates { .. } |
            Self::SkillRustRate { .. } |
            Self::SkillRustRates { .. } |
            Self::Sound { .. } |
            Self::SpecificFood { .. } |
            Self::SyndromeDilutionFactor { .. } |
            Self::Tendons { .. } |
            Self::TissueLayer { .. } |
            Self::TissueLayerUnder { .. } |
            Self::VerminBite { .. } |
            Self::VisionArc { .. }
          => {
                tracing::trace!("get_complexity: {self:?} is 'Complex'");
                TagComplexity::Complex
        }
          Self::AltTile { .. } |
          Self::Baby { .. } |
          Self::BeachFrequency { .. } |
          Self::BodyGloss { .. } |
          Self::BodyPartAddType { .. } |
          Self::BodyPartRemoveType { .. } |
          Self::BuildingDestroyer { .. } |
          Self::CanDoInteraction { .. } |
          Self::ChangeBodySizePercent { .. } |
          Self::Child { .. } |
          Self::CreatureClass { .. } |
          Self::CreatureVariationAddTag { .. } |
          Self::CreatureVariationRemoveTag { .. } |
          Self::Description { .. } |
          Self::Difficulty { .. } |
          Self::ExtraButcherObjectShape { .. } |
          Self::EggSize { .. } |
          Self::Extract { .. } |
          Self::FixedTemp { .. } |
          Self::Gait { .. } | // This isn't really simple..
          Self::GlowTile { .. } |
          Self::Gnawer { .. } |
          Self::GobbleVerminClass { .. } |
          Self::GrassTrample { .. } |
          Self::GravitateBodySize { .. } |
          Self::Grazer { .. } |
          Self::Habit { ..}|
          Self::HabitNumber { .. } |
          Self::Homeotherm { .. } |
          Self::ItemCorpseQuality { .. }|
          Self::LairCharacteristic { .. }|
          Self::LairHunterSpeech { .. }|
          Self::LowLightVision {.. }|
          Self::MannerismArms { .. }|
          Self::MannerismCheek { .. }|
          Self::MannerismEar { .. }|
          Self::MannerismEyes { .. }|
          Self::MannerismFeet { .. }|
          Self::MannerismHair { .. }|
          Self::MannerismKnuckles {.. }|
          Self::MannerismLips { .. }|
          Self::MannerismHands { .. }|
          Self::MannerismHead { .. }|
          Self::MannerismLeg { .. }|
          Self::MannerismMouth { .. }|
          Self::MannerismNose { .. }|
          Self::MannerismTongue { .. }|
          Self::MannerismNails { .. }|
          Self::ModValue { .. }|
          Self::OdorLevel { .. }|
          Self::OdorString { .. }|
          Self::PenetratePower { .. }|
          Self::PetValue { .. }|
          Self::PetValueDivisor { .. }|
          Self::PopulationRatio { .. }|
          Self::ProneToRage { .. }|
          Self::RemainsColor { .. }|
          Self::SkillLearnRates { .. }|
          Self::SlainSpeech { .. }|
          Self::SoldierAltTile { .. }|
          Self::SoldierTile { .. }|
          Self::Tile { .. }|
          Self::TradeCapacity { .. }|
          Self::ViewRange { .. }|
          Self::Webber { .. } => {
                tracing::trace!("get_complexity: {self:?} is 'Simple'");
                TagComplexity::Simple
          }
          _ => {
                tracing::trace!("get_complexity: {self:?} is 'None'");
                TagComplexity::None
          }
        }
    }

    fn parse(key: &str, value: &str) -> Option<Self>
    where
        Self: Sized,
    {
        let Some(token) = CASTE_TOKENS.get(key) else {
            tracing::error!("parse_token: unknown token: {}", key);
            return None;
        };

        match token.get_complexity() {
            TagComplexity::None => Some(token.clone()),
            TagComplexity::Simple => {
                // All of these tokens have a pattern of `key:value` so we can parse `value` as appropriate
                // We just pass this off to the token's `simple_parse` method to handle the parsing
                token.parse_simple_token(value)
            }
            TagComplexity::Complex => {
                // These tokens have a variable number of arguments, so we need to parse them differently
                // We pass this off to the token's `complex_parse` method to handle the parsing
                let values = value.split(':').collect::<Vec<&str>>();
                token.parse_complex_token(&values)
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    /// Parses a token with the simple pattern of `key:value`
    /// This is used for tokens that have a single value, such as `BABY:1`
    /// This is also used for tokens that have a single value with a single argument, such as `BODY:BODY_HUMANOID`
    ///
    /// # Arguments
    ///
    /// * `token` - The token to parse
    /// * `value` - The value of the token (the part after the `:`)
    ///
    /// # Returns
    ///
    /// * `Some(Self)` - The parsed token
    /// * `None` - The token could not be parsed
    fn parse_simple_token(&self, value: &str) -> Option<Self> {
        match self {
            Self::AltTile { .. } => {
                let tile = String::from(value);
                Some(Self::AltTile { tile })
            }
            Self::Baby { .. } => {
                let age: u32 = value.parse().ok().unwrap_or_default();
                Some(Self::Baby { age })
            }
            Self::BeachFrequency { .. } => {
                let frequency: u32 = value.parse().ok().unwrap_or_default();
                Some(Self::BeachFrequency { frequency })
            }
            Self::BodyGloss { .. } => {
                let gloss = String::from(value);
                Some(Self::BodyGloss { gloss })
            }
            Self::BodyPartAddType { .. } => {
                let body_part_type = String::from(value);
                Some(Self::BodyPartAddType { body_part_type })
            }
            Self::BodyPartRemoveType { .. } => {
                let body_part_type = String::from(value);
                Some(Self::BodyPartRemoveType { body_part_type })
            }
            Self::BuildingDestroyer { .. } => {
                let building_destroyer: u32 = value.parse().ok().unwrap_or_default();
                let door_and_furniture_focused = building_destroyer == 1;
                Some(Self::BuildingDestroyer {
                    door_and_furniture_focused,
                })
            }
            Self::CanDoInteraction { .. } => {
                let interaction = String::from(value);
                Some(Self::CanDoInteraction { interaction })
            }
            Self::ChangeBodySizePercent { .. } => {
                let percent: u32 = value.parse().ok().unwrap_or_default();
                Some(Self::ChangeBodySizePercent { percent })
            }
            Self::Child { .. } => {
                let age: u32 = value.parse().ok().unwrap_or_default();
                Some(Self::Child { age })
            }
            Self::CreatureClass { .. } => {
                let class = String::from(value);
                Some(Self::CreatureClass { class })
            }
            Self::CreatureVariationAddTag { .. } => {
                let tag = String::from(value);
                Some(Self::CreatureVariationAddTag { tag })
            }
            Self::CreatureVariationRemoveTag { .. } => {
                let tag = String::from(value);
                Some(Self::CreatureVariationRemoveTag { tag })
            }
            Self::Description { .. } => {
                let description = String::from(value);
                Some(Self::Description { description })
            }
            Self::Difficulty { .. } => {
                let Ok(difficulty) = value.parse::<u32>() else {
                    tracing::warn!("parse_simple_token: Cannot parse difficulty: {}", value);
                    return None;
                };
                Some(Self::Difficulty { difficulty })
            }
            Self::ExtraButcherObjectShape { .. } => {
                let shape = String::from(value);
                Some(Self::ExtraButcherObjectShape { shape })
            }
            Self::EggSize { .. } => {
                let size: u32 = value.parse().ok().unwrap_or_default();
                Some(Self::EggSize { size })
            }
            Self::Extract { .. } => {
                let material = String::from(value);
                Some(Self::Extract { material })
            }
            Self::FixedTemp { .. } => {
                let temperature: i32 = value.parse().ok().unwrap_or_default();
                Some(Self::FixedTemp { temperature })
            }
            Self::Gait { .. } => {
                let gait = String::from(value);
                Some(Self::Gait { gait })
            }
            Self::GlowTile { .. } => {
                let tile = String::from(value);
                Some(Self::GlowTile { tile })
            }
            Self::Gnawer { .. } => {
                let verb = String::from(value);
                Some(Self::Gnawer { verb })
            }
            Self::GobbleVerminClass { .. } => {
                let vermin_class = String::from(value);
                Some(Self::GobbleVerminClass { vermin_class })
            }
            Self::GrassTrample { .. } => {
                let trample = value.parse::<u32>().ok()?;
                Some(Self::GrassTrample { trample })
            }
            Self::GravitateBodySize { .. } => {
                let target: u32 = value.parse().ok().unwrap_or_default();
                Some(Self::GravitateBodySize { target })
            }
            Self::Grazer { .. } => {
                let grazer: u32 = value.parse().ok().unwrap_or_default();
                Some(Self::Grazer { grazer })
            }
            Self::Habit { .. } => {
                let habit = String::from(value);
                Some(Self::Habit { habit })
            }
            Self::HabitNumber { .. } => {
                let number: u32 = value.parse().ok().unwrap_or_default();
                Some(Self::HabitNumber { number })
            }
            Self::Homeotherm { .. } => {
                let temperature: u32 = value.parse().ok().unwrap_or_default();
                Some(Self::Homeotherm {
                    temperature: Some(temperature),
                })
            }
            Self::ItemCorpseQuality { .. } => {
                let quality = value.parse::<u32>().ok()?;
                Some(Self::ItemCorpseQuality { quality })
            }
            Self::LairCharacteristic { .. } => {
                let characteristic = String::from(value);
                Some(Self::LairCharacteristic { characteristic })
            }
            Self::LairHunterSpeech { .. } => {
                let speech_file = String::from(value);
                Some(Self::LairHunterSpeech { speech_file })
            }
            Self::LowLightVision { .. } => {
                let vision = value.parse::<u32>().ok()?;
                Some(Self::LowLightVision { vision })
            }
            Self::MannerismArms { .. } => {
                let arms = String::from(value);
                Some(Self::MannerismArms { arms })
            }
            Self::MannerismCheek { .. } => {
                let cheek = String::from(value);
                Some(Self::MannerismCheek { cheek })
            }
            Self::MannerismEar { .. } => {
                let ear = String::from(value);
                Some(Self::MannerismEar { ear })
            }
            Self::MannerismEyes { .. } => {
                let eyes = String::from(value);
                Some(Self::MannerismEyes { eyes })
            }
            Self::MannerismFeet { .. } => {
                let feet = String::from(value);
                Some(Self::MannerismFeet { feet })
            }
            Self::MannerismHair { .. } => {
                let hair = String::from(value);
                Some(Self::MannerismHair { hair })
            }
            Self::MannerismKnuckles { .. } => {
                let knuckles = String::from(value);
                Some(Self::MannerismKnuckles { knuckles })
            }
            Self::MannerismLips { .. } => {
                let lips = String::from(value);
                Some(Self::MannerismLips { lips })
            }
            Self::MannerismHands { .. } => {
                let hands = String::from(value);
                Some(Self::MannerismHands { hands })
            }
            Self::MannerismHead { .. } => {
                let head = String::from(value);
                Some(Self::MannerismHead { head })
            }
            Self::MannerismLeg { .. } => {
                let leg = String::from(value);
                Some(Self::MannerismLeg { leg })
            }
            Self::MannerismMouth { .. } => {
                let mouth = String::from(value);
                Some(Self::MannerismMouth { mouth })
            }
            Self::MannerismNose { .. } => {
                let nose = String::from(value);
                Some(Self::MannerismNose { nose })
            }
            Self::MannerismTongue { .. } => {
                let tongue = String::from(value);
                Some(Self::MannerismTongue { tongue })
            }
            Self::MannerismNails { .. } => {
                let nails = String::from(value);
                Some(Self::MannerismNails { nails })
            }
            Self::ModValue { .. } => {
                let value = String::from(value);
                Some(Self::ModValue { value })
            }
            Self::OdorLevel { .. } => {
                let odor_level: u32 = value.parse().ok().unwrap_or_default();
                Some(Self::OdorLevel { odor_level })
            }
            Self::OdorString { .. } => {
                let odor_string = String::from(value);
                Some(Self::OdorString { odor_string })
            }
            Self::PenetratePower { .. } => {
                let penetrate_power: u32 = value.parse().ok().unwrap_or_default();
                Some(Self::PenetratePower { penetrate_power })
            }
            Self::PetValue { .. } => {
                let pet_value: u32 = value.parse().ok().unwrap_or_default();
                Some(Self::PetValue { pet_value })
            }
            Self::PetValueDivisor { .. } => {
                let Ok(divisor) = value.parse::<u32>() else {
                    tracing::warn!(
                        "parse_simple_token: Cannot parse pet value divisor: {}",
                        value
                    );
                    return None;
                };
                Some(Self::PetValueDivisor { divisor })
            }
            Self::PopulationRatio { .. } => {
                let Ok(pop_ratio) = value.parse::<u32>() else {
                    tracing::warn!(
                        "parse_simple_token: Cannot parse population ratio: {}",
                        value
                    );
                    return None;
                };
                Some(Self::PopulationRatio { pop_ratio })
            }
            Self::ProneToRage { .. } => {
                let Ok(rage_chance) = value.parse::<u32>() else {
                    tracing::warn!("parse_simple_token: Cannot parse rage chance: {}", value);
                    return None;
                };
                Some(Self::ProneToRage { rage_chance })
            }
            Self::RemainsColor { .. } => {
                let remains_color = String::from(value);
                Some(Self::RemainsColor { remains_color })
            }
            Self::SkillLearnRates { .. } => {
                let Ok(rate) = value.parse::<u32>() else {
                    tracing::warn!(
                        "parse_simple_token: Cannot parse skill learn rate: {}",
                        value
                    );
                    return None;
                };
                Some(Self::SkillLearnRates { rate })
            }
            Self::SlainSpeech { .. } => {
                let speech_file = String::from(value);
                Some(Self::SlainSpeech { speech_file })
            }
            Self::SoldierAltTile { .. } => {
                let tile = String::from(value);
                Some(Self::SoldierAltTile { tile })
            }
            Self::SoldierTile { .. } => {
                let tile = String::from(value);
                Some(Self::SoldierTile { tile })
            }
            Self::Tile { .. } => {
                let tile = String::from(value);
                Some(Self::Tile { tile })
            }
            Self::TradeCapacity { .. } => {
                let Ok(capacity) = value.parse::<u32>() else {
                    tracing::warn!("parse_simple_token: Cannot parse trade capacity: {}", value);
                    return None;
                };
                Some(Self::TradeCapacity { capacity })
            }
            Self::ViewRange { .. } => {
                let Ok(view_range) = value.parse::<u32>() else {
                    tracing::warn!("parse_simple_token: Cannot parse view range: {}", value);
                    return None;
                };
                Some(Self::ViewRange { view_range })
            }
            Self::Webber { .. } => {
                let material = String::from(value);
                Some(Self::Webber { material })
            }
            _ => {
                tracing::error!("parse_simple_token: Cannot parse token (not simple): {self:?}");
                None
            }
        }
    }

    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    /// Parses a token with the complex pattern of `key:value1:value2:value3` or similar. Each complex token
    /// has a different number of arguments, so we need to parse them differently.
    ///
    /// # Arguments
    ///
    /// * `token` - The token to parse
    /// * `value` - The value of the token (the part after the first `:`)
    ///
    /// # Returns
    ///
    /// * `Some(Self)` - The parsed token
    /// * `None` - The token could not be parsed
    fn parse_complex_token(&self, values: &[&str]) -> Option<Self> {
        match self {
            Self::ApplyCreatureVariation { .. } => {
                // check if there are enough arguments
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse ApplyCreatureVariation: not enough arguments: {}/2 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let id = (*values.first().unwrap_or(&"")).to_string();
                let args = (*values.get(1..).unwrap_or_default())
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>();
                Some(Self::ApplyCreatureVariation { id, args })
            }
            Self::Attack { .. } => {
                // Appears as `ATTACK:NAME:BODYPART:BY_CATEGORY:HORN`
                // check if there are enough arguments
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse Attack: not enough arguments: {}/2 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let name = (*values.first().unwrap_or(&"")).to_string();
                let body_part = (*values.get(1..).unwrap_or_default()).join(":");
                Some(Self::Attack { name, body_part })
            }
            Self::AttackTrigger { .. } => {
                // Appears as `ATTACK_TRIGGER:0:1:2` for population, exported_wealth and created_wealth
                // check if there are enough arguments
                if values.len() < 3 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse AttackTrigger: not enough arguments: {}/3 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let Ok(population) = (*values.first().unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse AttackTrigger: population: {values:?}"
                    );
                    return None;
                };
                let Ok(exported_wealth) = (*values.get(1).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse AttackTrigger: exported wealth: {values:?}"
                );
                    return None;
                };
                let Ok(created_wealth) = (*values.get(2).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse AttackTrigger: created wealth: {values:?}"
                );
                    return None;
                };
                Some(Self::AttackTrigger {
                    population,
                    exported_wealth,
                    created_wealth,
                })
            }
            Self::BabyName { .. } => {
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse BabyName: not enough arguments: {}/2 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let singular = (*values.first().unwrap_or(&"")).to_string();
                let plural = (*values.get(1..).unwrap_or_default()).join(":");
                Some(Self::BabyName { singular, plural })
            }
            Self::Blood { .. } => {
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse Blood: not enough arguments: {}/2 '{values:?}'",
                    values.len(),
                );
                    return None;
                }
                let material = (*values.first().unwrap_or(&"")).to_string();
                let state = (*values.get(1..).unwrap_or_default()).join(":");
                Some(Self::Blood { material, state })
            }
            Self::Body { .. } => {
                let body_parts = values
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect();
                Some(Self::Body { body_parts })
            }
            Self::BodyAppearanceModifier { .. } => {
                // Arguments become a string (attribute) and 7 i32s, separated by `:`
                if values.len() < 8 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse BodyAppearanceModifier: not enough arguments: {}/8 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // Parse first argument as `String`
                let attribute = (*values.first().unwrap_or(&"")).to_string();
                let Ok(lowest) = (*values.get(1).unwrap_or(&"")).parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lowest: {values:?}"
                );
                    return None;
                };
                let Ok(lower) = (*values.get(2).unwrap_or(&"")).parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lower: {values:?}"
                );
                    return None;
                };
                let Ok(lower_median) = (*values.get(3).unwrap_or(&"")).parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lower medium: {values:?}"
                );
                    return None;
                };
                let Ok(median) = (*values.get(4).unwrap_or(&"")).parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: medium: {values:?}"
                );
                    return None;
                };
                let Ok(upper_median) = (*values.get(5).unwrap_or(&"")).parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: upper medium: {values:?}"
                );
                    return None;
                };
                let Ok(upper) = (*values.get(6).unwrap_or(&"")).parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: upper: {values:?}"
                );
                    return None;
                };
                let Ok(highest) = (*values.get(7).unwrap_or(&"")).parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: highest: {values:?}"
                );
                    return None;
                };
                Some(Self::BodyAppearanceModifier {
                    attribute,
                    values: [
                        lowest,
                        lower,
                        lower_median,
                        median,
                        upper_median,
                        upper,
                        highest,
                    ],
                })
            }
            Self::BodyPartAppearanceModifier { .. } => {
                // Arguments become a string (attribute) and 7 i32s, separated by `:`
                if values.len() < 8 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse BodyPartAppearanceModifier: not enough arguments: {}/8 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // Parse first argument as `String`
                let quality = (*values.first().unwrap_or(&"")).to_string();
                let Ok(lowest) = (*values.get(1).unwrap_or(&"")).parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lowest: {values:?}"
                );
                    return None;
                };
                let Ok(lower) = (*values.get(2).unwrap_or(&"")).parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lower: {values:?}"
                );
                    return None;
                };
                let Ok(lower_median) = (*values.get(3).unwrap_or(&"")).parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lower medium: {values:?}"
                );
                    return None;
                };
                let Ok(median) = (*values.get(4).unwrap_or(&"")).parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: medium: {values:?}"
                );
                    return None;
                };
                let Ok(upper_median) = (*values.get(5).unwrap_or(&"")).parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: upper medium: {values:?}"
                );
                    return None;
                };
                let Ok(upper) = (*values.get(6).unwrap_or(&"")).parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: upper: {values:?}"
                );
                    return None;
                };
                let Ok(highest) = (*values.get(7).unwrap_or(&"")).parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: highest: {values:?}"
                );
                    return None;
                };
                Some(Self::BodyPartAppearanceModifier {
                    quality,
                    spread: [
                        lowest,
                        lower,
                        lower_median,
                        median,
                        upper_median,
                        upper,
                        highest,
                    ],
                })
            }
            Self::BodyDetailPlan { .. } => {
                let body_plan = (*values.first().unwrap_or(&"")).to_string();
                let arguments = (*values.get(1..).unwrap_or_default())
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>();
                Some(Self::BodyDetailPlan {
                    body_plan,
                    arguments,
                })
            }
            Self::BodySize { .. } => {
                // Body size is [YEAR:DAYS:SIZE], all are u32s
                if values.len() < 3 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse body size: not enough arguments: {}/3 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let Ok(year) = (*values.first().unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!("parse_complex_token: Cannot parse body size: year: {values:?}");
                    return None;
                };
                let Ok(days) = (*values.get(1).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!("parse_complex_token: Cannot parse body size: days: {values:?}");
                    return None;
                };
                let Ok(size) = (*values.get(2).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!("parse_complex_token: Cannot parse body size: size: {values:?}");
                    return None;
                };
                Some(Self::BodySize { year, days, size })
            }
            Self::ChildName { .. } => {
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse ChildName: not enough arguments: {}/2 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let singular = (*values.first().unwrap_or(&"")).to_string();
                let plural = (*values.get(1..).unwrap_or_default()).join(":");
                Some(Self::ChildName { singular, plural })
            }
            Self::ClutchSize { .. } => {
                // Two `u32`s
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse clutch size: not enough arguments: {}/2 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let Ok(min) = (*values.first().unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse clutch size: min: {values:?}"
                    );
                    return None;
                };
                let Ok(max) = (*values.get(1).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse clutch size: max: {values:?}"
                    );
                    return None;
                };
                Some(Self::ClutchSize { min, max })
            }
            Self::Color { .. } => {
                if values.len() < 3 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse color: not enough arguments: {}/3 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let Ok(foreground) = (*values.first().unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse color: foreground: {values:?}"
                    );
                    return None;
                };
                let Ok(background) = (*values.get(1).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse color: background: {values:?}"
                    );
                    return None;
                };
                let Ok(brightness) = (*values.get(2).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse color: brightness: {values:?}"
                    );
                    return None;
                };
                Some(Self::Color {
                    foreground,
                    background,
                    brightness,
                })
            }
            Self::EggMaterial { .. } => {
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse egg material: not enough arguments: {}/2 '{values:?}'",
                    values.len(),
                );
                    return None;
                }
                // Take the last `String` as the `state` and the rest as the `material`
                let state = (*values.last().unwrap_or(&"")).to_string();
                let material = (*values.get(..values.len() - 1).unwrap_or_default()).join(":");
                Some(Self::EggMaterial { material, state })
            }
            Self::ExtraButcherObject { .. } => {
                // `String` and `Vec<String>`
                let object_type = (*values.first().unwrap_or(&"")).to_string();
                let arguments = (*values.get(1..).unwrap_or_default())
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>();
                Some(Self::ExtraButcherObject {
                    object_type,
                    arguments,
                })
            }
            Self::ExtraButcherObjectItem { .. } => {
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse extra butcher object item: not enough arguments: {}/2 '{values:?}'",
                    values.len(),
                );
                    return None;
                }
                // Two strings
                let item = (*values.first().unwrap_or(&"")).to_string();
                let material = (*values.get(1..).unwrap_or_default()).join(":");
                Some(Self::ExtraButcherObjectItem { item, material })
            }
            Self::GeneralMaterialForceMultiplier { .. } => {
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse general material force multiplier: not enough arguments: {}/2 '{values:?}'",
                    values.len(),
                );
                    return None;
                }
                // Two `u32`s
                let Ok(value_a) = (*values.first().unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse general material force multiplier: {values:?}"
                );
                    return None;
                };
                let Ok(value_b) = (*values.get(1).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse general material force multiplier: {values:?}"
                );
                    return None;
                };
                Some(Self::GeneralMaterialForceMultiplier { value_a, value_b })
            }
            Self::GlowColor { .. } => {
                // Arguments become 3 `u32`s, separated by `:`
                if values.len() < 3 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse glow color: not enough arguments: {}/3 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let Ok(foreground) = (*values.first().unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse glow color: foreground: {values:?}"
                    );
                    return None;
                };
                let Ok(background) = (*values.get(1).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse glow color: background: {values:?}"
                    );
                    return None;
                };
                let Ok(brightness) = (*values.get(2).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse glow color: brightness: {values:?}"
                    );
                    return None;
                };
                Some(Self::GlowColor {
                    foreground,
                    background,
                    brightness,
                })
            }
            Self::GobbleVerminCreature { .. } => {
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse gobble vermin creature: not enough arguments: {}/2 '{values:?}'",
                    values.len(),
                );
                    return None;
                }
                // Two strings
                let vermin_creature = (*values.first().unwrap_or(&"")).to_string();
                let vermin_caste = (*values.get(1..).unwrap_or_default()).join(":");
                Some(Self::GobbleVerminCreature {
                    vermin_creature,
                    vermin_caste,
                })
            }
            Self::InteractionDetail { .. } => {
                let args = values
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>();
                Some(Self::InteractionDetail { args })
            }
            Self::ItemCorpse { .. } => {
                // Two strings
                let item = (*values.first().unwrap_or(&"")).to_string();
                let material = (*values.get(1..).unwrap_or_default()).join(":");
                Some(Self::ItemCorpse { item, material })
            }
            Self::Lair { .. } => {
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse lair: not enough arguments: {}/2 '{values:?}'",
                    values.len(),
                );
                    return None;
                }
                // `String` and `u32`
                let lair = (*values.first().unwrap_or(&"")).to_string();
                let Ok(probability) = (*values.get(1).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!("parse_complex_token: Cannot parse lair proability: {values:?}");
                    return None;
                };
                Some(Self::Lair { lair, probability })
            }
            Self::LaysUnusualEggs { .. } => {
                // Two strings
                let item = (*values.first().unwrap_or(&"")).to_string();
                let material = (*values.get(1..).unwrap_or_default()).join(":");
                Some(Self::LaysUnusualEggs { item, material })
            }
            Self::Ligaments { .. } => {
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse ligaments: not enough arguments: {}/2 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // Grab `healing_rate` from the end of `value`
                let Ok(healing_rate) = (*values.last().unwrap_or(&"")).parse() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse ligaments: healing rate: {values:?}"
                    );
                    return None;
                };
                // The rest of the arguments are the `material`
                let material = (*values.get(..values.len() - 1).unwrap_or_default()).join(":");
                Some(Self::Ligaments {
                    material,
                    healing_rate,
                })
            }
            Self::LitterSize { .. } => {
                // Two `u32`s
                let Ok(min) = (*values.first().unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse litter size: min: {values:?}"
                    );
                    return None;
                };
                let Ok(max) = (*values.get(1).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse litter size: max: {values:?}"
                    );
                    return None;
                };
                Some(Self::LitterSize { min, max })
            }
            Self::MannerismFingers { .. } => {
                let finger = (*values.first().unwrap_or(&"")).to_string();
                let fingers = (*values.get(1..).unwrap_or_default()).join(":");
                Some(Self::MannerismFingers { finger, fingers })
            }
            Self::MaxAge { .. } => {
                // Two `u32`s
                let Ok(min) = (*values.first().unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!("parse_complex_token: Cannot parse max age: min: {values:?}");
                    return None;
                };
                let Ok(max) = (*values.get(1).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!("parse_complex_token: Cannot parse max age: max: {values:?}");
                    return None;
                };
                Some(Self::MaxAge { min, max })
            }
            Self::MentalAttributeCapPercentage { .. } => {
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute cap percentage: not enough arguments: {}/2 '{values:?}'",
                    values.len(),
                );
                    return None;
                }
                let attribute = (*values.first().unwrap_or(&"")).to_string();
                let Ok(percentage) = (*values.get(1).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute cap percentage: {values:?}"
                );
                    return None;
                };
                Some(Self::MentalAttributeCapPercentage {
                    attribute,
                    percentage,
                })
            }
            Self::MentalAttributeRange { .. } => {
                // Arguments become a `String` and 7 `u32`s
                if values.len() < 8 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: not enough arguments: {}/8 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // Parse first argument as `String`
                let attribute = (*values.first().unwrap_or(&"")).to_string();
                let Ok(lowest) = (*values.get(1).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lowest: {values:?}"
                );
                    return None;
                };
                let Ok(lower) = (*values.get(2).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lower: {values:?}"
                );
                    return None;
                };
                let Ok(lower_median) = (*values.get(3).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lower medium: {values:?}"
                );
                    return None;
                };
                let Ok(median) = (*values.get(4).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: medium: {values:?}"
                );
                    return None;
                };
                let Ok(upper_median) = (*values.get(5).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: upper medium: {values:?}"
                );
                    return None;
                };
                let Ok(upper) = (*values.get(6).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: upper: {values:?}"
                );
                    return None;
                };
                let Ok(highest) = (*values.get(7).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: highest: {values:?}"
                );
                    return None;
                };
                Some(Self::MentalAttributeRange {
                    attribute,
                    ranges: [
                        lowest,
                        lower,
                        lower_median,
                        median,
                        upper_median,
                        upper,
                        highest,
                    ],
                })
            }
            Self::MentalAttributeRate { .. } => {
                // Arguments become a `String` and 4 `u32`s
                if values.len() < 5 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute rate: not enough arguments: {}/5 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // Parse first argument as `String`
                let attribute = (*values.first().unwrap_or(&"")).to_string();
                let Ok(improvement_cost) = (*values.get(1).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute rate: improvement cost: {values:?}"
                );
                    return None;
                };
                let Ok(decay_rate_unused) = (*values.get(2).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute rate: decay rate unused: {values:?}"
                );
                    return None;
                };
                let Ok(decay_rate_rusty) = (*values.get(3).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute rate: decay rate rusty: {values:?}"
                );
                    return None;
                };
                let Ok(decay_rate_demotion) = (*values.get(4).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute rate: decay rate demotion: {values:?}"
                );
                    return None;
                };
                Some(Self::MentalAttributeRate {
                    attribute,
                    improvement_cost,
                    decay_rate_unused,
                    decay_rate_rusty,
                    decay_rate_demotion,
                })
            }
            Self::Milkable { .. } => {
                // Arguments become a `String` and `u32`
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse milkable: not enough arguments: {}/2 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // Frequency is the last argument, parsed as `u32`
                let Ok(frequency) = (*values.last().unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse milkable: frequency: {values:?}"
                    );
                    return None;
                };
                // Material is the rest of the arguments, joined as a `String`
                let material = (*values.get(..values.len() - 1).unwrap_or_default()).join(":");
                Some(Self::Milkable {
                    material,
                    frequency,
                })
            }
            Self::Name { .. } => {
                if values.len() < 3 {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse name: not enough arguments: {}/3 '{:?}'",
                        values.len(),
                        values
                    );
                    return None;
                }
                let singular = (*values.first().unwrap_or(&"")).to_string();
                let plural = (*values.get(1).unwrap_or(&"")).to_string();
                let adjective = (*values.get(2..).unwrap_or_default()).join(":");
                Some(Self::Name {
                    singular,
                    plural,
                    adjective,
                })
            }
            Self::NaturalSkill { .. } => {
                // Grab `level` from the end of `value`
                let Ok(level) = (*values.last().unwrap_or(&"")).parse() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse natural skill: level: {values:?}"
                    );
                    return None;
                };
                // The rest of the arguments are the `skill`
                let skill = (*values.get(..values.len() - 1).unwrap_or_default()).join(":");
                Some(Self::NaturalSkill { skill, level })
            }
            Self::Personality { .. } => {
                // Check if there are enough arguments to parse
                if values.len() < 4 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse personality: not enough arguments: {}/4 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // Parse first argument as `String`
                let personality_trait = (*values.first().unwrap_or(&"")).to_string();
                let Ok(low) = (*values.get(1).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse personality: low: {values:?}"
                    );
                    return None;
                };
                let Ok(median) = (*values.get(2).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse personality: median: {values:?}"
                    );
                    return None;
                };
                let Ok(high) = (*values.get(3).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse personality: high: {values:?}"
                    );
                    return None;
                };
                Some(Self::Personality {
                    personality_trait,
                    low,
                    median,
                    high,
                })
            }
            Self::PhysicalAttributeCapPercentage { .. } => {
                // Arguments become a `String` and 1 `u32`s
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute cap percentage: not enough arguments: {}/2 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // Parse first argument as `String`
                let attribute = (*values.first().unwrap_or(&"")).to_string();
                let Ok(percentage) = (*values.get(1).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute cap percentage: {values:?}"
                );
                    return None;
                };
                Some(Self::PhysicalAttributeCapPercentage {
                    attribute,
                    percentage,
                })
            }
            Self::PhysicalAttributeRange { .. } => {
                // Arguments become a `String` and 7 `u32`s
                if values.len() < 8 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute range: not enough arguments: {}/8 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // Parse first argument as `String`
                let attribute = (*values.first().unwrap_or(&"")).to_string();
                let Ok(lowest) = (*values.get(1).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute range: lowest: {values:?}"
                );
                    return None;
                };
                let Ok(lower) = (*values.get(2).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute range: lower: {values:?}"
                );
                    return None;
                };
                let Ok(lower_median) = (*values.get(3).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute range: lower medium: {values:?}"
                );
                    return None;
                };
                let Ok(median) = (*values.get(4).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute range: medium: {values:?}"
                );
                    return None;
                };
                let Ok(upper_median) = (*values.get(5).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute range: upper medium: {values:?}"
                );
                    return None;
                };
                let Ok(upper) = (*values.get(6).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute range: upper: {values:?}"
                );
                    return None;
                };
                let Ok(highest) = (*values.get(7).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute range: highest: {values:?}"
                );
                    return None;
                };
                Some(Self::PhysicalAttributeRange {
                    attribute,
                    ranges: [
                        lowest,
                        lower,
                        lower_median,
                        median,
                        upper_median,
                        upper,
                        highest,
                    ],
                })
            }
            Self::PhysicalAttributeRate { .. } => {
                // Arguments become a `String` and 4 `u32`s
                if values.len() < 5 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute rate: not enough arguments: {}/5 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // Parse first argument as `String`
                let attribute = (*values.first().unwrap_or(&"")).to_string();
                let Ok(improvement_cost) = (*values.get(1).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute rate: improvement cost: {values:?}"
                );
                    return None;
                };
                let Ok(decay_rate_unused) = (*values.get(2).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute rate: decay rate unused: {values:?}"
                );
                    return None;
                };
                let Ok(decay_rate_rusty) = (*values.get(3).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute rate: decay rate rusty: {values:?}"
                );
                    return None;
                };
                let Ok(decay_rate_demotion) = (*values.get(4).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute rate: decay rate demotion: {values:?}"
                );
                    return None;
                };
                Some(Self::PhysicalAttributeRate {
                    attribute,
                    improvement_cost,
                    decay_rate_unused,
                    decay_rate_rusty,
                    decay_rate_demotion,
                })
            }
            Self::ProfessionName { .. } => {
                // Arguments become a singular name, plural name, and adjective, separated by `:`
                if values.len() < 3 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse profession name: not enough arguments: {}/3 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let profession = (*values.first().unwrap_or(&"")).to_string();
                let singular = (*values.get(1).unwrap_or(&"")).to_string();
                let plural = (*values.get(2).unwrap_or(&"")).to_string();
                Some(Self::ProfessionName {
                    profession,
                    singular,
                    plural,
                })
            }
            Self::Pus { .. } => {
                // Grab `state` from the end
                let state = (*values.last().unwrap_or(&"")).to_string();
                // Set `material` to `simple_value` + the remains of `value`
                let material = (*values.get(..values.len() - 1).unwrap_or_default()).join(":");
                Some(Self::Pus { material, state })
            }
            Self::RelativeSize { .. } => {
                // Appears as `RELATIVE_SIZE:SomeBodyPartSelector:SomeBodyPart:100`
                // check if there are enough arguments
                if values.len() < 3 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse RelativeSize: not enough arguments: {}/3 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // `relative_size` is the last argument, parsed as `u32`
                let Ok(relative_size) = (*values.last().unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse RelativeSize: relative size: {values:?}"
                    );
                    return None;
                };
                let body_part_selector = (*values.first().unwrap_or(&"")).to_string();
                let body_part = (*values.get(1..values.len() - 1).unwrap_or_default()).join(":");
                Some(Self::RelativeSize {
                    body_part_selector,
                    body_part,
                    relative_size,
                })
            }
            Self::Remains { .. } => {
                // Appears as `REMAINS:SomeRemain:SomeRemains`
                // check if there are enough arguments
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse Remains: not enough arguments: {}/2 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let singular = (*values.first().unwrap_or(&"")).to_string();
                let plural = (*values.get(1..).unwrap_or_default()).join(":");
                Some(Self::Remains { singular, plural })
            }
            Self::RetractIntoBodyPart { .. } => {
                // check if there are enough arguments
                if values.len() < 6 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse RetractIntoBodyPart: not enough arguments: {}/6 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // We grab the strings from the end first
                let third_person_cancel = (*values.last().unwrap_or(&"")).to_string();
                let second_person_cancel =
                    (*values.get(values.len() - 2).unwrap_or(&"")).to_string();
                let third_person = (*values.get(values.len() - 3).unwrap_or(&"")).to_string();
                let second_person = (*values.get(values.len() - 4).unwrap_or(&"")).to_string();
                // Then the body_part_selector
                let body_part_selector = (*values.first().unwrap_or(&"")).to_string();
                // And finally the body_part
                let body_part = (*values.get(1..values.len() - 4).unwrap_or_default()).join(":");
                Some(Self::RetractIntoBodyPart {
                    body_part_selector,
                    body_part,
                    second_person,
                    third_person,
                    second_person_cancel,
                    third_person_cancel,
                })
            }
            Self::RootAround { .. } => {
                if values.len() < 4 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse RootAround: not enough arguments: {}/4 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let third_person_verb = (*values.last().unwrap_or(&"")).to_string();
                let second_person_verb = (*values.get(values.len() - 2).unwrap_or(&"")).to_string();
                let body_part_selector = (*values.first().unwrap_or(&"")).to_string();
                let body_part = (*values.get(1..values.len() - 2).unwrap_or_default()).join(":");
                Some(Self::RootAround {
                    body_part_selector,
                    body_part,
                    second_person_verb,
                    third_person_verb,
                })
            }
            Self::Secretion { .. } => {
                if values.len() < 6 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse Secretion: not enough arguments: {}/6 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let material_token = (*values.first().unwrap_or(&"")).to_string();
                let material_state = (*values.first().unwrap_or(&"")).to_string();
                let body_part_selector = (*values.first().unwrap_or(&"")).to_string();
                // Grab from the end
                let trigger = (*values.last().unwrap_or(&"")).to_string();
                let tissue_layer = (*values.get(values.len() - 2).unwrap_or(&"")).to_string();
                let body_part = (*values.get(1..values.len() - 2).unwrap_or_default()).join(":");
                Some(Self::Secretion {
                    material_token,
                    material_state,
                    body_part_selector,
                    body_part,
                    tissue_layer,
                    trigger,
                })
            }
            Self::SenseCreatureClass { .. } => {
                // Appears as `SENSE_CREATURE_CLASS:SomeCreatureClass:SomeTile:0:0:0`
                if values.len() < 5 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse SenseCreatureClass: not enough arguments: {}/5 '{values:?}'",
                    values.len(),
                );
                    return None;
                }
                let creature_class = (*values.first().unwrap_or(&"")).to_string();
                let tile = (*values.get(1).unwrap_or(&"")).to_string();
                let Ok(foreground) = (*values.get(2).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse SenseCreatureClass: foreground: {values:?}"
                );
                    return None;
                };
                let Ok(background) = (*values.get(3).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse SenseCreatureClass: background: {values:?}"
                );
                    return None;
                };
                let Ok(brightness) = (*values.get(4).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse SenseCreatureClass: brightness: {values:?}"
                );
                    return None;
                };

                Some(Self::SenseCreatureClass {
                    creature_class,
                    tile,
                    foreground,
                    background,
                    brightness,
                })
            }
            Self::SetBodyPartGroup { .. } => {
                // Check if there are enough arguments to parse
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse set body part group: not enough arguments: {}/2 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // Parse first argument as `String`
                let body_part_selector = (*values.first().unwrap_or(&"")).to_string();
                let body_part = (*values.get(1..).unwrap_or_default()).join(":");
                Some(Self::SetBodyPartGroup {
                    body_part_selector,
                    body_part,
                })
            }
            Self::SkillRates { .. } => {
                // Check if there are enough arguments to parse
                if values.len() < 4 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse skill rates: not enough arguments: {}/4 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let Ok(improvement_rate) = (*values.first().unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse skill rates: improvement rate: {values:?}"
                );
                    return None;
                };
                let Ok(decay_rate_unused) = (*values.get(1).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse skill rates: decay rate unused: {values:?}"
                );
                    return None;
                };
                let Ok(decay_rate_rusty) = (*values.get(2).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse skill rates: decay rate rusty: {values:?}"
                );
                    return None;
                };
                let Ok(decay_rate_demotion) = (*values.get(3).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse skill rates: decay rate demotion: {values:?}"
                );
                    return None;
                };
                Some(Self::SkillRates {
                    improvement_rate,
                    decay_rate_unused,
                    decay_rate_rusty,
                    decay_rate_demotion,
                })
            }
            Self::SkillRustRates { .. } => {
                // Check if there are enough arguments to parse
                if values.len() < 3 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse skill rust rates: not enough arguments: {}/3 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let Ok(decay_rate_unused) = (*values.first().unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse skill rust rates: decay rate unused: {values:?}"
                );
                    return None;
                };
                let Ok(decay_rate_rusty) = (*values.get(1).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse skill rust rates: decay rate rusty: {values:?}"
                );
                    return None;
                };
                let Ok(decay_rate_demotion) = (*values.get(2).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse skill rust rates: decay rate demotion: {values:?}"
                );
                    return None;
                };
                Some(Self::SkillRustRates {
                    decay_rate_unused,
                    decay_rate_rusty,
                    decay_rate_demotion,
                })
            }
            Self::Sound { .. } => {
                // Check if there are enough arguments to parse
                if values.len() < 6 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse sound: not enough arguments: {}/6 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let requires_breathing = values.len() == 7;
                let sound_type = (*values.first().unwrap_or(&"")).to_string();
                let Ok(sound_range) = (*values.get(1).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse sound: sound range: {values:?}"
                    );
                    return None;
                };
                let Ok(sound_interval) = (*values.get(2).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse sound: sound interval: {values:?}"
                    );
                    return None;
                };
                let breathing_bump = usize::from(requires_breathing);

                let third_person = (*values.get(3 + breathing_bump).unwrap_or(&"")).to_string();
                let first_person = (*values.get(4 + breathing_bump).unwrap_or(&"")).to_string();
                let out_of_sight = (*values.get(5 + breathing_bump).unwrap_or(&"")).to_string();
                Some(Self::Sound {
                    sound_type,
                    sound_range,
                    sound_interval,
                    requires_breathing,
                    third_person,
                    first_person,
                    out_of_sight,
                })
            }
            Self::SpecificFood { .. } => {
                let Some(food_type) = OBJECT_TOKEN_MAP.get(*values.first().unwrap_or(&"")) else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse SpecificFood: object type: {values:?}"
                    );
                    return None;
                };
                let identifier = (*values.get(1..).unwrap_or_default()).join(":");
                Some(Self::SpecificFood {
                    food_type: food_type.clone(),
                    identifier,
                })
            }
            Self::SyndromeDilutionFactor { .. } => {
                // Check if there are enough arguments to parse
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse syndrome dilution factor: not enough arguments: {}/2 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let syndrome = (*values.first().unwrap_or(&"")).to_string();
                let Ok(percentage) = (*values.get(1).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse syndrome dilution factor: percentage: {values:?}"
                );
                    return None;
                };
                Some(Self::SyndromeDilutionFactor {
                    syndrome,
                    percentage,
                })
            }
            Self::Tendons { .. } => {
                // `material_state` is a `String` and is at the end of `value`
                // `material` is `simple_value` + the remains of `value`
                // Grab `healing_rate` from the end of `value`
                let Ok(healing_rate) = (*values.last().unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse tendons: healing rate: {values:?}"
                    );
                    return None;
                };
                // Set `material` to `simple_value` + the remains of `value`
                let material = (*values.get(..values.len() - 1).unwrap_or_default()).join(":");
                Some(Self::Tendons {
                    material,
                    healing_rate,
                })
            }
            Self::TissueLayer { .. } => {
                let body_part_selector = (*values.first().unwrap_or(&"")).to_string();
                let body_part = (*values.get(1).unwrap_or(&"")).to_string();
                let tissue = (*values.get(2).unwrap_or(&"")).to_string();
                let location = (*values.get(3..).unwrap_or_default()).join(":");
                Some(Self::TissueLayer {
                    body_part_selector,
                    body_part,
                    tissue,
                    location,
                })
            }
            Self::TissueLayerUnder { .. } => {
                let body_part_selector = (*values.first().unwrap_or(&"")).to_string();
                let body_part = (*values.get(1).unwrap_or(&"")).to_string();
                let tissue = (*values.get(2..).unwrap_or_default()).join(":");
                Some(Self::TissueLayerUnder {
                    body_part_selector,
                    body_part,
                    tissue,
                })
            }
            Self::VerminBite { .. } => {
                let Ok(chance) = (*values.first().unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse vermin bite: chance: {values:?}"
                    );
                    return None;
                };
                let verb = (*values.get(1).unwrap_or(&"")).to_string();
                let material = (*values.get(2).unwrap_or(&"")).to_string();
                let material_state = (*values.get(3..).unwrap_or_default()).join(":");
                Some(Self::VerminBite {
                    chance,
                    verb,
                    material,
                    material_state,
                })
            }
            Self::VisionArc { .. } => {
                let Ok(binocular) = (*values.first().unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse vision arc: binocular: {values:?}"
                    );
                    return None;
                };
                let Ok(non_binocular) = (*values.get(1).unwrap_or(&"")).parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse vision arc: non binocular: {values:?}"
                    );
                    return None;
                };
                Some(Self::VisionArc {
                    binocular,
                    non_binocular,
                })
            }
            _ => {
                tracing::error!("parse_complex_token: cannot parse {self:?}");
                None
            }
        }
    }
}
