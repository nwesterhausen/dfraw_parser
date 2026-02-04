use crate::{
    SelectCreature,
    tokens::ObjectType,
    traits::{RawObject, RawToken, ToRawFileString},
};

impl ToRawFileString for SelectCreature {
    fn to_raw_file(&self) -> String {
        let mut file_lines: Vec<String> = Vec::new();

        file_lines.push(ObjectType::SelectCreature.to_raw_token());
        file_lines.push(String::new());
        file_lines.push(format!("[SELECT_CREATURE:{}]", self.get_identifier()));

        file_lines.join("\n") + "\n"
    }
}
