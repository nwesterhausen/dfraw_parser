use crate::{
    tokens::{EntityToken, raw_definitions::ENTITY_TOKENS},
    traits::{TagOperations, TokenParser as _},
};

impl TagOperations for EntityToken {
    fn parse(key: &str, value: &str) -> Option<Self> {
        let Some(token) = ENTITY_TOKENS.get(key) else {
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
            EntityToken::AllMainPopsControllable
            | EntityToken::SiteControllable
            | EntityToken::SourceHfid
            | EntityToken::CurrencyByYear
            | EntityToken::WillAcceptTribute
            | EntityToken::Wanderer
            | EntityToken::BeastHunter
            | EntityToken::Scout
            | EntityToken::Mercenary
            | EntityToken::AbuseBodies
            | EntityToken::Ambusher
            | EntityToken::AtPeaceWithWildlife
            | EntityToken::BabySnatcher
            | EntityToken::BuildsOutdoorFortifications
            | EntityToken::BuildsOutdoorTombs
            | EntityToken::DiplomatBodyguards
            | EntityToken::Generated
            | EntityToken::InvadersIgnoreNeutrals
            | EntityToken::ItemThief
            | EntityToken::LocalBanditry
            | EntityToken::MerchantBodyguards
            | EntityToken::MerchantNobility
            | EntityToken::Sieger
            | EntityToken::SiteGuardian
            | EntityToken::Skulking
            | EntityToken::TreeCapDiplomacy
            | EntityToken::LayerLinked
            | EntityToken::GenerateKeyboardInstruments
            | EntityToken::GeneratePercussionInstruments
            | EntityToken::GenerateStringedInstruments
            | EntityToken::GenerateWindInstruments
            | EntityToken::GenerateDanceForms
            | EntityToken::GenerateMusicalForms
            | EntityToken::GeneratePoeticForms
            | EntityToken::SetScholarsOnValuesAndJobs
            | EntityToken::NoArtifactClaims
            | EntityToken::MiningUnderworldDisasters
            | EntityToken::UseAnimalProducts
            | EntityToken::UseAnyPetRace
            | EntityToken::UseCaveAnimals
            | EntityToken::UseEvilAnimals
            | EntityToken::UseEvilPlants
            | EntityToken::UseEvilWood
            | EntityToken::UseGoodAnimals
            | EntityToken::UseGoodPlants
            | EntityToken::UseGoodWood
            | EntityToken::UseMiscProcessedWoodProducts
            | EntityToken::UseNoneExoticPetRace
            | EntityToken::CommonDomesticMount
            | EntityToken::CommonDomesticPackAnimal
            | EntityToken::CommonDomesticPet
            | EntityToken::CommonDomesticPullAnimal
            | EntityToken::RiverProducts
            | EntityToken::OceanProducts
            | EntityToken::IndoorFarming
            | EntityToken::OutdoorFarming
            | EntityToken::IndoorGardens
            | EntityToken::OutdoorGardens
            | EntityToken::IndoorOrchards
            | EntityToken::OutdoorOrchards
            | EntityToken::Clothing
            | EntityToken::SubterraneanClothing
            | EntityToken::EquipmentImprovements
            | EntityToken::ImprovedBows
            | EntityToken::MetalPref
            | EntityToken::StonePref
            | EntityToken::WoodWeapons
            | EntityToken::WoodArmor
            | EntityToken::GemPref
            | EntityToken::IndoorWood
            | EntityToken::OutdoorWood
            | EntityToken::DivineMatClothing
            | EntityToken::DivineMatCrafts
            | EntityToken::DivineMatWeapons
            | EntityToken::DivineMatArmor
            | EntityToken::Animal
            | EntityToken::AnimalAlwaysPresent
            | EntityToken::AnimalNeverMount
            | EntityToken::AnimalAlwaysMount
            | EntityToken::AnimalNeverWagonPuller
            | EntityToken::AnimalAlwaysWagonPuller
            | EntityToken::AnimalNeverSiege
            | EntityToken::AnimalAlwaysSiege
            | EntityToken::AnimalNeverPet
            | EntityToken::AnimalAlwaysPet
            | EntityToken::AnimalNeverPackAnimal
            | EntityToken::AnimalAlwaysPackAnimal
            | EntityToken::Unknown
            | EntityToken::SiegeSkilledMiners
            | EntityToken::WoodPref
            | EntityToken::UndeadCandidate
            | EntityToken::CutEntity
            | EntityToken::SelectEntity => {
                // Use the trait to parse a "flag" token
                token.parse_flag(&values, token.clone())
            }
            EntityToken::Creature { .. } => {
                token.parse_single(&values, |creature| EntityToken::Creature { creature })
            }
            EntityToken::BiomeSupport { .. } => {
                token.parse_key_value(&values, |biome, frequency| EntityToken::BiomeSupport {
                    biome,
                    frequency,
                })
            }
            EntityToken::SettlementBiome { .. } => {
                token.parse_single(&values, |biome| EntityToken::SettlementBiome { biome })
            }
            EntityToken::StartBiome { .. } => {
                token.parse_single(&values, |biome| EntityToken::StartBiome { biome })
            }
            EntityToken::ExclusiveStartBiome { .. } => {
                token.parse_single(&values, |biome| EntityToken::ExclusiveStartBiome { biome })
            }
            EntityToken::DefaultSiteType { .. } => token.parse_single(&values, |site_type| {
                EntityToken::DefaultSiteType { site_type }
            }),
            EntityToken::LikesSite { .. } => {
                token.parse_single(&values, |site_type| EntityToken::LikesSite { site_type })
            }
            EntityToken::ToleratesSite { .. } => token.parse_single(&values, |site_type| {
                EntityToken::ToleratesSite { site_type }
            }),
            EntityToken::WorldConstruction { .. } => token.parse_single(&values, |construction| {
                EntityToken::WorldConstruction { construction }
            }),
            EntityToken::MaxPopNumber { .. } => {
                token.parse_single(&values, |number| EntityToken::MaxPopNumber { number })
            }
            EntityToken::MaxSitePopNumber { .. } => {
                token.parse_single(&values, |number| EntityToken::MaxSitePopNumber { number })
            }
            EntityToken::MaxStartingCivNumber { .. } => token.parse_single(&values, |number| {
                EntityToken::MaxStartingCivNumber { number }
            }),
            EntityToken::PermittedBuilding { .. } => token.parse_single(&values, |building| {
                EntityToken::PermittedBuilding { building }
            }),
            EntityToken::PermittedJob { .. } => {
                token.parse_single(&values, |job| EntityToken::PermittedJob { job })
            }
            EntityToken::PermittedReaction { .. } => token.parse_single(&values, |reaction| {
                EntityToken::PermittedReaction { reaction }
            }),
            EntityToken::Currency { .. } => token.parse_key_value(&values, |material, value| {
                EntityToken::Currency { material, value }
            }),
            EntityToken::ArtFacetModifier { .. } => {
                token.parse_key_value(&values, |modifier, number| EntityToken::ArtFacetModifier {
                    modifier,
                    number,
                })
            }
            EntityToken::ArtImageElementModifier { .. } => token
                .parse_key_value(&values, |item, number| {
                    EntityToken::ArtImageElementModifier { item, number }
                }),
            EntityToken::ItemImprovementModifier { .. } => token
                .parse_key_value(&values, |item, number| {
                    EntityToken::ItemImprovementModifier { item, number }
                }),
            EntityToken::Translation { .. } => {
                token.parse_single(&values, |language| EntityToken::Translation { language })
            }
            EntityToken::SelectSymbol { .. } => token.parse_key_value(&values, |noun, symbol| {
                EntityToken::SelectSymbol { noun, symbol }
            }),
            EntityToken::SubselectSymbol { .. } => {
                token.parse_key_value(&values, |noun, symbol| EntityToken::SubselectSymbol {
                    noun,
                    symbol,
                })
            }
            EntityToken::CullSymbol { .. } => token.parse_key_value(&values, |noun, symbol| {
                EntityToken::CullSymbol { noun, symbol }
            }),
            EntityToken::FriendlyColor { .. } => {
                token.parse_single(&values, |color| EntityToken::FriendlyColor { color })
            }
            EntityToken::Religion { .. } => token.parse_single(&values, |religion_type| {
                EntityToken::Religion { religion_type }
            }),
            EntityToken::ReligionSphere { .. } => {
                token.parse_single(&values, |sphere| EntityToken::ReligionSphere { sphere })
            }
            EntityToken::SphereAlignment { .. } => {
                token.parse_key_value(&values, |sphere, number| EntityToken::SphereAlignment {
                    sphere,
                    number,
                })
            }
            EntityToken::Position { .. } => {
                token.parse_single(&values, |name| EntityToken::Position { name })
            }
            EntityToken::SiteVariablePositions { .. } => token
                .parse_single(&values, |responsibility| {
                    EntityToken::SiteVariablePositions { responsibility }
                }),
            EntityToken::VariablePositions { .. } => token
                .parse_single(&values, |responsibility| EntityToken::VariablePositions {
                    responsibility,
                }),
            EntityToken::Ethic { .. } => token.parse_key_value(&values, |behavior, rating| {
                EntityToken::Ethic { behavior, rating }
            }),
            EntityToken::Value { .. } => token.parse_key_value(&values, |value, strength| {
                EntityToken::Value { value, strength }
            }),
            EntityToken::VariableValue { .. } => {
                token.parse_labeled_array(&values, |value, [min, max]| EntityToken::VariableValue {
                    value,
                    min,
                    max,
                })
            }
            EntityToken::ActiveSeason { .. } => {
                token.parse_single(&values, |season| EntityToken::ActiveSeason { season })
            }
            EntityToken::Banditry { .. } => {
                token.parse_single(&values, |percentage| EntityToken::Banditry { percentage })
            }
            EntityToken::ProgressTriggerPopulation { .. } => token.parse_single(&values, |level| {
                EntityToken::ProgressTriggerPopulation { level }
            }),
            EntityToken::ProgressTriggerProduction { .. } => token.parse_single(&values, |level| {
                EntityToken::ProgressTriggerProduction { level }
            }),
            EntityToken::ProgressTriggerTrade { .. } => {
                token.parse_single(&values, |level| EntityToken::ProgressTriggerTrade { level })
            }
            EntityToken::ProgressTriggerPopulationSiege { .. } => token
                .parse_single(&values, |level| {
                    EntityToken::ProgressTriggerPopulationSiege { level }
                }),
            EntityToken::ProgressTriggerProductionSiege { .. } => token
                .parse_single(&values, |level| {
                    EntityToken::ProgressTriggerProductionSiege { level }
                }),
            EntityToken::ProgressTriggerTradeSiege { .. } => token.parse_single(&values, |level| {
                EntityToken::ProgressTriggerTradeSiege { level }
            }),
            EntityToken::Scholar { .. } => token.parse_single(&values, |scholar_type| {
                EntityToken::Scholar { scholar_type }
            }),
            EntityToken::Ammo { .. } => {
                token.parse_single(&values, |item| EntityToken::Ammo { item })
            }
            EntityToken::Armor { .. } => {
                token.parse_key_value(&values, |item, chance| EntityToken::Armor { item, chance })
            }
            EntityToken::Digger { .. } => {
                token.parse_single(&values, |item| EntityToken::Digger { item })
            }
            EntityToken::Gloves { .. } => {
                token.parse_key_value(&values, |item, chance| EntityToken::Gloves { item, chance })
            }
            EntityToken::Helm { .. } => {
                token.parse_key_value(&values, |item, chance| EntityToken::Helm { item, chance })
            }
            EntityToken::Instrument { .. } => {
                token.parse_single(&values, |item| EntityToken::Instrument { item })
            }
            EntityToken::Pants { .. } => {
                token.parse_key_value(&values, |item, chance| EntityToken::Pants { item, chance })
            }
            EntityToken::Shield { .. } => {
                token.parse_single(&values, |item| EntityToken::Shield { item })
            }
            EntityToken::Shoes { .. } => {
                token.parse_key_value(&values, |item, chance| EntityToken::Shoes { item, chance })
            }
            EntityToken::SiegeAmmo { .. } => {
                token.parse_single(&values, |item| EntityToken::SiegeAmmo { item })
            }
            EntityToken::Tool { .. } => {
                token.parse_single(&values, |item| EntityToken::Tool { item })
            }
            EntityToken::Toy { .. } => {
                token.parse_single(&values, |item| EntityToken::Toy { item })
            }
            EntityToken::TrapComponent { .. } => {
                token.parse_single(&values, |item| EntityToken::TrapComponent { item })
            }
            EntityToken::Weapon { .. } => {
                token.parse_single(&values, |item| EntityToken::Weapon { item })
            }
            EntityToken::GemShape { .. } => {
                token.parse_single(&values, |shape| EntityToken::GemShape { shape })
            }
            EntityToken::StoneShape { .. } => {
                token.parse_single(&values, |shape| EntityToken::StoneShape { shape })
            }
            EntityToken::AnimalToken { .. } => {
                token.parse_single(&values, |creature| EntityToken::AnimalToken { creature })
            }
            EntityToken::AnimalCasteToken { .. } => {
                token.parse_single(&values, |caste| EntityToken::AnimalCasteToken { caste })
            }
            EntityToken::AnimalClass { .. } => {
                token.parse_single(&values, |class| EntityToken::AnimalClass { class })
            }
            EntityToken::AnimalForbiddenClass { .. } => {
                token.parse_single(&values, |class| EntityToken::AnimalForbiddenClass { class })
            }
            EntityToken::TissueStyle { .. } => {
                token.parse_single(&values, |tissue| EntityToken::TissueStyle { tissue })
            }
            EntityToken::TissueStyleMaintainLength { .. } => token
                .parse_array(&values, |[min, max]| {
                    EntityToken::TissueStyleMaintainLength { min, max }
                }),
            EntityToken::TissueStylePreferredShaping { .. } => token
                .parse_single(&values, |style| EntityToken::TissueStylePreferredShaping {
                    style,
                }),
        }
    }
}
