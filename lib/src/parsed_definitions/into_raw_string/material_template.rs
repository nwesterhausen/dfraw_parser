use crate::{
    MaterialTemplate,
    tokens::ObjectType,
    traits::{RawObject, RawToken, ToRawFileString},
};

impl ToRawFileString for MaterialTemplate {
    fn to_raw_file(&self) -> String {
        let mut file_lines: Vec<String> = Vec::new();

        file_lines.push(ObjectType::MaterialTemplate.to_raw_token());
        file_lines.push(String::new());
        file_lines.push(format!("[MATERIAL_TEMPLATE:{}]", self.get_identifier()));

        file_lines.join("\n") + "\n"
    }
}
