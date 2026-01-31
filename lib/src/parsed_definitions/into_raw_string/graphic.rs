use crate::{
    Graphic,
    tokens::ObjectType,
    traits::{RawObject, RawToken, ToRawFileString},
};

impl ToRawFileString for Graphic {
    fn to_raw_file(&self) -> String {
        let mut file_lines: Vec<String> = Vec::new();

        file_lines.push(ObjectType::Graphics.to_raw_token());
        file_lines.push(String::new());
        file_lines.push(format!("[GRAPHIC:{}]", self.get_identifier()));

        file_lines.join("\n") + "\n"
    }
}
