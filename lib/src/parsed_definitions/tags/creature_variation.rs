//! An enum representing a creature variation tag.

use tracing::warn;

use crate::raw_definitions::CREATURE_VARIATION_TOKENS;

/// An enum representing a creature variation tag.
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Default,
    specta::Type,
    Copy,
    strum_macros::EnumIter,
)]
pub enum CreatureVariationTag {
    /// A tag to add a new tag to the creature.
    NewTag,
    /// A tag to add a tag to the creature.
    AddTag,
    /// A tag to remove a tag from the creature.
    RemoveTag,
    /// A tag to convert a tag to a new tag.
    ConvertTag,
    /// A tag to convert a tag to a new tag with specific token
    ConvertTagMaster,
    /// A tag to convert a tag to a new tag with specific target
    ConvertTagTarget,
    /// A tag to convert a tag to a new tag with specific replacement
    ConvertTagReplacement,
    /// Conditionally add a new tag to the creature.
    ConditionalNewTag,
    /// Conditionally add a tag to the creature.
    ConditionalAddTag,
    /// Conditionally remove a tag from the creature.
    ConditionalRemoveTag,
    /// Conditionally convert a tag to a new tag.
    ConditionalConvertTag,
    /// An unknown tag.
    #[default]
    Unknown,
}

impl CreatureVariationTag {
    /// Function to create a new `CVTag` from a key.
    ///
    /// # Parameters
    ///
    /// * `key` - The key to create the `CVTag` from.
    ///
    /// # Returns
    ///
    /// * `CVTag` - The `CVTag` created from the key.
    #[must_use]
    pub fn from_key(key: &str) -> Self {
        let tag = CREATURE_VARIATION_TOKENS.get(key).unwrap_or(&Self::Unknown);
        if tag == &Self::Unknown {
            warn!("Unknown creature variation (CV) tag: {}", key);
        }
        *tag
    }
}
