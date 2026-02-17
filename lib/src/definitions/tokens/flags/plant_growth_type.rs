use crate::tokens::PlantGrowthTypeToken;

impl PlantGrowthTypeToken {
    pub const FLAG_TOKENS: [&PlantGrowthTypeToken; 11] = [
        &PlantGrowthTypeToken::Cone,
        &PlantGrowthTypeToken::Eggs,
        &PlantGrowthTypeToken::Feathers,
        &PlantGrowthTypeToken::Flowers,
        &PlantGrowthTypeToken::Fruit,
        &PlantGrowthTypeToken::Leaves,
        &PlantGrowthTypeToken::Nut,
        &PlantGrowthTypeToken::Pod,
        &PlantGrowthTypeToken::SeedCatkins,
        &PlantGrowthTypeToken::PollenCatkins,
        &PlantGrowthTypeToken::PollenCone,
    ];
}
