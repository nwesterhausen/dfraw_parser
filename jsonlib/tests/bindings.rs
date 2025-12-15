use specta::TypeCollection;
use specta_typescript::{self, Typescript};

#[test]
#[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
fn generate_ts_bindings() {
    // get our current working directory
    let cwd = std::env::current_dir().expect("Failed to get current working directory");
    // set lib/bindings/AllBindings.d.ts as the output file
    let output_dir = cwd.join("bindings");
    // make sure output dir exists
    std::fs::create_dir_all(&output_dir).expect("Failed to create output directory");
    eprintln!("Output dir: {:?}", &output_dir);

    let exporter = Typescript::default().bigint(specta_typescript::BigIntExportBehavior::String);
    let mut types = TypeCollection::default();
    types
        .register::<dfraw_parser::tags::BiomeTag>()
        .register::<dfraw_parser::tags::CasteTag>()
        .register::<dfraw_parser::tags::ColorModificationTag>()
        .register::<dfraw_parser::tags::ConditionTag>()
        .register::<dfraw_parser::tags::CreatureTag>()
        .register::<dfraw_parser::tags::CreatureEffectTag>()
        .register::<dfraw_parser::tags::CreatureEffectPropertyTag>()
        .register::<dfraw_parser::tags::CreatureVariationTag>()
        .register::<dfraw_parser::tags::CreatureVariationRuleTag>()
        .register::<dfraw_parser::tags::EntityTag>()
        .register::<dfraw_parser::tags::EnvironmentClassTag>()
        .register::<dfraw_parser::tags::FuelTypeTag>()
        .register::<dfraw_parser::tags::GaitModifierTag>()
        .register::<dfraw_parser::tags::GaitTypeTag>()
        .register::<dfraw_parser::tags::GraphicTypeTag>()
        .register::<dfraw_parser::tags::GrowthTag>()
        .register::<dfraw_parser::tags::InclusionTypeTag>()
        .register::<dfraw_parser::tags::InorganicTag>()
        .register::<dfraw_parser::tags::MaterialPropertyTag>()
        .register::<dfraw_parser::tags::MaterialStateTag>()
        .register::<dfraw_parser::tags::MaterialTypeTag>()
        .register::<dfraw_parser::tags::MaterialUsageTag>()
        .register::<dfraw_parser::tags::ModificationTag>()
        .register::<dfraw_parser::tags::PlantTag>()
        .register::<dfraw_parser::tags::PlantGraphicTemplateTag>()
        .register::<dfraw_parser::tags::PlantGrowthTag>()
        .register::<dfraw_parser::tags::PlantGrowthTypeTag>()
        .register::<dfraw_parser::tags::PlantPartTag>()
        .register::<dfraw_parser::tags::PositionTag>()
        .register::<dfraw_parser::tags::SeasonTag>()
        .register::<dfraw_parser::tags::SelectCreatureRuleTag>()
        .register::<dfraw_parser::tags::ShrubTag>()
        .register::<dfraw_parser::tags::SyndromeTag>()
        .register::<dfraw_parser::tags::TilePageTag>()
        .register::<dfraw_parser::tags::TreeTag>()
        .register::<dfraw_parser::tags::TwigPlacementTag>()
        .register::<dfraw_parser::BodySize>()
        .register::<dfraw_parser::Caste>()
        .register::<dfraw_parser::Color>()
        .register::<dfraw_parser::Creature>()
        .register::<dfraw_parser::CreatureEffect>()
        .register::<dfraw_parser::CreatureVariation>()
        .register::<dfraw_parser::CustomGraphicExtension>()
        .register::<dfraw_parser::Dimensions>()
        .register::<dfraw_parser::Entity>()
        .register::<dfraw_parser::Gait>()
        .register::<dfraw_parser::Graphic>()
        .register::<dfraw_parser::InfoFile>()
        .register::<dfraw_parser::Inorganic>()
        .register::<dfraw_parser::Material>()
        .register::<dfraw_parser::MaterialMechanics>()
        .register::<dfraw_parser::MaterialTemplate>()
        .register::<dfraw_parser::MechanicalProperties>()
        .register::<dfraw_parser::Milkable>()
        .register::<dfraw_parser::Name>()
        .register::<dfraw_parser::Plant>()
        .register::<dfraw_parser::PlantGrowth>()
        .register::<dfraw_parser::Position>()
        .register::<dfraw_parser::SeedMaterial>()
        .register::<dfraw_parser::SelectCreature>()
        .register::<dfraw_parser::Shrub>()
        .register::<dfraw_parser::SpriteGraphic>()
        .register::<dfraw_parser::SpriteLayer>()
        .register::<dfraw_parser::StateNames>()
        .register::<dfraw_parser::SteamData>()
        .register::<dfraw_parser::Syndrome>()
        .register::<dfraw_parser::Temperatures>()
        .register::<dfraw_parser::Tile>()
        .register::<dfraw_parser::TilePage>()
        .register::<dfraw_parser::Tree>()
        .register::<dfraw_parser::metadata::ObjectType>()
        .register::<dfraw_parser::metadata::ParserOptions>()
        .register::<dfraw_parser::metadata::RawModuleLocation>()
        .register::<dfraw_parser::metadata::RawMetadata>()
        .register::<dfraw_parser::metadata::TagComplexity>();

    exporter
        .export_to(output_dir.join("DFRawParser.d.ts"), &types)
        .expect("Failed to export types");
}
