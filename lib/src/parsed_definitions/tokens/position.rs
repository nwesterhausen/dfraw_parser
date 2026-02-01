//! Position tags are used to define the properties of a position in the game. They are used in the `position` token.

use crate::{custom_types::Amount, traits::IsEmpty};

/// Represents a position token
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Default,
    specta::Type,
    strum_macros::EnumIter,
)]
pub enum PositionToken {
    /// The position holder is not subjected to the economy. Less than relevant right now.
    AccountExempt,
    /// Arguments: creature class token
    ///
    /// Only creatures with the specified class token can be appointed to this position. Multiple entries are allowed
    AllowedClass { class: String },
    /// Arguments: creature:caste token
    ///
    /// Restricts the position to only the defined caste. Only works with a caste of the entity's current race.
    /// (If the entity had multiple CREATURE: tokens). Multiple entries are allowed
    AllowedCreature { creature: String, caste: String },
    /// Arguments: position
    ///
    /// This position can only be chosen for the task from the nobles screen, and is available only if there is an *argument* present.
    /// For example, the `GENERAL` is `[APPOINTED_BY:MONARCH]`. Contrast `[ELECTED]`. Being appointed by a `MONARCH` seems to handle a lot of
    /// worldgen stuff, and interferes with fort mode titles. Multiple entries are allowed. If you have neither an `ELECTED`-token nor a
    /// `APPOINTED_BY`-token, the holder may always be changed (like the expedition leader)
    AppointedBy { position: String },
    /// A creature that kills a member of this position will be sure to talk about it a lot.
    BragOnKill,
    /// In adventure mode, when referencing locations, an NPC may mention this position holder living there or having done some
    /// deed there, it also means that the position exists in world-gen, rather than being created only at the end of world-gen.
    ///
    /// Before 47.05, Dark Fortress civs cannot have this tag on anybody but their Law Maker, or the game will crash without
    /// leaving an errorlog.
    ChatWorthy,
    /// Arguments: color:background:foreground
    ///
    /// Creatures of this position will have this color, instead of their profession color
    ///
    /// e.g. `[COLOR:5:0:1]`.
    Color {
        foreground: u32,
        background: u32,
        brightness: u32,
    },
    /// Arguments: position, 'ALL'
    ///
    /// This position will act as a commander of the specified position.
    ///
    /// E.g. GENERAL is `[COMMANDER:LIEUTENANT:ALL]`. Unknown if values other than ALL work. Multiple entries are allowed
    Commander {
        position: String,
        commanded_position: String,
    },
    /// This position is a puppet ruler left behind in a conquered site.
    ConqueredSite,
    /// Arguments: number (0-`100`)
    ///
    /// How many demands the position can make of the population at one time.
    DemandMax { amount: u32 },
    /// The site's (or civ's) minted coins, if any, will have images that reflect the personality of this position holder.
    DeterminesCoinDesign,
    /// The position won't be culled from Legends as "unimportant" during world generation.
    DoNotCull,
    /// Members of this position will never agree to 'join' your character during adventure mode.
    DutyBound,
    /// The population will periodically select the most skill-eligible creature to fill this position for site-level positions
    /// at the player's fort. For responsibilities or positions that use more than one skill, no skill takes priority in electing
    /// a creature: an accomplished comedian is more qualified for the TRADE responsibility than a skilled appraiser.
    /// A creature may be elected to multiple positions at the same time. Contrast `[APPOINTED_BY]`. More info: Elections
    Elected,
    /// Arguments: weapon skill
    ///
    /// A mandatory sub-tag of `[RESPONSIBILITY:EXECUTIONS]`. Determines the weapon chosen by the executioner for their work.
    ExecutionSkill { skill: String },
    /// The various members who have filled this role will be listed in the civilization's history.
    ExportedInLegends,
    /// The creature holding this position will visibly flash, like legendary citizens. Represents a properly noble station by default.
    Flashes,
    /// Arguments: 'MALE' or 'FEMALE'
    ///
    /// The position can only be held by the specified gender. Currently bugged Bug:2714
    Gender { name: String },
    /// The position can assign quests to adventurers.
    KillQuest,
    /// Arguments: importance tier (1-`10`)
    ///
    /// This is an alternative to `SITE`. What it does is allow positions to be created at civ-level 'as needed' for all sites that
    /// meet the requirements to have them, which are the values set in `LAND_HOLDER_TRIGGER`. The character is tied permanently to
    /// a particular site but also operates at the civ-level. Since 50* modded levels of higher than 3 are possible.
    LandHolder { importance: u32 },
    /// Arguments: name (a string)
    ///
    /// The name the area takes on when under the control of a `LAND_HOLDER`.
    ///
    /// E.g. for the DUKE, `[LAND_NAME:a duchy]`.
    ///
    /// If the position is not a `LAND_HOLDER`, the `land_name` is still displayed left of the position in the nobles menu.
    LandName { name: String },
    /// Arguments: number (0-`100`)
    ///
    /// The maximum number of mandates the position can make at once.
    MandateMax,
    /// The position holder cannot be assigned labors. Currently nonfunctional.Bug:3721
    MenialWorkExemption,
    /// The spouse of the position holder doesn't have to work, either - see above.
    MenialWorkExemptionSpouse,
    /// This position cannot be appointed from the nobles screen. Intended for militia captains and other squad leaders to reduce clutter. Currently nonfunctionalBug:8965
    MilitaryScreenOnly,
    /// Arguments: `SingPlurName`
    ///
    /// The name of the position.
    Name { singular: String, plural: String },
    /// Arguments: `SingPlurName`
    ///
    /// If the creature holding the position is male, this is the position's name.
    ///
    /// E.g. for MONARCH, `[NAME_MALE:king:kings]`
    NameMale { singular: String, plural: String },
    /// Arguments: `SingPlurName`
    ///
    /// If the creature holding the position is female, this is the position's name.
    ///
    /// E.g. for MONARCH, `[NAME_FEMALE:queen:queens]`
    NameFemale { singular: String, plural: String },
    /// arguments: description
    ///
    /// Description of this position in the nobles screen.
    Description { description: String },
    /// Arguments: number or `AS_NEEDED`
    ///
    /// How many of the position there should be. If the `[SITE]` token exists, this is per site, otherwise this is per civilization.
    ///
    /// `AS_NEEDED` applies only to positions involved with the military command chain; this is used to allow armies to expand to
    /// whatever size they need to be. Non-military positions with `NUMBER:AS_NEEDED` will not be appointed.
    /// The problem with Lieutenants and Captains not been created, is their `AS_NEEDED` number.
    /// They are only then created when the're needed, and that has some pretty unusual conditions.
    /// When a fixed number is used, they are appointed with the creation of the civ.
    Number { number: Amount },
    /// Arguments: number (0 - `30_000`) or 'NONE'
    ///
    /// How important the position is in society; a lower number is more important and displayed higher in the Nobles menu.
    /// For `MONARCH` it's 1, for `MILITIA_CAPTAIN` it's 200. The game just assumes that anything with `[PRECEDENCE:1]` is the ruler,
    /// for both embark screen and mountain home purposes.
    ///
    /// A civ-position will also be created without precedence. Positions may have the same precedence and will be appointed,
    /// although the effect is unknown.
    Precedence { importance: Amount },
    /// The position holder will not be held accountable for his or her crimes. Currently nonfunctional.
    PunishmentExemption,
    /// The position holder can give quests in Adventure mode. Functionality in 0.31.13 and later is uncertain.
    QuestGiver,
    /// Arguments: creature class token
    ///
    /// Creatures of the specified class cannot be appointed to this position. Multiple entries are allowed
    RejectedClass { class: String },
    /// Arguments: `creature:caste` token
    ///
    /// Restricts position holders by `CREATURE` type. Multiple entries are allowed
    RejectedCreature { creature: String, caste: String },
    /// Arguments: position
    ///
    /// This position is absorbed by another down the line. For example, expedition leader is `[REPLACED_BY:MAYOR]`.
    /// Only a single entry is allowed.
    ReplacedBy { position: String },
    /// Arguments: number (0 - `10_000_000`)
    ///
    /// The position holder requires a bedroom with at least this value.
    RequiredBedroom { value: u32 },
    /// Arguments: number (0 - `100`)
    ///
    /// The position holder requires at least this many boxes.
    RequiredBoxes { amount: u32 },
    /// Arguments: number (0 - `100`)
    ///
    /// The position holder requires at least this many cabinets.
    RequiredCabinets { amount: u32 },
    /// Arguments: number (0 - `10_000_000`)
    ///
    /// The position holder requires a dining room with at least this value.
    RequiredDining { value: u32 },
    /// Arguments: number (0 - `10_000_000`)
    ///
    /// The position holder requires an office with at least this value.
    RequiredOffice { value: u32 },
    /// Arguments: number (0 - `100`)
    ///
    /// The position holder requires at least this many weapon racks.
    RequiredRacks { amount: u32 },
    /// Arguments: number (0 - `100`)
    ///
    /// The position holder requires at least this many armour stands.
    RequiredStands { amount: u32 },
    /// Arguments: number (0 - `10_000_000`)
    ///
    /// The position holder requires a tomb with at least this value.
    RequiredTomb { value: u32 },
    /// Does not have anything directly to do with markets. It means that in minor sites (such as hillocks) the position will not
    /// appear, while in major sites (such as dwarf fortresses) it will.
    RequiresMarket,
    /// Arguments: number
    ///
    /// The position requires the population to be at least this number before it becomes available, or before the position holder
    /// will move in.
    RequiresPopulation { population: u32 },
    /// Arguments: responsibility
    ///
    /// The position holder does a thing. See the table below for suitable arguments.
    ///
    /// A position does not need to have a responsibility.
    Responsibility { responsibility: String },
    /// If there is a special location set aside for rulers, such as a human castle/mead hall, the position holder will always be
    /// found at that particular location. Does nothing for dwarven nobles, because at present, dwarves have no such special locations.
    RulesFromLocation,
    /// Every site government will have the defined number of this position instead of the whole civilization; provided that other
    /// criteria (if any) are met. Unless `LAND_HOLDER` is present instead, the defined number of the position will be created only
    /// for the civilization as a whole.
    Site,
    /// The position holder will get upset if someone with a higher `PRECEDENCE` holds quarters with a greater value than their own.
    SleepPretension,
    /// The civilization will inter the corpse of the position holder in a special grave, either in catacombs or in monuments.
    /// If that grave is disturbed, the position holder can return as a mummy.
    SpecialBurial,
    /// Arguments: `SingPlurName`
    ///
    /// The name of the position holder's spouse.
    Spouse { singular: String, plural: String },
    /// Arguments: `SingPlurName`
    ///
    /// If the spouse of the creature holding the position is female, this is the spouse's position name.
    SpouseFemale { singular: String, plural: String },
    /// Arguments: `SingPlurName`
    ///
    /// If the spouse of the creature holding the position is male, this is the spouse's position name.
    SpouseMale { singular: String, plural: String },
    /// Arguments: `number:SingPlurName`
    ///
    /// The position holder is authorized to form a military squad, led by themselves using the leader and military tactics skills.
    /// The number denotes the maximum headcount. The noun used to describe the subordinates (e.g. royal guard) is used in adventure
    /// mode for the adventurer.
    Squad {
        headcount: u32,
        singular: String,
        plural: String,
    },
    /// Arguments: `BY_HEIR` or `BY_POSITION:position`
    ///
    /// How a new position holder is chosen. A single position can have multiple `BY_POSITION` tokens.
    /// See Noble for more information on how succession is handled in the game.
    Succession { inheritor: String },
    /// An uknown token.
    #[default]
    Unknown,
}

impl std::fmt::Display for PositionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl IsEmpty for PositionToken {
    fn is_empty(&self) -> bool {
        self == &PositionToken::Unknown
    }
}
