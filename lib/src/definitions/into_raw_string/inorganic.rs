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
        file_lines.push(format!("[INORGANIC:{}]", self.get_identifier()));

        for token in self.get_tags() {
            file_lines.push(format!("\t{}", token.to_raw_token()));
        }

        file_lines.join("\n") + "\n"
    }
}
