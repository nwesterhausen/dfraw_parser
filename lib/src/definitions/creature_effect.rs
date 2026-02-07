//! A module containing the `CreatureEffect` struct and its implementations.

use crate::tokens::CreatureEffectPropertyToken;
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
    #[serde(default)]
    affected_body_parts_by_category: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    affected_body_parts_by_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    affected_body_parts_by_token: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    tags: Option<Vec<CreatureEffectPropertyToken>>,

    start: u32,
    peak: u32,
    end: u32,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[serde(default)]
    dwf_stretch: Option<u8>,
}
