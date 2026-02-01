use std::{
    collections::HashMap,
    mem::{Discriminant, discriminant},
    sync::OnceLock,
};

use crate::{raw_definitions::CREATURE_TOKENS, tokens::CreatureToken, traits::RawToken};

impl RawToken for CreatureToken {
    fn get_key(&self) -> Option<&'static str> {
        // Define static storage for the reverse map
        static REVERSE_MAP: OnceLock<HashMap<Discriminant<CreatureToken>, &'static str>> =
            OnceLock::new();

        // Initialize it lazily (only runs once)
        let map = REVERSE_MAP.get_or_init(|| {
            let mut m = HashMap::new();
            // Iterate the existing PHF map
            for (key, tag_template) in &CREATURE_TOKENS {
                // Key: The Enum Variant (Discriminant)
                // Value: The String Token (e.g., "FLIER")
                m.insert(discriminant(tag_template), *key);
            }
            m
        });

        // Lookup the key using the discriminant of 'self'
        map.get(&discriminant(self)).copied()
    }

    fn to_raw_token(&self) -> String {
        let key = match self.get_key() {
            Some(key) => key,
            None => return String::new(),
        };

        match self {
            CreatureToken::AltTile { character } => format!("[{key}:{character}]"),
            CreatureToken::ApplyCreatureVariation { id, args } => {
                format!("[{key}:{id}:{}]", args.join(":"))
            }

            CreatureToken::Biome { id } => format!("[{key}:{id}]"),
            CreatureToken::Caste { name } => format!("[{key}:{name}]"),
            CreatureToken::ChangeFrequencyPercent { percent } => format!("[{key}:{percent}]"),
            CreatureToken::ClusterNumber { min, max } => format!("[{key}:{min}:{max}]"),
            CreatureToken::CopyTagsFrom { creature } => format!("[{key}:{creature}]"),
            CreatureToken::CreatureSoldierTile { character } => format!("[{key}:{character}]"),
            CreatureToken::CreatureTile { character } => format!("[{key}:{character}]"),
            CreatureToken::Color { color } => format!("[{key}:{}]", color.as_value()),

            CreatureToken::Frequency { frequency } => format!("[{key}:{frequency}]"),
            CreatureToken::GeneralBabyName { name } => {
                format!("[{key}:{}]", name.as_vec().join(":"))
            }
            CreatureToken::GeneralChildName { name } => {
                format!("[{key}:{}]", name.as_vec().join(":"))
            }

            CreatureToken::GlowColor { color } => format!("[{key}:{}]", color.as_value()),
            CreatureToken::GlowTile { character } => format!("[{key}:{character}]"),

            CreatureToken::GoToTag { tag } => format!("[{key}:{tag}]"),
            CreatureToken::HarvestProduct {
                number,
                time,
                item_tokens,
            } => format!("[{key}:{number}:{time}:{}]", item_tokens.join(":")),

            CreatureToken::Name { name } => format!("[{key}:{}]", name.as_vec().join(":")),
            CreatureToken::PlusMaterial { material } => format!("[{key}:{material}]"),
            CreatureToken::PopulationNumber { min, max } => format!("[{key}:{min}:{max}]"),
            CreatureToken::PrefString { pref_string } => format!("[{key}:{pref_string}]"),
            CreatureToken::ProfessionName {
                id,
                name,
                plural_name,
            } => format!("[{key}:{id}:{name}:{plural_name}]"),
            CreatureToken::RemoveMaterial { material } => format!("[{key}:{material}]"),
            CreatureToken::RemoveTissue { tissue } => format!("[{key}:{tissue}]"),

            CreatureToken::SelectAdditionalCaste { caste } => format!("[{key}:{caste}]"),
            CreatureToken::SelectCaste { caste } => format!("[{key}:{caste}]"),
            CreatureToken::SelectMaterial { material } => format!("[{key}:{material}]"),
            CreatureToken::SelectTissue { tissue } => format!("[{key}:{tissue}]"),
            CreatureToken::SlainSpeech { slain_speech } => format!("[{key}:{slain_speech}]"),
            CreatureToken::SmellTrigger { smell_trigger } => format!("[{key}:{smell_trigger}]"),
            CreatureToken::SoldierAltTile { tile } => format!("[{key}:{tile}]"),
            CreatureToken::SourceHfid { hfid } => format!("[{key}:{hfid}]"),
            CreatureToken::Sphere { sphere } => format!("[{key}:{sphere}]"),
            CreatureToken::Tissue { name } => format!("[{key}:{name}]"),
            CreatureToken::TriggerableGroup { min, max } => format!("[{key}:{min}:{max}]"),

            CreatureToken::UndergroundDepth { min, max } => format!("[{key}:{min}:{max}]"),
            CreatureToken::UseCaste {
                caste,
                original_caste,
            } => format!("[{key}:{caste}:{original_caste}]"),
            CreatureToken::UseMaterial {
                material,
                original_material,
            } => format!("[{key}:{material}:{original_material}]"),
            CreatureToken::UseMaterialTemplate { material, template } => {
                format!("[{key}:{material}:{template}]")
            }
            CreatureToken::UseTissue {
                tissue,
                original_tissue,
            } => format!("[{key}:{tissue}:{original_tissue}]"),
            CreatureToken::UseTissueTemplate { tissue, template } => {
                format!("[{key}:{tissue}:{template}]")
            }
            CreatureToken::ApplyCurrentCreatureVariation
            | CreatureToken::ArtificialHiveable
            | CreatureToken::DoesNotExist
            | CreatureToken::EquipmentWagon
            | CreatureToken::Evil
            | CreatureToken::Fanciful
            | CreatureToken::Generated
            | CreatureToken::Good
            | CreatureToken::GoToEnd
            | CreatureToken::GoToStart
            | CreatureToken::LargeRoaming
            | CreatureToken::LocalPopsControllable
            | CreatureToken::LocalPopsProduceHeroes
            | CreatureToken::LooseClusters
            | CreatureToken::Mundane
            | CreatureToken::Savage
            | CreatureToken::Ubiquitous
            | CreatureToken::Utterances
            | CreatureToken::VerminEater
            | CreatureToken::VerminFish
            | CreatureToken::VerminGrounder
            | CreatureToken::VerminRotter
            | CreatureToken::VerminSoil
            | CreatureToken::VerminSoilColony
            | CreatureToken::Unknown
            | CreatureToken::MatesToBreed
            | CreatureToken::TwoGenders
            | CreatureToken::AllCastesAlive
            | CreatureToken::SmallRace
            | CreatureToken::OccursAsEntityRace
            | CreatureToken::Equipment => format!("[{key}]"),
        }
    }
}
