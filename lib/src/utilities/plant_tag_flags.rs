use crate::tags::PlantTag;

impl PlantTag {
    pub const FLAG_TOKENS: [&PlantTag; 5] = [
        &PlantTag::Dry,
        &PlantTag::Evil,
        &PlantTag::Good,
        &PlantTag::Savage,
        &PlantTag::Wet,
    ];
}
