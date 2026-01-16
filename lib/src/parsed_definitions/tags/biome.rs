//! Tags which represent biomes.

/// An enum representing a biome.
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    specta::Type,
    strum_macros::EnumIter,
)]
pub enum BiomeTag {
    /// A mountain biome.
    Mountain,
    /// A mountainous biome
    Mountains,
    /// A glacier biome.
    Glacier,
    /// A tundra biome.
    Tundra,
    /// A temperate freshwater swamp
    SwampTemperateFreshwater,
    /// A temperate saltwater swamp
    SwampTemperateSaltwater,
    /// A temperate freshwater marsh
    MarshTemperateFreshwater,
    /// A temperate saltwater marsh
    MarshTemperateSaltwater,
    /// A tropical freshwater swamp
    SwampTropicalFreshwater,
    /// A tropical saltwater swamp
    SwampTropicalSaltwater,
    /// A mangrove swamp
    SwampMangrove,
    /// A tropical freshwater marsh
    MarshTropicalFreshwater,
    /// A tropical saltwater marsh
    MarshTropicalSaltwater,
    /// A taiga forest
    ForestTaiga,
    /// A taiga
    Taiga,
    /// A temperate conifer forest
    ForestTemperateConifer,
    /// A temperate broadleaf forest
    ForestTemperateBroadleaf,
    /// A tropical conifer forest
    ForestTropicalConifer,
    /// A tropical broadleaf forest
    ForestTropicalDryBroadleaf,
    /// A tropical moist broadleaf forest
    ForestTropicalMoistBroadleaf,
    /// A temperate grassland
    GrasslandTemperate,
    /// A temperate savanna
    SavannaTemperate,
    /// A temperate shrubland
    ShrublandTemperate,
    /// A tropical grassland
    GrasslandTropical,
    /// A tropical savanna
    SavannaTropical,
    /// A tropical shrubland
    ShrublandTropical,
    /// A badland desert
    DesertBadland,
    /// A rocky desert
    DesertRock,
    /// A sandy desert
    DesertSand,
    /// A tropical ocean
    OceanTropical,
    /// A temperate ocean
    OceanTemperate,
    /// An arctic ocean
    OceanArctic,
    /// A temperate freshwater pool
    PoolTemperateFreshwater,
    /// A temperate brackishwater pool
    PoolTemperateBrackishwater,
    /// A temperate saltwater pool
    PoolTemperateSaltwater,
    /// A tropical freshwater pool
    PoolTropicalFreshwater,
    /// A tropical brackishwater pool
    PoolTropicalBrackishwater,
    /// A tropical saltwater pool
    PoolTropicalSaltwater,
    /// A temperate freshwater lake
    LakeTemperateFreshwater,
    /// A temperate brackishwater lake
    LakeTemperateBrackishwater,
    /// A temperate saltwater lake
    LakeTemperateSaltwater,
    /// A tropical freshwater lake
    LakeTropicalFreshwater,
    /// A tropical brackishwater lake
    LakeTropicalBrackishwater,
    /// A tropical saltwater lake
    LakeTropicalSaltwater,
    /// A temperate freshwater river
    RiverTemperateFreshwater,
    /// A temperate brackishwater river
    RiverTemperateBrackishwater,
    /// A temperate saltwater river
    RiverTemperateSaltwater,
    /// A tropical freshwater river
    RiverTropicalFreshwater,
    /// A tropical brackishwater river
    RiverTropicalBrackishwater,
    /// A tropical saltwater river
    RiverTropicalSaltwater,
    /// A subterranean freshwater source
    SubterraneanWater,
    /// A subterranean chasm
    SubterraneanChasm,
    /// A subterranean magma pool
    SubterraneanLava,
    /// All the main biomes
    AllMain,
    /// Any land biome
    AnyLand,
    /// Any ocean biome
    AnyOcean,
    /// Any lake biome
    AnyLake,
    /// Any temperate lake biome
    AnyTemperateLake,
    /// Any tropical lake biome
    AnyTropicalLake,
    /// Any river
    AnyRiver,
    /// Any temperate river
    AnyTemperateRiver,
    /// Any tropical river
    AnyTropicalRiver,
    /// Any pool
    AnyPool,
    /// Any non-freezing biome
    NotFreezing,
    /// Any temperate biome
    AnyTemperate,
    /// Any tropical biome
    AnyTropical,
    /// Any forest biome
    AnyForest,
    /// Any shrubland biome
    AnyShrubland,
    /// Any grassland biome
    AnyGrassland,
    /// Any savanna biome
    AnySavanna,
    /// Any temperate forest biome
    AnyTemperateForest,
    /// Any tropical forest biome
    AnyTropicalForest,
    /// Any temperate broadleaf forest biome
    AnyTemperateBroadleaf,
    /// Any tropical broadleaf forest biome
    AnyTropicalBroadleaf,
    /// Any wetland biome
    AnyWetland,
    /// Any temperate wetland biome
    AnyTemperateWetland,
    /// Any tropical wetland biome
    AnyTropicalWetland,
    /// Any tropical marsh biome
    AnyTropicalMarsh,
    /// Any temperate marsh biome
    AnyTemperateMarsh,
    /// Any tropical swamp biome
    AnyTropicalSwamp,
    /// Any temperate swamp biome
    AnyTemperateSwamp,
    /// Any desert biome
    AnyDesert,
    /// An unknown token
    #[default]
    Unknown,
}

impl std::fmt::Display for BiomeTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mountain => write!(f, "Mountain"),
            Self::Mountains => write!(f, "Mountains"),
            Self::Glacier => write!(f, "Glacier"),
            Self::Tundra => write!(f, "Tundra"),
            Self::SwampTemperateFreshwater => write!(f, "Temperate Freshwater Swamp"),
            Self::SwampTemperateSaltwater => write!(f, "Temperate Saltwater Swamp"),
            Self::MarshTemperateFreshwater => write!(f, "Temperate Freshwater Marsh"),
            Self::MarshTemperateSaltwater => write!(f, "Temperate Saltwater Marsh"),
            Self::SwampTropicalFreshwater => write!(f, "Tropical Freshwater Swamp"),
            Self::SwampTropicalSaltwater => write!(f, "Tropical Saltwater Swamp"),
            Self::SwampMangrove => write!(f, "Mangrove Swamp"),
            Self::MarshTropicalFreshwater => write!(f, "Tropical Freshwater Marsh"),
            Self::MarshTropicalSaltwater => write!(f, "Tropical Saltwater Marsh"),
            Self::ForestTaiga => write!(f, "Taiga Forest"),
            Self::Taiga => write!(f, "Taiga"),
            Self::ForestTemperateConifer => write!(f, "Temperate Coniferous Forest"),
            Self::ForestTemperateBroadleaf => write!(f, "Temperate Broadleaf Forest"),
            Self::ForestTropicalConifer => write!(f, "Tropical Coniferous Forest"),
            Self::ForestTropicalDryBroadleaf => write!(f, "Tropical Dry Broadleaf Forest"),
            Self::ForestTropicalMoistBroadleaf => write!(f, "Tropical Moist Broadleaf Forest"),
            Self::GrasslandTemperate => write!(f, "Temperate Grassland"),
            Self::SavannaTemperate => write!(f, "Temperate Savanna"),
            Self::ShrublandTemperate => write!(f, "Temperate Shrubland"),
            Self::GrasslandTropical => write!(f, "Tropical Grassland"),
            Self::SavannaTropical => write!(f, "Tropical Savanna"),
            Self::ShrublandTropical => write!(f, "Tropical Shrubland"),
            Self::DesertBadland => write!(f, "Badlands"),
            Self::DesertRock => write!(f, "Rocky Wasteland"),
            Self::DesertSand => write!(f, "Sand Desert"),
            Self::OceanTropical => write!(f, "Tropical Ocean"),
            Self::OceanTemperate => write!(f, "Temperate Ocean"),
            Self::OceanArctic => write!(f, "Arctic Ocean"),
            Self::PoolTemperateFreshwater => write!(f, "Temperate Freshwater Pool"),
            Self::PoolTemperateBrackishwater => write!(f, "Temperate Brackish Pool"),
            Self::PoolTemperateSaltwater => write!(f, "Temperate Saltwater Pool"),
            Self::PoolTropicalFreshwater => write!(f, "Tropical Freshwater Pool"),
            Self::PoolTropicalBrackishwater => write!(f, "Tropical Brackish Pool"),
            Self::PoolTropicalSaltwater => write!(f, "Tropical Saltwater Pool"),
            Self::LakeTemperateFreshwater => write!(f, "Temperate Freshwater Lake"),
            Self::LakeTemperateBrackishwater => write!(f, "Temperate Brackish Lake"),
            Self::LakeTemperateSaltwater => write!(f, "Temperate Saltwater Lake"),
            Self::LakeTropicalFreshwater => write!(f, "Tropical Freshwater Lake"),
            Self::LakeTropicalBrackishwater => write!(f, "Tropical Brackish Lake"),
            Self::LakeTropicalSaltwater => write!(f, "Tropical Saltwater Lake"),
            Self::RiverTemperateFreshwater => write!(f, "Temperate Freshwater River"),
            Self::RiverTemperateBrackishwater => write!(f, "Temperate Brackish River"),
            Self::RiverTemperateSaltwater => write!(f, "Temperate Saltwater River"),
            Self::RiverTropicalFreshwater => write!(f, "Tropical Freshwater River"),
            Self::RiverTropicalBrackishwater => write!(f, "Tropical Brackish River"),
            Self::RiverTropicalSaltwater => write!(f, "Tropical Saltwater River"),
            Self::SubterraneanWater => write!(f, "Underground caverns (in water)"),
            Self::SubterraneanChasm => write!(f, "Underground caverns (out of water)"),
            Self::SubterraneanLava => write!(f, "Magma sea"),
            Self::AllMain => write!(
                f,
                "All biomes excluding pools, rivers, and underground features"
            ),
            Self::AnyLand => write!(f, "All main biomes excluding oceans and lakes"),
            Self::AnyOcean => write!(f, "All ocean biomes"),
            Self::AnyLake => write!(f, "All lake biomes"),
            Self::AnyTemperateLake => write!(f, "All temperate lake biomes"),
            Self::AnyTropicalLake => write!(f, "All tropical lake biomes"),
            Self::AnyRiver => write!(f, "All river biomes"),
            Self::AnyTemperateRiver => write!(f, "All temperate river biomes"),
            Self::AnyTropicalRiver => write!(f, "All tropical river biomes"),
            Self::AnyPool => write!(f, "All pool biomes"),
            Self::NotFreezing => {
                write!(f, "All land biomes excluding Mountain, Glacier, and Tundra")
            }
            Self::AnyTemperate => write!(
                f,
                "All Temperate land biomes - marshes, swamps, forests, grassland, savanna, and shrubland"
            ),
            Self::AnyTropical => write!(
                f,
                "All Tropical land biomes - marshes, swamps (including Mangrove), forests, grassland, savanna, and shrubland"
            ),
            Self::AnyForest => write!(f, "All Forest biomes (excluding Taiga)"),
            Self::AnyShrubland => write!(f, "Temperate and Tropical Shrubland"),
            Self::AnyGrassland => write!(f, "Temperate and Tropical Grassland"),
            Self::AnySavanna => write!(f, "Temperate and Tropical Savanna"),
            Self::AnyTemperateForest => write!(f, "Temperate Coniferous and Broadleaf Forests"),
            Self::AnyTropicalForest => {
                write!(f, "Tropical Coniferous and Dry/Moist Broadleaf Forests")
            }
            Self::AnyTemperateBroadleaf => write!(
                f,
                "Temperate Broadleaf Forest, Grassland/Savanna/Shrubland, Swamps, and Marshes"
            ),
            Self::AnyTropicalBroadleaf => write!(
                f,
                "Tropical Dry/Moist Broadleaf Forest, Grassland/Savanna/Shrubland, Swamps (including Mangrove), and Marshes"
            ),
            Self::AnyWetland => write!(f, "All swamps and marshes"),
            Self::AnyTemperateWetland => write!(f, "All temperate swamps and marshes"),
            Self::AnyTropicalWetland => write!(f, "All tropical swamps and marshes"),
            Self::AnyTropicalMarsh => write!(f, "All tropical marshes"),
            Self::AnyTemperateMarsh => write!(f, "All temperate marshes"),
            Self::AnyTropicalSwamp => write!(f, "All tropical swamps (including Mangrove)"),
            Self::AnyTemperateSwamp => write!(f, "All temperate swamps"),
            Self::AnyDesert => write!(f, "Badlands, Rocky Wasteland, and Sand Desert"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
