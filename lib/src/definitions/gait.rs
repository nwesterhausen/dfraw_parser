//! Module containing the parsed gait definition.

use tracing::warn;

use crate::tokens::{GaitModifierToken, GaitTypeToken};

/// A struct describing how a creature moves.
///
/// Gaits define the mechanics of movement modes like walking, swimming, or flying,
/// including speed, acceleration, and energy costs. They are defined in raw files
/// using the `[GAIT:type:name:full_speed:build_up:turning:start_speed:energy_use]` tag.
///
/// These optional flags go at the end:
///
/// * `LAYERS_SLOW` - fat/muscle layers slow the movement (muscle-slowing counter-acted by strength bonus)
/// * `STRENGTH` - strength attribute can speed/slow movement
/// * `AGILITY` - agility attribute can speed/slow movement
/// * `STEALTH_SLOWS:<n>` - n is percentage slowed
///
/// Instead of specifying a `build_up` you can use `NO_BUILD_UP` to instantly get to speed.
///
///
/// Examples:
///
///    `[CV_NEW_TAG:GAIT:WALK:Sprint:!ARG4:10:3:!ARG2:50:LAYERS_SLOW:STRENGTH:AGILITY:STEALTH_SLOWS:50]`
///    `[CV_NEW_TAG:GAIT:WALK:Run:!ARG3:5:3:!ARG2:10:LAYERS_SLOW:STRENGTH:AGILITY:STEALTH_SLOWS:20]`
///    `[CV_NEW_TAG:GAIT:WALK:Jog:!ARG2:NO_BUILD_UP:5:LAYERS_SLOW:STRENGTH:AGILITY:STEALTH_SLOWS:10]`
///    `[CV_NEW_TAG:GAIT:WALK:Walk:!ARG1:NO_BUILD_UP:0]`
///    `[CV_NEW_TAG:GAIT:WALK:Stroll:!ARG5:NO_BUILD_UP:0]`
///    `[CV_NEW_TAG:GAIT:WALK:Creep:!ARG6:NO_BUILD_UP:0]`
#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq, specta::Type,
)]
#[serde(rename_all = "camelCase")]
pub struct Gait {
    /// The movement medium (e.g., [`GaitTypeTag::Walk`], [`GaitTypeTag::Swim`]).
    gait_type: GaitTypeToken,
    /// The descriptive name of the movement (e.g., "Sprint", "Jog").
    name: String,
    /// The maximum speed achievable, where lower values are faster.
    max_speed: u32,
    /// The time in game ticks required to reach full speed.
    build_up_time: u32,
    /// The maximum speed at which the creature can turn effectively.
    turning_max: u32,
    /// The speed at which the creature begins moving from a standstill.
    start_speed: u32,
    /// The fatigue or energy cost associated with this movement.
    energy_use: u32,
    /// Optional modifiers affecting speed based on attributes or stealth.
    modifiers: Vec<GaitModifierToken>,
}

impl Gait {
    /// Parses a gait definition from a raw string value.
    ///
    /// * `value` - The colon-separated string from a `GAIT` tag.
    ///
    /// Returns a [`Gait`] populated with the parsed values. Optional flags like
    /// `LAYERS_SLOW` or `STEALTH_SLOWS` are parsed into the `modifiers` list.
    ///
    /// # Examples
    ///
    /// ```
    /// use dfraw_parser::Gait;
    /// let gait = Gait::from_value("WALK:Sprint:1000:NO_BUILD_UP:3:500:50:STRENGTH");
    /// ```
    #[must_use]
    pub fn from_value(value: &str) -> Self {
        let mut gait = Self::default();
        let mut parts = value.split(':');
        let mut has_build_up = false;

        // First part is the gait type
        gait.gait_type = match parts.next() {
            Some("WALK") => GaitTypeToken::Walk,
            Some("CLIMB") => GaitTypeToken::Climb,
            Some("SWIM") => GaitTypeToken::Swim,
            Some("CRAWL") => GaitTypeToken::Crawl,
            Some("FLY") => GaitTypeToken::Fly,
            Some(other) => GaitTypeToken::Other(other.to_string()),
            None => GaitTypeToken::Unknown,
        };

        // Next will be gait name
        gait.name = parts.next().unwrap_or("").to_string();

        // Next will be full speed
        gait.max_speed = parts.next().unwrap_or("0").parse().unwrap_or(0);

        // Next is build up time. Now if this is `NO_BUILD_UP`, then we don't have a build up time, and we also
        // don't have a turning max or start speed. Otherwise, we have a build up time, and we *should* have a
        // turning max and start speed.
        if let Some(raw_value) = parts.next() {
            if raw_value == "NO_BUILD_UP" {
                gait.modifiers.push(GaitModifierToken::NoBuildUp);
            } else if let Ok(value) = raw_value.parse() {
                gait.modifiers.push(GaitModifierToken::BuildUp {
                    time: value,
                    turning_max: 0,
                    start_speed: 0,
                });
                has_build_up = true;
            }
        }

        if has_build_up {
            // Next is turning max
            if let Some(raw_value) = parts.next()
                && let Ok(value) = raw_value.parse::<u32>()
            {
                // Modify the build up modifier to include the turning max
                if let Some(GaitModifierToken::BuildUp {
                    time,
                    turning_max: _,
                    start_speed,
                }) = gait.modifiers.pop()
                {
                    gait.modifiers.push(GaitModifierToken::BuildUp {
                        time,
                        turning_max: value,
                        start_speed,
                    });
                }
            }

            // Next is start speed
            if let Some(raw_value) = parts.next()
                && let Ok(value) = raw_value.parse::<u32>()
            {
                // Modify the build up modifier to include the start speed
                if let Some(GaitModifierToken::BuildUp {
                    time,
                    turning_max,
                    start_speed: _,
                }) = gait.modifiers.pop()
                {
                    gait.modifiers.push(GaitModifierToken::BuildUp {
                        time,
                        turning_max,
                        start_speed: value,
                    });
                }
            }
        }

        // Next is energy use. This might be the final part, or there might be modifiers after this.
        gait.energy_use = parts.next().unwrap_or("0").parse().unwrap_or(0);

        // Now we have modifiers. These are optional, so we'll just loop until we run out of parts.
        parts.clone().enumerate().for_each(|(idx, s)| match s {
            "LAYERS_SLOW" => gait.modifiers.push(GaitModifierToken::LayersSlow),
            "STRENGTH" => gait.modifiers.push(GaitModifierToken::Strength),
            "AGILITY" => gait.modifiers.push(GaitModifierToken::Agility),
            "STEALTH_SLOWS" => {
                if let Some(raw_value) = parts.nth(idx + 1) {
                    if let Ok(value) = raw_value.parse() {
                        gait.modifiers
                            .push(GaitModifierToken::StealthSlows { percentage: value });
                    }
                } else {
                    warn!("STEALTH_SLOWS modifier is missing a value in {value}");
                }
            }
            _ => {}
        });

        gait
    }

    /// Returns true if the gait is empty or uninitialized.
    ///
    /// A gait is considered empty if its type is [`GaitTypeTag::Unknown`].
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.gait_type == GaitTypeToken::Unknown
    }

    /// Returns the movement medium tag for this gait.
    ///
    /// This indicates if the movement is walking, swimming, flying, etc.
    #[must_use]
    pub fn get_gait_type(&self) -> &GaitTypeToken {
        &self.gait_type
    }

    /// Returns the descriptive name of the gait.
    ///
    /// Examples include "Sprint", "Run", or "Humble Walk".
    #[must_use]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns the maximum speed value.
    ///
    /// In Dwarf Fortress, lower values represent higher speeds.
    #[must_use]
    pub fn get_full_speed(&self) -> u32 {
        self.max_speed
    }

    /// Returns the maximum speed value.
    ///
    /// In Dwarf Fortress, lower values represent higher speeds.
    #[must_use]
    #[deprecated = "Use `get_full_speed` instead."]
    pub fn get_max_speed(&self) -> u32 {
        self.max_speed
    }

    /// Returns the time required to reach full speed.
    ///
    /// Measured in game ticks. A value of 0 usually indicates "NO_BUILD_UP".
    #[must_use]
    pub fn get_build_up_time(&self) -> u32 {
        self.build_up_time
    }

    /// Returns the turning speed limit.
    ///
    /// This limits how fast a creature can change direction while using this gait.
    #[must_use]
    pub fn get_turning_max(&self) -> u32 {
        self.turning_max
    }

    /// Returns the initial movement speed.
    ///
    /// The speed at which movement starts before the build-up phase begins.
    #[must_use]
    pub fn get_start_speed(&self) -> u32 {
        self.start_speed
    }

    /// Returns the energy or fatigue cost of this movement.
    ///
    /// Higher values cause the creature to tire more quickly.
    #[must_use]
    pub fn get_energy_use(&self) -> u32 {
        self.energy_use
    }

    /// Returns a slice of active modifiers for this gait.
    ///
    /// These include attribute scaling (Strength, Agility) and stealth penalties.
    #[must_use]
    pub fn get_modifiers(&self) -> &[GaitModifierToken] {
        self.modifiers.as_slice()
    }
}
