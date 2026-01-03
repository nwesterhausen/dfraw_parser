use crate::tags::PlantGrowthTypeTag;

impl PlantGrowthTypeTag {
    pub const FLAG_TOKENS: [&PlantGrowthTypeTag; 11] = [
        &PlantGrowthTypeTag::Cone,
        &PlantGrowthTypeTag::Eggs,
        &PlantGrowthTypeTag::Feathers,
        &PlantGrowthTypeTag::Flowers,
        &PlantGrowthTypeTag::Fruit,
        &PlantGrowthTypeTag::Leaves,
        &PlantGrowthTypeTag::Nut,
        &PlantGrowthTypeTag::Pod,
        &PlantGrowthTypeTag::SeedCatkins,
        &PlantGrowthTypeTag::PollenCatkins,
        &PlantGrowthTypeTag::PollenCone,
    ];
}
