use crate::{InfoFile, traits::RawObjectfo};

/// A parsing result that contains the parsed raws and info files.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ParseResult {
    /// The parsed raw objects.
    pub raws: Vec<Box<dyn RawObject>>,
    /// The parsed module info files.
    pub info_files: Vec<ModuleInfo>,
}
