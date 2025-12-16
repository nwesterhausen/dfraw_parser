//! Module containing the parsed gait definition.

use tracing::warn;

use crate::tags::{GaitModifierTag, GaitTypeTag};

/// Gaits are a way to describe how a creature moves. Defined in the raws with:
///
/// "GAIT:type:name:full speed:build up time:turning max:start speed:energy use"
///
/// * use `NO_BUILD_UP` if you jump immediately to full speed
///
/// these optional flags go at the end:
///
/// * `LAYERS_SLOW` - fat/muscle layers slow the movement (muscle-slowing counter-acted by strength bonus)
/// * `STRENGTH` - strength attribute can speed/slow movement
/// * `AGILITY` - agility attribute can speed/slow movement
/// * `STEALTH_SLOWS:<n>` - n is percentage slowed
/// * it would be interesting to allow quirky attributes (like mental stats), but they aren't supported yet
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
    /// The type of gait
    gait_type: GaitTypeTag,
    /// The name of the gait
    name: String,
    /// The maximum speed achievable by a creature using this gait.
    max_speed: u32,
    /// The energy use of the gait
    energy_use: u32,
    /// The gait modifiers
    ///
    /// These are optional, and may be empty.
    modifiers: Vec<GaitModifierTag>,
}

impl Gait {
    /// Parse a gait given the raw string (i.e. the string after the `GAIT:` tag)
    ///
    /// ## Parameters
    ///
    /// * `raw_gait` - The raw string to parse
    ///
    /// ## Returns
    ///
    /// The parsed gait
    #[must_use]
    pub fn from_value(value: &str) -> Self {
        let mut gait = Self::default();
        let mut parts = value.split(':');
        let mut has_build_up = false;

        // First part is the gait type
        gait.gait_type = match parts.next() {
            Some("WALK") => GaitTypeTag::Walk,
            Some("CLIMB") => GaitTypeTag::Climb,
            Some("SWIM") => GaitTypeTag::Swim,
            Some("CRAWL") => GaitTypeTag::Crawl,
            Some("FLY") => GaitTypeTag::Fly,
            Some(other) => GaitTypeTag::Other(other.to_string()),
            None => GaitTypeTag::Unknown,
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
                gait.modifiers.push(GaitModifierTag::NoBuildUp);
            } else if let Ok(value) = raw_value.parse() {
                gait.modifiers.push(GaitModifierTag::BuildUp {
                    time: value,
                    turning_max: 0,
                    start_speed: 0,
                });
                has_build_up = true;
            }
        }

        if has_build_up {
            // Next is turning max
            if let Some(raw_value) = parts.next() {
                if let Ok(value) = raw_value.parse::<u32>() {
                    // Modify the build up modifier to include the turning max
                    if let Some(GaitModifierTag::BuildUp {
                        time,
                        turning_max: _,
                        start_speed,
                    }) = gait.modifiers.pop()
                    {
                        gait.modifiers.push(GaitModifierTag::BuildUp {
                            time,
                            turning_max: value,
                            start_speed,
                        });
                    }
                }
            }

            // Next is start speed
            if let Some(raw_value) = parts.next() {
                if let Ok(value) = raw_value.parse::<u32>() {
                    // Modify the build up modifier to include the start speed
                    if let Some(GaitModifierTag::BuildUp {
                        time,
                        turning_max,
                        start_speed: _,
                    }) = gait.modifiers.pop()
                    {
                        gait.modifiers.push(GaitModifierTag::BuildUp {
                            time,
                            turning_max,
                            start_speed: value,
                        });
                    }
                }
            }
        }

        // Next is energy use. This might be the final part, or there might be modifiers after this.
        gait.energy_use = parts.next().unwrap_or("0").parse().unwrap_or(0);

        // Now we have modifiers. These are optional, so we'll just loop until we run out of parts.
        parts.clone().enumerate().for_each(|(idx, s)| match s {
            "LAYERS_SLOW" => gait.modifiers.push(GaitModifierTag::LayersSlow),
            "STRENGTH" => gait.modifiers.push(GaitModifierTag::Strength),
            "AGILITY" => gait.modifiers.push(GaitModifierTag::Agility),
            "STEALTH_SLOWS" => {
                if let Some(raw_value) = parts.nth(idx + 1) {
                    if let Ok(value) = raw_value.parse() {
                        gait.modifiers
                            .push(GaitModifierTag::StealthSlows { percentage: value });
                    }
                } else {
                    warn!("STEALTH_SLOWS modifier is missing a value in {value}");
                }
            }
            _ => {}
        });

        gait
    }

    /// Returns true if the gait is empty (i.e. unset/default)
    ///
    /// ## Returns
    ///
    /// True if the gait is empty, false otherwise.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.gait_type == GaitTypeTag::Unknown
    }
    /// Returns the type tag of the gait
    #[must_use]
    pub fn get_type(&self) -> &GaitTypeTag {
        &self.gait_type
    }
    /// Returns the max speed of the gait
    #[must_use]
    pub fn get_max_speed(&self) -> u32 {
        self.max_speed
    }
}
