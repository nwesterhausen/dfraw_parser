/// The shape of a tissue
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    specta::Type,
    strum_macros::EnumIter,
)]
pub enum TissueShape {
    /// Regular "layer" tissue
    #[default]
    Layer,
    /// Can be spun into thread at a farmer's workshop. Edge attacks will pass right through the tissue.
    Strands,
    /// Edge attacks will pass right through the tissue
    Feathers,
    /// Tissue is scales
    Scales,
    /// Tissue is some custom type
    Custom,
}
