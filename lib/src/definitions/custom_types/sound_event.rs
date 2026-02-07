use std::{fmt::Display, str::FromStr};

/// Specifies where to attach a body part or tissue
#[derive(
    Debug, Default, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, specta::Type, Eq,
)]
pub enum SoundEvent {
    /// Plays when founding a new fortress.
    #[default]
    JustEmbarked,
    /// A siege is announced.
    Siege,
    /// A new cave layer is discovered
    FirstCavernOpened,
    /// Plays when a megabeast's arrival is announced. It is unknown if it is also relevant for semi-megabeasts or titans.
    MegabeastAttack,
    /// Plays when a forgotten beast's arrival is announced.
    ForgottenBeastAttack,
    /// Many citizen deaths have occurred in short succession.
    DeathSpiral,
    /// Many units have gathered to perform or watch a musical form.
    TavernMusicPresent,
    /// Many units have gathered to perform or watch a dance.
    TavernDancePresent,
    /// Ending a game: a fortress has just been abandoned or retired or your adventurer has died.
    LostFort,
    /// May relate to reaching higher ranks of Fortress Nomenclature
    FortLevel,
    /// The first time a ghost attacks
    FirstGhost,
}

// To allow `parse_single` to work
impl FromStr for SoundEvent {
    type Err = String; // Or a specific error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "JUST_EMBARKED" => Ok(SoundEvent::JustEmbarked),
            "SIEGE" => Ok(SoundEvent::Siege),
            "FIRST_CAVERN_OPENED" => Ok(SoundEvent::FirstCavernOpened),
            "MEGABEAST_ATTACK" => Ok(SoundEvent::MegabeastAttack),
            "FORGOTTEN_BEAST_ATTACK" => Ok(SoundEvent::ForgottenBeastAttack),
            "DEATH_SPIRAL" => Ok(SoundEvent::DeathSpiral),
            "TAVERN_MUSIC_PRESENT" => Ok(SoundEvent::TavernMusicPresent),
            "TAVERN_DANCE_PRESENT" => Ok(SoundEvent::TavernDancePresent),
            "LOST_FORT" => Ok(SoundEvent::LostFort),
            "FORT_LEVEL" => Ok(SoundEvent::FortLevel),
            "FIRST_CHOST" => Ok(SoundEvent::FirstGhost),
            _ => Err(format!("Unknown body part position '{s}'")),
        }
    }
}

impl Display for SoundEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SoundEvent::JustEmbarked => write!(f, "JUST_EMBARKED"),
            SoundEvent::Siege => write!(f, "SIEGE"),
            SoundEvent::FirstCavernOpened => write!(f, "FIRST_CAVERN_OPENED"),
            SoundEvent::MegabeastAttack => write!(f, "MEGABEAST_ATTACK"),
            SoundEvent::ForgottenBeastAttack => write!(f, "FORGOTTEN_BEAST_ATTACK"),
            SoundEvent::DeathSpiral => write!(f, "DEATH_SPIRAL"),
            SoundEvent::TavernMusicPresent => write!(f, "TAVERN_MUSIC_PRESENT"),
            SoundEvent::TavernDancePresent => write!(f, "TAVERN_DANCE_PRESENT"),
            SoundEvent::LostFort => write!(f, "LOST_FORT"),
            SoundEvent::FortLevel => write!(f, "FORT_LEVEL"),
            SoundEvent::FirstGhost => write!(f, "FIRST_CHOST"),
        }
    }
}
