use std::path::PathBuf;

use dfraw_parser::metadata::{LocationHelper, RawModuleLocation};

#[test]
fn test_location_helper() {
    // Test the location helper functions
    let helper = LocationHelper::new();
    println!("{helper:?}");

    println!(
        "Installed Modes: {:?}",
        helper.get_path_for_location(RawModuleLocation::InstalledMods)
    );
    println!(
        "Vanilla Raws: {:?}",
        helper.get_path_for_location(RawModuleLocation::Vanilla)
    );
}

#[test]
fn test_with_manual_paths() {
    let mut helper = LocationHelper::new();
    helper.set_user_df_directory(PathBuf::from(
        "/Volumes/LaCie/Samples/Bay_12_Games/Dwarf Fortress",
    ));
    helper.set_df_directory(PathBuf::from("/Volumes/LaCie/Samples/Dwarf_Fortress"));
    println!("{helper:?}");
    println!(
        "Installed Modes: {:?}",
        helper.get_path_for_location(RawModuleLocation::InstalledMods)
    );
    println!(
        "Vanilla Raws: {:?}",
        helper.get_path_for_location(RawModuleLocation::Vanilla)
    );
}
