use std::path::Path;

use crate::{
    InfoFile,
    metadata::{ParserOptions, RawMetadata, RawModuleLocation},
    tags::ObjectType,
};

pub fn legends_metadata(
    filepath: &Path,
    object_type: &ObjectType,
    options: &ParserOptions,
) -> RawMetadata {
    // Create a module info file
    #[allow(clippy::unwrap_used)]
    let file_name = filepath.file_name().unwrap().to_str().unwrap();
    #[allow(clippy::unwrap_used)]
    let parent_dir = filepath.parent().unwrap().to_str().unwrap();
    let mut module_info_file =
        InfoFile::new(file_name, RawModuleLocation::LegendsExport, parent_dir);
    module_info_file.set_module_name("Legends Export");
    RawMetadata::new(
        &module_info_file,
        object_type,
        file_name,
        &filepath,
        options.attach_metadata_to_raws,
    )
}
