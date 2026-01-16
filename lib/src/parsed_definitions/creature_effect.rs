//! A module containing the `CreatureEffect` struct and its implementations.

use crate::tags::CreatureEffectPropertyTag;
use dfraw_parser_proc_macros::{Cleanable, IsEmpty};

/// A creature effect.
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    Default,
    specta::Type,
    PartialEq,
    Eq,
    IsEmpty,
    Cleanable,
)]
#[serde(rename_all = "camelCase")]
pub struct CreatureEffect {
    severity: u32,
    probability: u8,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    affected_body_parts_by_category: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    affected_body_parts_by_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    affected_body_parts_by_token: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    tags: Option<Vec<CreatureEffectPropertyTag>>,

    start: u32,
    peak: u32,
    end: u32,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    dwf_stretch: Option<u8>,
}
