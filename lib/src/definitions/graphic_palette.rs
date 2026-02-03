//! Graphic palette definition
use dfraw_parser_proc_macros::IsEmpty;

/// A struct representing a Graphic object.
#[allow(clippy::module_name_repetitions)]
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    Default,
    specta::Type,
    PartialEq,
    Eq,
    IsEmpty,
)]
#[serde(rename_all = "camelCase")]
pub struct GraphicPalette {
    /// Name of the palette
    name: String,
    /// Relative file path to the palette file
    file: String,
    /// Default row of the palette
    default_row: u32,
}

impl GraphicPalette {
    #[must_use]
    pub fn new(new_name: &str) -> Self {
        Self {
            name: String::from(new_name),
            ..Default::default()
        }
    }
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
    pub fn set_file(&mut self, file_path: &str) {
        self.file = String::from(file_path);
    }
    pub fn get_file(&self) -> &str {
        self.file.as_str()
    }
    pub fn set_default_row(&mut self, row_num: u32) {
        self.default_row = row_num;
    }
    pub fn get_default_row(&self) -> u32 {
        self.default_row
    }
}
