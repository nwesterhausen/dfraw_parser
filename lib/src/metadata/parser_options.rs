use std::path::{Path, PathBuf};

use strum::IntoEnumIterator;

use crate::{
    metadata::{LocationHelper, RawModuleLocation},
    tags::ObjectType,
};

/// # Parsing Options
///
/// Specify what to parse and where to parse it from.
///
/// ## Parsing `info.txt` vs the raw files
///
/// There are two main parsing functions: `parse` and `parse_module_info_files`.
///
/// Both use the same options struct, but they use it in different ways.
///
/// When calling `parse`, the `ParserOptions` struct is used to specify what raws to parse and where to parse them from.
/// Any specified `raw_modules_to_parse` will not be parsed in the `parse` function, and the only items parsed in the
/// `parse_module_info_files` function are the `module_info_files_to_parse`.
///
/// ## Example
///
/// ```rust
/// use std::path::PathBuf;
/// use dfraw_parser::metadata::{ParserOptions, RawModuleLocation};
/// use dfraw_parser::tags::ObjectType;
/// use dfraw_parser::traits::RawObject;
///
/// let mut options = ParserOptions::new();
/// options.add_location_to_parse(RawModuleLocation::Vanilla);
/// // Clear the default object types
/// options.set_object_types_to_parse(vec![]);
/// // Add back in the ones we want
/// options.add_object_type_to_parse(ObjectType::Creature);
/// options.add_object_type_to_parse(ObjectType::CreatureVariation);
/// // Include the metadata with the parsed raws
/// options.attach_metadata_to_raws();
///
/// // Then you could parse the raws and info.txt files
/// // let parsed_raws = dfraw_parser::parse(&options);
///```
///
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ParserOptions {
    /// Whether to attach a metadata field to the raws.
    /// If true, all raws will have a `metadata` field which shows information about the
    /// raw file, its path, its module, and its parent directory.
    ///
    /// Default: false.
    pub attach_metadata_to_raws: bool,
    /// Whether to skip the "copy tags from" resolution step.
    /// If true, the creature will have a populated `copy_tags_from` field instead.
    ///
    /// Default: false.
    pub skip_apply_copy_tags_from: bool,
    /// Whether to skip the apply "creature variations" resolution step.
    /// When this is true, it will just leave the variations attached to the creature
    /// in a `creature_variations` field.
    /// If false, it will modify the creature data to include the variations.
    ///
    /// Note: This is currently not implemented.
    ///
    /// Default: false.
    pub skip_apply_creature_variations: bool,
    /// What types of raws to parse. If this is left empty, all parsable raws will be parsed.
    ///
    /// Default: `[Creature, CreatureVariation, Entity, Plant, Inorganic, MaterialTemplate, Graphics, TilePage]`
    pub object_types_to_parse: Vec<ObjectType>,
    /// What locations to parse raws from. If this is left empty, no locations will be parsed.
    ///
    /// Setting locations to parse requires a valid `dwarf_fortress_directory` to be set.
    ///
    /// Default: None
    pub locations_to_parse: Vec<RawModuleLocation>,
    /// The paths to the locations used for parsing: the Dwarf Fortress installation directory and the
    /// Dwarf Fortress user data directory.
    ///
    /// This can be automatically gathered or explicitly set.
    ///
    /// Default: Attempted to be automatically gathered.
    pub locations: LocationHelper,
    /// Optionally specify one or more `legends_plus` exports to parse in addition to the raws.
    /// These exports include information about generated creatures which are not included in the
    /// raws.
    ///
    /// Default: None
    pub legends_exports_to_parse: Vec<PathBuf>,
    /// Optionally specify one or more raw files to parse directly. These should be the raw files
    /// themselves, not the containing directory.
    ///
    /// (e.g. `creature_standard.txt` in `data/vanilla/vanilla_creatures/objects/`)
    ///
    /// Note that these will be parsed in addition to the raws in the specified locations in the other
    /// options. That means that if you specify a raw file that is also in the vanilla raws, it will
    /// be parsed twice (if vanilla is in the locations to parse).
    ///
    /// Default: None
    pub raw_files_to_parse: Vec<PathBuf>,
    /// Optionally specify one or more raw modules to parse directly. These should be the module
    /// directories, not the info.txt file.
    ///
    /// (e.g. `vanilla_creatures` in `data/vanilla/`)
    ///
    /// Note that these will be parsed in addition to the raws in the specified locations in the other
    /// options. That means that if you specify a module that is also in the vanilla raws, it will
    /// be parsed twice (if vanilla is in the locations to parse).
    ///
    /// Default: None
    pub raw_modules_to_parse: Vec<PathBuf>,
    /// Optionally specify one or more module info files to parse directly. These should be the info.txt
    /// files themselves, not the containing directory.
    ///
    /// (e.g. `info.txt` in `data/vanilla/vanilla_creatures/`)
    ///
    /// Note that if you are calling the `parse` function, this will be ignored. This is only used
    /// when calling the `parse_module_info_files` function.
    pub module_info_files_to_parse: Vec<PathBuf>,
    /// Include a summary of what was parsed in the log.
    ///
    /// If running with `tauri`, this will emit a `PARSE_SUMMARY` event with the summary as well.
    ///
    /// Default: false
    pub log_summary: bool,
    /// Log warnings about the format of the info.txt file.
    ///
    /// Typically this includes non-integer "before version" tags or other format errors which Dwarf Fortress
    /// will ignore/do its best to parse. They tend to not prevent the module to work, but they are technically
    /// incorrectly formatted. This would mostly be useful for mod authors to check.
    ///
    /// Default: false
    pub include_warnings_for_info_file_format: bool,
}

impl Default for ParserOptions {
    fn default() -> Self {
        let all_object_types = ObjectType::iter().collect();
        Self {
            attach_metadata_to_raws: false,
            skip_apply_copy_tags_from: false,
            skip_apply_creature_variations: false,
            include_warnings_for_info_file_format: false,
            log_summary: false,
            object_types_to_parse: all_object_types,
            locations_to_parse: vec![],
            locations: LocationHelper::new(),
            legends_exports_to_parse: Vec::new(),
            raw_files_to_parse: Vec::new(),
            raw_modules_to_parse: Vec::new(),
            module_info_files_to_parse: Vec::new(),
        }
    }
}

impl ParserOptions {
    /// Creates a new `ParserOptions` struct with the default values.
    ///
    /// For `ParsingJob::ALL` or `ParsingJob::SingleLocation`, this should be the path to the dwarf fortress directory.
    ///
    /// For `ParsingJob::SingleModule`, this should be the path to the module (which includes the
    /// info.txt file).
    ///
    /// For `ParsingJob::SingleRaw`, this should be the path directly to the raw.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_dwarf_fortress_directory(&mut self, df_dir: &PathBuf) {
        match self.locations.set_df_directory(df_dir) {
            Ok(()) => (),
            Err(e) => tracing::error!("{e:?}"),
        };
    }

    pub fn set_user_data_directory(&mut self, user_data_dir: &PathBuf) {
        match self.locations.set_user_data_directory(user_data_dir) {
            Ok(()) => (),
            Err(e) => tracing::error!("{e:?}"),
        };
    }

    /// If applied, all raws will have a `metadata` field which shows information about the
    /// raw file, its path, its module, and its parent directory.
    pub fn attach_metadata_to_raws(&mut self) {
        self.attach_metadata_to_raws = true;
    }

    /// Skip the "copy tags from" resolution step.
    ///
    /// Default: true.
    pub fn skip_apply_copy_tags_from(&mut self) {
        self.skip_apply_copy_tags_from = true;
    }

    /// Skip the apply "creature variations" resolution step.
    ///
    /// Note: This is currently not implemented.
    pub fn skip_apply_creature_variations(&mut self) {
        self.skip_apply_creature_variations = true;
    }

    /// Sets what kind of raws to parse.
    /// The default value will parse all the raws that are currently supported:
    ///
    /// * `ObjectType::Creature`
    /// * `ObjectType::CreatureVariation`
    /// * `ObjectType::Entity`
    /// * `ObjectType::Plant`
    /// * `ObjectType::Inorganic`
    /// * `ObjectType::MaterialTemplate`
    /// * `ObjectType::Graphics`
    /// * `ObjectType::TilePage`
    ///
    /// Note: This will overwrite any previously set raws (e.g. those set by `add_raw_to_parse`). It
    /// also will discard the default set of target object types.
    pub fn set_object_types_to_parse(&mut self, raws_to_parse: Vec<ObjectType>) {
        self.object_types_to_parse = raws_to_parse;
    }

    /// Add a single object type to parse.
    ///
    /// Note: This is in addition to the default value or the value set by `set_raws_to_parse`.
    pub fn add_object_type_to_parse(&mut self, raw_to_parse: ObjectType) {
        // If it's already in the list, don't add it again.
        if self.object_types_to_parse.contains(&raw_to_parse) {
            return;
        }

        self.object_types_to_parse.push(raw_to_parse);
    }

    /// Include graphics and tile pages in the parse.
    ///
    /// This is a convenience function that is equivalent to:
    ///
    /// ```rust
    /// use dfraw_parser::metadata::ParserOptions;
    /// use dfraw_parser::tags::ObjectType;
    ///
    /// let mut options = ParserOptions::new();
    /// options.add_object_type_to_parse(ObjectType::Graphics);
    /// options.add_object_type_to_parse(ObjectType::TilePage);
    ///
    /// let mut options2 = ParserOptions::new();
    /// options2.include_graphics();
    ///
    /// assert_eq!(options, options2);
    /// ```
    ///
    /// Note: This is in addition to the default value or the value set by `set_raws_to_parse`.
    pub fn include_graphics(&mut self) {
        self.add_object_type_to_parse(ObjectType::Graphics);
        self.add_object_type_to_parse(ObjectType::TilePage);
    }

    /// Sets what locations to parse raws from.
    ///
    /// * `RawModuleLocation::Vanilla` will parse the vanilla raws.
    /// * `RawModuleLocation::InstalledMods` will parse the installed mods folder.
    /// * `RawModuleLocation::Mods` will parse the downloaded mods folder.
    ///
    /// If left unset, no locations will be parsed.
    ///
    /// Note: This will overwrite any previously set locations (e.g. those set by `add_location_to_parse`).
    pub fn set_locations_to_parse(&mut self, locations_to_parse: Vec<RawModuleLocation>) {
        self.locations_to_parse = locations_to_parse;
    }

    /// Sets what raw files to parse directly. These should be the raw files
    /// themselves, not the containing directory.
    ///
    /// (e.g. `creature_standard.txt` in `data/vanilla/vanilla_creatures/objects/`)
    ///
    /// Note: this will overwrite any previously set raws (e.g. those set by `add_raw_file_to_parse`).
    pub fn set_raw_files_to_parse(&mut self, raw_files_to_parse: Vec<PathBuf>) {
        self.raw_files_to_parse = raw_files_to_parse;
    }

    /// Sets what raw modules to parse directly. These should be the module
    /// directories, not the info.txt file.
    ///
    /// (e.g. `vanilla_creatures` in `data/vanilla/`)
    ///
    /// Note: this will overwrite any previously set modules (e.g. those set by `add_raw_module_to_parse`).
    pub fn set_raw_modules_to_parse(&mut self, raw_modules_to_parse: Vec<PathBuf>) {
        self.raw_modules_to_parse = raw_modules_to_parse;
    }

    /// Sets what module info files to parse directly. These should be the info.txt
    /// files themselves, not the containing directory.
    ///
    /// (e.g. `info.txt` in `data/vanilla/vanilla_creatures/`)
    ///
    /// Note: this will overwrite any previously set info files (e.g. those set by `add_module_info_file_to_parse`).
    pub fn set_module_info_files_to_parse(&mut self, module_info_files_to_parse: Vec<PathBuf>) {
        self.module_info_files_to_parse = module_info_files_to_parse;
    }

    /// Set what legends-plus exports to parse in addition to the raws. These exports include
    /// information about generated creatures which are not included in the raws.
    ///
    /// These should be the legends-plus exports themselves, not the containing directory. When
    /// exported from legends mode, the file is dumped in the root directory of the game.
    ///
    /// (e.g. `region12-000005-01-01-legends_plus.xml` in the Dwarf Fortress directory)
    ///
    /// Note: this will overwrite any previously set legends exports (e.g. those set by `add_legends_export_to_parse`).
    pub fn set_legends_exports_to_parse(&mut self, legends_exports_to_parse: Vec<PathBuf>) {
        self.legends_exports_to_parse = legends_exports_to_parse;
    }

    /// Optionally specify one or more `legends_plus` exports to parse in addition to the raws.
    ///
    /// These exports include information about generated creatures which are not included in the
    /// raws.
    ///
    /// Default: None
    pub fn add_legends_export_to_parse<P: AsRef<Path>>(&mut self, legends_export_to_parse: &P) {
        self.legends_exports_to_parse
            .push(legends_export_to_parse.as_ref().to_path_buf());
    }

    /// Optionally specify one or more raw files to parse directly. These should be the raw files
    ///
    /// (e.g. `creature_standard.txt` in `data/vanilla/vanilla_creatures/objects/`)
    ///
    /// Note that these will be parsed in addition to the raws in the specified locations in the other
    /// options. That means that if you specify a raw file that is also in the vanilla raws, it will
    /// be parsed twice (if vanilla is in the locations to parse).
    ///
    /// Default: None
    pub fn add_raw_file_to_parse<P: AsRef<Path>>(&mut self, raw_file_to_parse: &P) {
        self.raw_files_to_parse
            .push(raw_file_to_parse.as_ref().to_path_buf());
    }

    /// Optionally specify one or more raw modules to parse directly. These should be the module
    /// directories, not the info.txt file.
    ///
    /// (e.g. `vanilla_creatures` in `data/vanilla/`)
    ///
    /// Note that these will be parsed in addition to the raws in the specified locations in the other
    /// options. That means that if you specify a module that is also in the vanilla raws, it will
    /// be parsed twice (if vanilla is in the locations to parse).
    ///
    /// Default: None
    pub fn add_raw_module_to_parse<P: AsRef<Path>>(&mut self, raw_module_to_parse: &P) {
        self.raw_modules_to_parse
            .push(raw_module_to_parse.as_ref().to_path_buf());
    }

    /// Optionally specify one or more module info files to parse directly. These should be the info.txt
    /// files themselves, not the containing directory.
    ///
    /// (e.g. `info.txt` in `data/vanilla/vanilla_creatures/`)
    ///
    /// Note that if you are calling the `parse` function, this will be ignored. This is only used
    /// when calling the `parse_module_info_files` function.
    ///
    /// Default: None
    pub fn add_module_info_file_to_parse<P: AsRef<Path>>(&mut self, module_info_file_to_parse: &P) {
        self.module_info_files_to_parse
            .push(module_info_file_to_parse.as_ref().to_path_buf());
    }

    /// Include a summary of what was parsed in the log.
    ///
    /// If running with `tauri`, this will emit a `PARSE_SUMMARY` event with the summary as well.
    ///
    /// Default: false
    pub fn log_summary(&mut self) {
        self.log_summary = true;
    }

    /// Add a location to parse raws from.
    ///
    /// * `RawModuleLocation::Vanilla` will parse the vanilla raws.
    /// * `RawModuleLocation::InstalledMods` will parse the installed mods folder.
    /// * `RawModuleLocation::Mods` will parse the downloaded mods folder.
    ///
    /// Parsing locations requires a valid `dwarf_fortress_directory` to be set.
    pub fn add_location_to_parse(&mut self, location_to_parse: RawModuleLocation) {
        self.locations_to_parse.push(location_to_parse);
    }
}
