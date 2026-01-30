use crate::{
    Inorganic,
    tokens::ObjectType,
    traits::{RawObject, RawToken, ToRawFileString},
};

impl ToRawFileString for Inorganic {
    fn to_raw_file(&self) -> String {
        let mut file_lines: Vec<String> = Vec::new();

        file_lines.push(ObjectType::Inorganic.to_raw_token());
        file_lines.push(String::new());
        file_lines.push(format!("[CREATURE_VARIATION:{}]", self.get_identifier()));

        file_lines.join(":")
    }
}
