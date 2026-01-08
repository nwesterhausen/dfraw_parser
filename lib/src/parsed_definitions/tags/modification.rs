//! Tags that modify a creature

/// A struct representing a modification to a creature
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    specta::Type,
    strum_macros::EnumIter,
)]
pub enum ModificationTag {
    /// `COPY_TAGS_FROM` tag
    CopyTagsFrom {
        /// The creature to copy tags from
        identifier: String,
    },
    /// `APPLY_CREATURE_VARIATION` tag
    ApplyCreatureVariation {
        /// The creature to apply the variation from
        identifier: String,
    },
    /// Follows `GO_TO_END` until `GO_TO_START` or object definition finishes
    ///
    /// When using tags from an existing creature, inserts new tags at the end of the creature.
    AddToEnding {
        /// The set of raws to add to the end of the object
        ///
        /// This should be the entire raw in order to apply.
        raws: Vec<String>,
    },
    /// Follows `GO_TO_START` until `GO_TO_END` or object definition finishes
    ///
    /// When using tags from an existing creature, inserts new tags at the beginning of the creature.
    AddToBeginning {
        /// The set of raws to add to the beginning of the object
        ///
        /// This should be the entire raw in order to apply.
        raws: Vec<String>,
    },
    /// `GO_TO_TAG:tag` raw instruction
    ///
    /// When using tags from an existing creature, inserts new tags before the specified tag.
    AddBeforeTag {
        /// The tag to insert before
        ///
        /// Since we don't actually know the tag order after parsing, this will be ignored in parsing, and
        /// instead will just apply the raws...
        tag: String,
        /// The set of raws to add before the tag
        ///
        /// This should be the entire raw in order to apply.
        raws: Vec<String>,
    },
    /// The main body of the object
    MainRawBody {
        /// The set of raws that make up the object. This is usually defined first unless
        /// its specified to be added to the end or beginning (or before a tag)
        ///
        /// This should be the entire raw in order to apply.
        raws: Vec<String>,
    },
}

impl ModificationTag {
    /// Adds a raw to the modification
    ///
    /// # Arguments
    ///
    /// * `format` - The raw to add
    #[allow(dead_code)]
    pub(crate) fn add_raw(&mut self, format: String) {
        match self {
            Self::AddToEnding { raws }
            | Self::AddToBeginning { raws }
            | Self::AddBeforeTag { raws, .. }
            | Self::MainRawBody { raws } => raws.push(format),
            _ => {}
        }
    }
}

impl std::fmt::Display for ModificationTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
