use crate::{
    Creature,
    tokens::ObjectType,
    traits::{RawObject, RawToken, ToRawFileString},
};

impl ToRawFileString for Creature {
    fn to_raw_file(&self) -> String {
        let mut file_lines: Vec<String> = Vec::new();

        file_lines.push(ObjectType::Creature.to_raw_token());
        file_lines.push(String::new());
        file_lines.push(format!("[CREATURE:{}]", self.get_identifier()));

        for token in self.get_tokens() {
            file_lines.push(format!("\t{}", token.to_raw_token()));
        }

        for caste in self.get_castes() {
            file_lines.push(format!("\t[CASTE:{}]", caste.get_identifier()));
            caste.get_tags().iter().for_each(|token| {
                file_lines.push(format!("\t\t{}", token.to_raw_token()));
            });
        }

        file_lines.join("\n") + "\n"
    }
}
