//! A module containing the `CreatureEffect` struct and its implementations.

use crate::{default_checks, tags::CreatureEffectPropertyTag};

/// A creature effect.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct CreatureEffect {
    severity: u32,
    probability: u8,

    #[serde(skip_serializing_if = "Option::is_none")]
    affected_body_parts_by_category: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    affected_body_parts_by_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    affected_body_parts_by_token: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<CreatureEffectPropertyTag>>,

    start: u32,
    peak: u32,
    end: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    dwf_stretch: Option<u8>,
}

impl CreatureEffect {
    /// Function to "clean" the creature. This is used to remove any empty list or strings,
    /// and to remove any default values. By "removing" it means setting the value to None.
    ///
    /// This also will remove the metadata if `is_metadata_hidden` is true.
    ///
    /// Steps:
    /// - Set any metadata to None if `is_metadata_hidden` is true.
    /// - Set any empty string to None.
    /// - Set any empty list to None.
    /// - Set any default values to None.
    #[must_use]
    pub fn cleaned(&self) -> Self {
        let mut cleaned = self.clone();

        // Set any empty string to None.
        if let Some(affected_body_parts_by_category) =
            cleaned.affected_body_parts_by_category.clone()
            && affected_body_parts_by_category.is_empty()
        {
            cleaned.affected_body_parts_by_category = None;
        }

        // Set any empty string to None.
        if let Some(affected_body_parts_by_type) = cleaned.affected_body_parts_by_type.clone()
            && affected_body_parts_by_type.is_empty()
        {
            cleaned.affected_body_parts_by_type = None;
        }

        // Set any empty string to None.
        if let Some(affected_body_parts_by_token) = cleaned.affected_body_parts_by_token.clone()
            && affected_body_parts_by_token.is_empty()
        {
            cleaned.affected_body_parts_by_token = None;
        }

        // Set any empty string to None.
        if let Some(tags) = cleaned.tags.clone()
            && tags.is_empty()
        {
            cleaned.tags = None;
        }

        // Set any default values to None.
        if default_checks::is_zero_u8(cleaned.dwf_stretch) {
            cleaned.dwf_stretch = None;
        }

        cleaned
    }
}
