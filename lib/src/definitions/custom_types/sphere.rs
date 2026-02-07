use std::str::FromStr;

use dfraw_parser_proc_macros::{Cleanable, IsEmpty};

use crate::{
    tokens::{SphereToken, SymbolToken},
    traits::RawToken,
};

/// A sphere, or cosmic principle, is an aspect where a being has influence. Deities, forces, angels,
/// demons, megabeasts, semi-megabeasts, forgotten beasts and titans may be associated with one or more
/// spheres, and civilizations may prefer certain spheres when selecting creatures to worship.
///
/// There are currently a total of 130 spheres.
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    Default,
    specta::Type,
    Eq,
    PartialEq,
    IsEmpty,
    Cleanable,
)]
pub struct Sphere {
    /// The identifier of the sphere
    pub identifier: String,
    /// List of parent spheres by identifier
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    pub parents: Vec<SphereToken>,
    /// List of children spheres by identifier
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    pub children: Vec<SphereToken>,
    /// List of friend spheres by identifier
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    pub friends: Vec<SphereToken>,
    /// List of precluded spheres by identifier
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    pub precluded_spheres: Vec<SphereToken>,
    /// A descriptive string associated with this sphere
    pub descriptor: String,
    /// Properties associated with this sphere
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    pub properties: Vec<(String, String)>,
    /// Symbols associated with this sphere
    pub symbols: Vec<SymbolToken>,
}

impl TryFrom<SphereToken> for Sphere {
    type Error = String;

    fn try_from(value: SphereToken) -> Result<Sphere, String> {
        Self::from_str(value.get_key().unwrap_or_default())
    }
}

impl TryFrom<Sphere> for SphereToken {
    type Error = String;

    fn try_from(value: Sphere) -> Result<Self, Self::Error> {
        SphereToken::from_str(&value.identifier)
    }
}

impl FromStr for Sphere {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AGRICULTURE" => Ok(Self {
                identifier: "AGRICULTURE".to_string(),
                friends: vec![SphereToken::Food, SphereToken::Rain, SphereToken::Fertility],
                descriptor: "it looks constantly to the sky for rain".to_string(),
                ..Default::default()
            }),
            "ANIMALS" => Ok(Self {
                identifier: "ANIMALS".to_string(),
                parents: vec![SphereToken::Nature],
                children: vec![SphereToken::Fish],
                friends: vec![SphereToken::Plants],
                descriptor: "it growls, buzzes, clicks and generally makes a varied racket"
                    .to_string(),
                ..Default::default()
            }),
            "ART" => Ok(Self {
                identifier: "ART".to_string(),
                ..Default::default()
            }),
            "BALANCE" => Ok(Self {
                identifier: "BALANCE".to_string(),
                ..Default::default()
            }),
            "BEAUTY" => Ok(Self {
                identifier: "BEAUTY".to_string(),
                ..Default::default()
            }),
            "BIRTH" => Ok(Self {
                identifier: "BIRTH".to_string(),
                ..Default::default()
            }),
            "BLIGHT" => Ok(Self {
                identifier: "BLIGHT".to_string(),
                ..Default::default()
            }),
            "BOUNDARIES" => Ok(Self {
                identifier: "BOUNDARIES".to_string(),
                ..Default::default()
            }),
            "CAVERNS" => Ok(Self {
                identifier: "CAVERNS".to_string(),
                ..Default::default()
            }),
            "CHAOS" => Ok(Self {
                identifier: "CHAOS".to_string(),
                ..Default::default()
            }),
            "CHARITY" => Ok(Self {
                identifier: "CHARITY".to_string(),
                ..Default::default()
            }),
            "CHILDREN" => Ok(Self {
                identifier: "CHILDREN".to_string(),
                ..Default::default()
            }),
            "COASTS" => Ok(Self {
                identifier: "COASTS".to_string(),
                ..Default::default()
            }),
            "CONSOLATION" => Ok(Self {
                identifier: "CONSOLATION".to_string(),
                ..Default::default()
            }),
            "COURAGE" => Ok(Self {
                identifier: "COURAGE".to_string(),
                ..Default::default()
            }),
            "CRAFTS" => Ok(Self {
                identifier: "CRAFTS".to_string(),
                ..Default::default()
            }),
            "CREATION" => Ok(Self {
                identifier: "CREATION".to_string(),
                ..Default::default()
            }),
            "DANCE" => Ok(Self {
                identifier: "DANCE".to_string(),
                ..Default::default()
            }),
            "DARKNESS" => Ok(Self {
                identifier: "DARKNESS".to_string(),
                ..Default::default()
            }),
            "DAWN" => Ok(Self {
                identifier: "DAWN".to_string(),
                ..Default::default()
            }),
            "DAY" => Ok(Self {
                identifier: "DAY".to_string(),
                ..Default::default()
            }),
            "DEATH" => Ok(Self {
                identifier: "DEATH".to_string(),
                ..Default::default()
            }),
            "DEFORMITY" => Ok(Self {
                identifier: "DEFORMITY".to_string(),
                ..Default::default()
            }),
            "DEPRAVITY" => Ok(Self {
                identifier: "DEPRAVITY".to_string(),
                ..Default::default()
            }),
            "DISCIPLINE" => Ok(Self {
                identifier: "DISCIPLINE".to_string(),
                ..Default::default()
            }),
            "DISEASE" => Ok(Self {
                identifier: "DISEASE".to_string(),
                ..Default::default()
            }),
            "DREAMS" => Ok(Self {
                identifier: "DREAMS".to_string(),
                ..Default::default()
            }),
            "DUSK" => Ok(Self {
                identifier: "DUSK".to_string(),
                ..Default::default()
            }),
            "DUTY" => Ok(Self {
                identifier: "DUTY".to_string(),
                ..Default::default()
            }),
            "EARTH" => Ok(Self {
                identifier: "EARTH".to_string(),
                ..Default::default()
            }),
            "FAMILY" => Ok(Self {
                identifier: "FAMILY".to_string(),
                ..Default::default()
            }),
            "FAME" => Ok(Self {
                identifier: "FAME".to_string(),
                ..Default::default()
            }),
            "FATE" => Ok(Self {
                identifier: "FATE".to_string(),
                ..Default::default()
            }),
            "FERTILITY" => Ok(Self {
                identifier: "FERTILITY".to_string(),
                ..Default::default()
            }),
            "FESTIVALS" => Ok(Self {
                identifier: "FESTIVALS".to_string(),
                ..Default::default()
            }),
            "FIRE" => Ok(Self {
                identifier: "FIRE".to_string(),
                ..Default::default()
            }),
            "FISH" => Ok(Self {
                identifier: "FISH".to_string(),
                ..Default::default()
            }),
            "FISHING" => Ok(Self {
                identifier: "FISHING".to_string(),
                ..Default::default()
            }),
            "FOOD" => Ok(Self {
                identifier: "FOOD".to_string(),
                ..Default::default()
            }),
            "FORGIVENESS" => Ok(Self {
                identifier: "FORGIVENESS".to_string(),
                ..Default::default()
            }),
            "FORTRESSES" => Ok(Self {
                identifier: "FORTRESSES".to_string(),
                ..Default::default()
            }),
            "FREEDOM" => Ok(Self {
                identifier: "FREEDOM".to_string(),
                ..Default::default()
            }),
            "GAMBLING" => Ok(Self {
                identifier: "GAMBLING".to_string(),
                ..Default::default()
            }),
            "GAMES" => Ok(Self {
                identifier: "GAMES".to_string(),
                ..Default::default()
            }),
            "GENEROSITY" => Ok(Self {
                identifier: "GENEROSITY".to_string(),
                ..Default::default()
            }),
            "HAPPINESS" => Ok(Self {
                identifier: "HAPPINESS".to_string(),
                ..Default::default()
            }),
            "HEALING" => Ok(Self {
                identifier: "HEALING".to_string(),
                ..Default::default()
            }),
            "HOSPITALITY" => Ok(Self {
                identifier: "HOSPITALITY".to_string(),
                ..Default::default()
            }),
            "HUNTING" => Ok(Self {
                identifier: "HUNTING".to_string(),
                ..Default::default()
            }),
            "INSPIRATION" => Ok(Self {
                identifier: "INSPIRATION".to_string(),
                ..Default::default()
            }),
            "JEALOUSY" => Ok(Self {
                identifier: "JEALOUSY".to_string(),
                ..Default::default()
            }),
            "JEWELS" => Ok(Self {
                identifier: "JEWELS".to_string(),
                ..Default::default()
            }),
            "JUSTICE" => Ok(Self {
                identifier: "JUSTICE".to_string(),
                ..Default::default()
            }),
            "LABOR" => Ok(Self {
                identifier: "LABOR".to_string(),
                ..Default::default()
            }),
            "LAKES" => Ok(Self {
                identifier: "LAKES".to_string(),
                ..Default::default()
            }),
            "LAWS" => Ok(Self {
                identifier: "LAWS".to_string(),
                ..Default::default()
            }),
            "LIES" => Ok(Self {
                identifier: "LIES".to_string(),
                ..Default::default()
            }),
            "LIGHT" => Ok(Self {
                identifier: "LIGHT".to_string(),
                ..Default::default()
            }),
            "LIGHTNING" => Ok(Self {
                identifier: "LIGHTNING".to_string(),
                ..Default::default()
            }),
            "LONGEVITY" => Ok(Self {
                identifier: "LONGEVITY".to_string(),
                ..Default::default()
            }),
            "LOVE" => Ok(Self {
                identifier: "LOVE".to_string(),
                ..Default::default()
            }),
            "LOYALTY" => Ok(Self {
                identifier: "LOYALTY".to_string(),
                ..Default::default()
            }),
            "LUCK" => Ok(Self {
                identifier: "LUCK".to_string(),
                ..Default::default()
            }),
            "LUST" => Ok(Self {
                identifier: "LUST".to_string(),
                ..Default::default()
            }),
            "MARRIAGE" => Ok(Self {
                identifier: "MARRIAGE".to_string(),
                ..Default::default()
            }),
            "MERCY" => Ok(Self {
                identifier: "MERCY".to_string(),
                ..Default::default()
            }),
            "METALS" => Ok(Self {
                identifier: "METALS".to_string(),
                ..Default::default()
            }),
            "MINERALS" => Ok(Self {
                identifier: "MINERALS".to_string(),
                ..Default::default()
            }),
            "MISERY" => Ok(Self {
                identifier: "MISERY".to_string(),
                ..Default::default()
            }),
            "MIST" => Ok(Self {
                identifier: "MIST".to_string(),
                ..Default::default()
            }),
            "MOON" => Ok(Self {
                identifier: "MOON".to_string(),
                ..Default::default()
            }),
            "MOUNTAINS" => Ok(Self {
                identifier: "MOUNTAINS".to_string(),
                ..Default::default()
            }),
            "MUCK" => Ok(Self {
                identifier: "MUCK".to_string(),
                ..Default::default()
            }),
            "MURDER" => Ok(Self {
                identifier: "MURDER".to_string(),
                ..Default::default()
            }),
            "MUSIC" => Ok(Self {
                identifier: "MUSIC".to_string(),
                ..Default::default()
            }),
            "NATURE" => Ok(Self {
                identifier: "NATURE".to_string(),
                ..Default::default()
            }),
            "NIGHT" => Ok(Self {
                identifier: "NIGHT".to_string(),
                ..Default::default()
            }),
            "NIGHTMARES" => Ok(Self {
                identifier: "NIGHTMARES".to_string(),
                ..Default::default()
            }),
            "OATHS" => Ok(Self {
                identifier: "OATHS".to_string(),
                ..Default::default()
            }),
            "OCEANS" => Ok(Self {
                identifier: "OCEANS".to_string(),
                ..Default::default()
            }),
            "ORDER" => Ok(Self {
                identifier: "ORDER".to_string(),
                ..Default::default()
            }),
            "PAINTING" => Ok(Self {
                identifier: "PAINTING".to_string(),
                ..Default::default()
            }),
            "PEACE" => Ok(Self {
                identifier: "PEACE".to_string(),
                ..Default::default()
            }),
            "PERSUASION" => Ok(Self {
                identifier: "PERSUASION".to_string(),
                ..Default::default()
            }),
            "PLANTS" => Ok(Self {
                identifier: "PLANTS".to_string(),
                ..Default::default()
            }),
            "POETRY" => Ok(Self {
                identifier: "POETRY".to_string(),
                ..Default::default()
            }),
            "PREGNANCY" => Ok(Self {
                identifier: "PREGNANCY".to_string(),
                ..Default::default()
            }),
            "RAIN" => Ok(Self {
                identifier: "RAIN".to_string(),
                ..Default::default()
            }),
            "RAINBOWS" => Ok(Self {
                identifier: "RAINBOWS".to_string(),
                ..Default::default()
            }),
            "REBIRTH" => Ok(Self {
                identifier: "REBIRTH".to_string(),
                ..Default::default()
            }),
            "REVELRY" => Ok(Self {
                identifier: "REVELRY".to_string(),
                ..Default::default()
            }),
            "REVENGE" => Ok(Self {
                identifier: "REVENGE".to_string(),
                ..Default::default()
            }),
            "RIVERS" => Ok(Self {
                identifier: "RIVERS".to_string(),
                ..Default::default()
            }),
            "RULERSHIP" => Ok(Self {
                identifier: "RULERSHIP".to_string(),
                ..Default::default()
            }),
            "RUMORS" => Ok(Self {
                identifier: "RUMORS".to_string(),
                ..Default::default()
            }),
            "SACRIFICE" => Ok(Self {
                identifier: "SACRIFICE".to_string(),
                ..Default::default()
            }),
            "SALT" => Ok(Self {
                identifier: "SALT".to_string(),
                ..Default::default()
            }),
            "SCHOLARSHIP" => Ok(Self {
                identifier: "SCHOLARSHIP".to_string(),
                ..Default::default()
            }),
            "SEASONS" => Ok(Self {
                identifier: "SEASONS".to_string(),
                ..Default::default()
            }),
            "SILENCE" => Ok(Self {
                identifier: "SILENCE".to_string(),
                ..Default::default()
            }),
            "SKY" => Ok(Self {
                identifier: "SKY".to_string(),
                ..Default::default()
            }),
            "SONG" => Ok(Self {
                identifier: "SONG".to_string(),
                ..Default::default()
            }),
            "SPEECH" => Ok(Self {
                identifier: "SPEECH".to_string(),
                ..Default::default()
            }),
            "STARS" => Ok(Self {
                identifier: "STARS".to_string(),
                ..Default::default()
            }),
            "STORMS" => Ok(Self {
                identifier: "STORMS".to_string(),
                ..Default::default()
            }),
            "STRENGTH" => Ok(Self {
                identifier: "STRENGTH".to_string(),
                ..Default::default()
            }),
            "SUICIDE" => Ok(Self {
                identifier: "SUICIDE".to_string(),
                ..Default::default()
            }),
            "SUN" => Ok(Self {
                identifier: "SUN".to_string(),
                ..Default::default()
            }),
            "THEFT" => Ok(Self {
                identifier: "THEFT".to_string(),
                ..Default::default()
            }),
            "THRALLDOM" => Ok(Self {
                identifier: "THRALLDOM".to_string(),
                ..Default::default()
            }),
            "THUNDER" => Ok(Self {
                identifier: "THUNDER".to_string(),
                ..Default::default()
            }),
            "TORTURE" => Ok(Self {
                identifier: "TORTURE".to_string(),
                ..Default::default()
            }),
            "TRADE" => Ok(Self {
                identifier: "TRADE".to_string(),
                ..Default::default()
            }),
            "TRAVELERS" => Ok(Self {
                identifier: "TRAVELERS".to_string(),
                ..Default::default()
            }),
            "TREACHERY" => Ok(Self {
                identifier: "TREACHERY".to_string(),
                ..Default::default()
            }),
            "TREES" => Ok(Self {
                identifier: "TREES".to_string(),
                ..Default::default()
            }),
            "TRICKERY" => Ok(Self {
                identifier: "TRICKERY".to_string(),
                ..Default::default()
            }),
            "TRUTH" => Ok(Self {
                identifier: "TRUTH".to_string(),
                ..Default::default()
            }),
            "TWILIGHT" => Ok(Self {
                identifier: "TWILIGHT".to_string(),
                ..Default::default()
            }),
            "VALOR" => Ok(Self {
                identifier: "VALOR".to_string(),
                ..Default::default()
            }),
            "VICTORY" => Ok(Self {
                identifier: "VICTORY".to_string(),
                ..Default::default()
            }),
            "VOLCANOS" => Ok(Self {
                identifier: "VOLCANOS".to_string(),
                ..Default::default()
            }),
            "WAR" => Ok(Self {
                identifier: "WAR".to_string(),
                ..Default::default()
            }),
            "WATER" => Ok(Self {
                identifier: "WATER".to_string(),
                ..Default::default()
            }),
            "WEALTH" => Ok(Self {
                identifier: "WEALTH".to_string(),
                ..Default::default()
            }),
            "WEATHER" => Ok(Self {
                identifier: "WEATHER".to_string(),
                ..Default::default()
            }),
            "WIND" => Ok(Self {
                identifier: "WIND".to_string(),
                ..Default::default()
            }),
            "WISDOM" => Ok(Self {
                identifier: "WISDOM".to_string(),
                ..Default::default()
            }),
            "WRITING" => Ok(Self {
                identifier: "WRITING".to_string(),
                ..Default::default()
            }),
            "YOUTH" => Ok(Self {
                identifier: "YOUTH".to_string(),
                ..Default::default()
            }),
            _ => Err(format!("Unknown sphere '{s}'")),
        }
    }
}
