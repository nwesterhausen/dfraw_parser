use crate::tokens::EntityToken;
use crate::tokens::raw_definitions::ENTITY_TOKENS;
use crate::traits::RawToken;
use std::collections::HashMap;
use std::mem::{Discriminant, discriminant};
use std::sync::OnceLock;

/// Utility impl to provide a reverse lookup from `EntityTag` enum variants back to
/// their original token string (if one exists).
///
/// This mirrors the pattern used by other tag lookup utilities: a lazily-initialized
/// static `HashMap` keyed by the enum `Discriminant` is populated from the
/// existing PHF token map and cached in a `OnceLock` for fast subsequent lookups.
impl RawToken for EntityToken {
    fn get_key(&self) -> Option<&'static str> {
        static REVERSE_MAP: OnceLock<HashMap<Discriminant<EntityToken>, &'static str>> =
            OnceLock::new();

        let map = REVERSE_MAP.get_or_init(|| {
            let mut m = HashMap::new();
            for (key, tag_template) in &ENTITY_TOKENS {
                m.insert(discriminant(tag_template), *key);
            }
            m
        });

        map.get(&discriminant(self)).copied()
    }

    fn to_raw_token(&self) -> String {
        // Lookup the key using the discriminant of 'self'
        let key = match self.get_key() {
            Some(k) => k,
            None => return String::new(),
        };

        match self {
            EntityToken::BiomeSupport { biome, frequency } => {
                format!("[{}:{}:{}]", key, biome, frequency)
            }
            EntityToken::Currency { material, value } => {
                format!("[{}:{}:{}]", key, material, value)
            }
            EntityToken::ArtFacetModifier { modifier, number } => {
                format!("[{}:{}:{}]", key, modifier, number)
            }
            EntityToken::ArtImageElementModifier { item, number } => {
                format!("[{}:{}:{}]", key, item, number)
            }
            EntityToken::ItemImprovementModifier { item, number } => {
                format!("[{}:{}:{}]", key, item, number)
            }
            EntityToken::SelectSymbol { noun, symbol } => {
                format!("[{}:{}:{}]", key, noun, symbol)
            }
            EntityToken::SubselectSymbol { noun, symbol } => {
                format!("[{}:{}:{}]", key, noun, symbol)
            }
            EntityToken::CullSymbol { noun, symbol } => {
                format!("[{}:{}:{}]", key, noun, symbol)
            }
            EntityToken::FriendlyColor { color } => format!("[{key}:{}]", color.as_value()),
            EntityToken::SphereAlignment { sphere, number } => {
                format!("[{}:{}:{}]", key, sphere, number)
            }
            EntityToken::Ethic { behavior, rating } => {
                format!("[{}:{}:{}]", key, behavior, rating)
            }
            EntityToken::Value { value, strength } => format!("[{}:{}:{}]", key, value, strength),
            EntityToken::VariableValue { value, min, max } => {
                format!("[{}:{}:{}:{}]", key, value, min, max)
            }
            EntityToken::Armor { item, chance } => format!("[{}:{}:{}]", key, item, chance),
            EntityToken::Gloves { item, chance } => format!("[{}:{}:{}]", key, item, chance),
            EntityToken::Helm { item, chance } => format!("[{}:{}:{}]", key, item, chance),
            EntityToken::Pants { item, chance } => format!("[{}:{}:{}]", key, item, chance),
            EntityToken::Shoes { item, chance } => format!("[{}:{}:{}]", key, item, chance),
            EntityToken::GemShape { shape } | EntityToken::StoneShape { shape } => {
                format!("[{}:{}]", key, shape)
            }
            EntityToken::TissueStyleMaintainLength { min, max } => {
                format!("[{}:{}:{}]", key, min, max)
            }
            EntityToken::Creature { creature: v }
            | EntityToken::SettlementBiome { biome: v }
            | EntityToken::StartBiome { biome: v }
            | EntityToken::ExclusiveStartBiome { biome: v }
            | EntityToken::DefaultSiteType { site_type: v }
            | EntityToken::LikesSite { site_type: v }
            | EntityToken::ToleratesSite { site_type: v }
            | EntityToken::WorldConstruction { construction: v }
            | EntityToken::PermittedBuilding { building: v }
            | EntityToken::PermittedJob { job: v }
            | EntityToken::PermittedReaction { reaction: v }
            | EntityToken::Translation { language: v }
            | EntityToken::Religion { religion_type: v }
            | EntityToken::ReligionSphere { sphere: v }
            | EntityToken::Position { name: v }
            | EntityToken::SiteVariablePositions { responsibility: v }
            | EntityToken::VariablePositions { responsibility: v }
            | EntityToken::ActiveSeason { season: v }
            | EntityToken::Scholar { scholar_type: v }
            | EntityToken::Ammo { item: v }
            | EntityToken::Digger { item: v }
            | EntityToken::Instrument { item: v }
            | EntityToken::Shield { item: v }
            | EntityToken::SiegeAmmo { item: v }
            | EntityToken::Tool { item: v }
            | EntityToken::Toy { item: v }
            | EntityToken::TrapComponent { item: v }
            | EntityToken::Weapon { item: v }
            | EntityToken::AnimalToken { creature: v }
            | EntityToken::AnimalCasteToken { caste: v }
            | EntityToken::AnimalClass { class: v }
            | EntityToken::AnimalForbiddenClass { class: v }
            | EntityToken::TissueStyle { tissue: v }
            | EntityToken::TissueStylePreferredShaping { style: v } => format!("[{}:{}]", key, v),
            EntityToken::MaxPopNumber { number: v }
            | EntityToken::MaxSitePopNumber { number: v }
            | EntityToken::MaxStartingCivNumber { number: v }
            | EntityToken::Banditry { percentage: v }
            | EntityToken::ProgressTriggerPopulation { level: v }
            | EntityToken::ProgressTriggerProduction { level: v }
            | EntityToken::ProgressTriggerTrade { level: v }
            | EntityToken::ProgressTriggerPopulationSiege { level: v }
            | EntityToken::ProgressTriggerProductionSiege { level: v }
            | EntityToken::ProgressTriggerTradeSiege { level: v } => format!("[{}:{}]", key, v),
            _ => format!("[{}]", key),
        }
    }
}
