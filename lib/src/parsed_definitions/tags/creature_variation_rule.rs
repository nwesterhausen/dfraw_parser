//! A module for creature variation rules.

use lazy_regex::regex;

use crate::{
    creature::Creature,
    regex::VARIATION_ARGUMENT_RE,
    utilities::{
        apply_new_tag, argument_as_string, convert_tag, remove_tag, replace_args_in_string,
    },
};

/// A variation rule for a creature.
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
pub enum CreatureVariationRuleTag {
    /// An unknown rule.
    #[default]
    Unknown,
    /// Removes a tag from a creature.
    RemoveTag {
        /// The tag to remove.
        tag: String,
        /// The value to remove.
        value: Option<String>,
    },
    /// Adds a new tag to a creature.
    NewTag {
        /// The tag to add.
        tag: String,
        /// The value to add.
        value: Option<String>,
    },
    /// Adds a new tag to a creature.
    AddTag {
        /// The tag to add.
        tag: String,
        /// The value to add.
        value: Option<String>,
    },
    /// Converts a tag on a creature.
    ConvertTag {
        /// The tag to convert.
        tag: String,
        /// The target value to convert.
        target: Option<String>,
        /// The replacement value to convert to.
        replacement: Option<String>,
    },
    /// Adds a new tag to a creature if a condition is met.
    ConditionalNewTag {
        /// The tag to add.
        tag: String,
        /// The value to add.
        value: Option<String>,
        /// The index of the argument to check.
        argument_index: usize,
        /// The requirement for the argument.
        argument_requirement: String,
    },
    /// Adds a new tag to a creature if a condition is met.
    ConditionalAddTag {
        /// The tag to add.
        tag: String,
        /// The value to add.
        value: Option<String>,
        /// The index of the argument to check.
        argument_index: usize,
        /// The requirement for the argument.
        argument_requirement: String,
    },
    /// Removes a tag from a creature if a condition is met.
    ConditionalRemoveTag {
        /// The tag to remove.
        tag: String,
        /// The value to remove.
        value: Option<String>,
        /// The index of the argument to check.
        argument_index: usize,
        /// The requirement for the argument.
        argument_requirement: String,
    },
    /// Converts a tag on a creature if a condition is met.
    ConditionalConvertTag {
        /// The tag to convert.
        tag: String,
        /// The target value to convert.
        target: Option<String>,
        /// The replacement value to convert to.
        replacement: Option<String>,
        /// The index of the argument to check.
        argument_index: usize,
        /// The requirement for the argument.
        argument_requirement: String,
    },
}

impl CreatureVariationRuleTag {
    /// Apply a set of arguments to the rule and get a rule that has the arguments applied.
    /// This will replace all instances of `!ARGn` with the corresponding argument.
    ///
    /// This returns a new rule with the arguments applied because we don't want to mutate the
    /// original rule (multiple creatures may use the same rule)
    ///
    /// ## Arguments
    ///
    /// * `args` - The arguments to apply to the rule.
    ///
    /// ## Returns
    ///
    /// * `CreatureVariationRule` - The rule with the arguments applied.
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn with_args(&self, args: &[&str]) -> Self {
        // Short circuit if there are no arguments to replace.
        if args.is_empty() {
            return self.clone();
        }
        // We simply replace all instances of `!ARGn` with the corresponding argument.
        match self {
            Self::RemoveTag { tag, value } => {
                // Only have the tag to replace.
                Self::RemoveTag {
                    tag: replace_args_in_string(tag, args),
                    value: value
                        .as_ref()
                        .map(|value| replace_args_in_string(value, args)),
                }
            }
            Self::NewTag { tag, value } | Self::AddTag { tag, value } => {
                // Have both the tag and the value to replace.
                Self::NewTag {
                    tag: replace_args_in_string(tag, args),
                    value: value
                        .as_ref()
                        .map(|value| replace_args_in_string(value, args)),
                }
            }
            Self::ConvertTag {
                tag,
                target,
                replacement,
            } => {
                // Have the tag, target, and replacement to replace.
                Self::ConvertTag {
                    tag: replace_args_in_string(tag, args),
                    target: target
                        .as_ref()
                        .map(|value| replace_args_in_string(value, args)),
                    replacement: replacement
                        .as_ref()
                        .map(|value| replace_args_in_string(value, args)),
                }
            }
            Self::ConditionalRemoveTag {
                tag,
                value,
                argument_requirement,
                argument_index,
            } => {
                // Have the tag and the argument requirement to replace.
                Self::ConditionalRemoveTag {
                    tag: replace_args_in_string(tag, args),
                    value: value
                        .as_ref()
                        .map(|value| replace_args_in_string(value, args)),
                    argument_requirement: String::from(
                        VARIATION_ARGUMENT_RE.replace_all(
                            argument_requirement.as_str(),
                            |caps: &regex::Captures| argument_as_string(caps, args),
                        ),
                    ),
                    argument_index: *argument_index,
                }
            }
            Self::ConditionalNewTag {
                tag,
                value,
                argument_requirement,
                argument_index,
            }
            | Self::ConditionalAddTag {
                tag,
                value,
                argument_requirement,
                argument_index,
            } => {
                // Have the tag, value, and argument requirement to replace.
                Self::ConditionalNewTag {
                    tag: replace_args_in_string(tag, args),
                    value: value
                        .as_ref()
                        .map(|value| replace_args_in_string(value, args)),
                    argument_requirement: String::from(
                        VARIATION_ARGUMENT_RE.replace_all(
                            argument_requirement.as_str(),
                            |caps: &regex::Captures| argument_as_string(caps, args),
                        ),
                    ),
                    argument_index: *argument_index,
                }
            }
            Self::ConditionalConvertTag {
                tag,
                target,
                replacement,
                argument_index,
                argument_requirement,
            } => {
                // Have the tag, target, replacement, and argument requirement to replace.
                Self::ConditionalConvertTag {
                    tag: replace_args_in_string(tag, args),
                    target: target
                        .as_ref()
                        .map(|value| replace_args_in_string(value, args)),
                    replacement: replacement
                        .as_ref()
                        .map(|value| replace_args_in_string(value, args)),
                    argument_requirement: String::from(
                        VARIATION_ARGUMENT_RE.replace_all(
                            argument_requirement.as_str(),
                            |caps: &regex::Captures| argument_as_string(caps, args),
                        ),
                    ),
                    argument_index: *argument_index,
                }
            }
            Self::Unknown => {
                // Unknown rules don't have anything to replace.
                Self::Unknown
            }
        }
    }
    /// Apply the rule to a creature. This will apply the rule to the creature based on the arguments
    /// provided.
    ///
    /// # Arguments
    ///
    /// * `creature` - The creature to apply the rule to.
    /// * `args` - The arguments to apply to the rule.
    ///
    /// # Side Effects
    ///
    /// This will modify the creature provided.
    pub fn apply(&self, creature: &mut Creature, args: &[&str]) {
        match self.with_args(args) {
            Self::RemoveTag { tag, .. } => {
                remove_tag(creature, &tag);
            }
            Self::NewTag { tag, value } | Self::AddTag { tag, value } => {
                apply_new_tag(creature, &tag, value.as_deref());
            }
            Self::ConvertTag {
                tag,
                target,
                replacement,
            } => convert_tag(creature, &tag, target.as_deref(), replacement.as_deref()),
            Self::ConditionalNewTag {
                tag,
                value,
                argument_index,
                argument_requirement,
            }
            | Self::ConditionalAddTag {
                tag,
                value,
                argument_index,
                argument_requirement,
            } => {
                // Guard against out of bounds arguments.
                if args.len() < argument_index {
                    tracing::warn!(
                        "Creature Variation Argument index {} is out of bounds for {:?}",
                        argument_index,
                        args
                    );
                    return;
                }
                // Check if the argument matches the requirement.
                if let Some(argument_value) = args.get(argument_index - 1)
                    && argument_value == &argument_requirement
                {
                    apply_new_tag(creature, &tag, value.as_deref());
                }
            }
            Self::ConditionalRemoveTag {
                tag,
                argument_index,
                argument_requirement,
                ..
            } => {
                // Guard against out of bounds arguments.
                if args.len() < argument_index {
                    tracing::warn!(
                        "Creature Variation Argument index {} is out of bounds for {:?}",
                        argument_index,
                        args
                    );
                    return;
                }
                // Check if the argument matches the requirement.
                if let Some(argument_value) = args.get(argument_index - 1)
                    && argument_value == &argument_requirement
                {
                    remove_tag(creature, &tag);
                }
            }
            Self::ConditionalConvertTag {
                tag,
                target,
                replacement,
                argument_index,
                argument_requirement,
            } => {
                // Guard against out of bounds arguments.
                if args.len() < argument_index {
                    tracing::warn!(
                        "Creature Variation Argument index {} is out of bounds for {:?}",
                        argument_index,
                        args
                    );
                    return;
                }
                // Check if the argument matches the requirement.
                if let Some(argument_value) = args.get(argument_index - 1)
                    && argument_value == &argument_requirement
                {
                    convert_tag(creature, &tag, target.as_deref(), replacement.as_deref());
                }
            }
            Self::Unknown => {}
        }
    }
}

impl std::fmt::Display for CreatureVariationRuleTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
