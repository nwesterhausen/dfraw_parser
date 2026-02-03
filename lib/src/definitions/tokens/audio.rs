pub enum AudioToken {
    /// File identifier, either a path or a reference to another sound/music file.
    File { target: String },
    /// (Music only) Displays as the song author on the pause menu
    Author { name: String },
    /// (Music only) If set, should play continuously until interrupted
    Loops,
    /// (Music only) Displays as track title on pause menu
    Title { name: String },
    /// Trigger condition, can use more than one announcement token to specify multiple triggers
    Announcement { announcement: String },
    /// Whether it should be played randomly in a savage area
    SavageArea,
    /// Defines a list of "short bits" that will be shuffled and chosen from to play for an event
    Card { music_file: String },
    /// If any context is met, the song can be chosed to play
    Context { context: SoundContext },
    /// When the chosed event occurs, the son will play and override the current song. If multiple match, a ranom
    /// one will be played from the matches.
    Event { event: SoundEvent },
    /// Can be set to `UNCOMMON` to set the frequency to "half as often" as any other matching options or `RARE` to
    /// make it 1/5 as often as other matching options.
    Frequrency { frequency: String },
}

// see https://dwarffortresswiki.org/index.php/Audio

pub enum SoundContext {
    Any,
    Main,
    FirstYear,
    SecondYearPlus,
    CavernsOpened,
    Spring,
    Summer,
    Autumn,
    Winter,
}

pub enum SoundEvent {
    JustEmbarked,
    Siege,
    FirstCavernOpened,
    MegabeastAttack,
    ForgottenBeastAttack,
    DeathSpiral,
    TavernMusicPresent,
    TavernDancePresent,
    LostFort,
    FortLevel,
    FirstGhost,
}
