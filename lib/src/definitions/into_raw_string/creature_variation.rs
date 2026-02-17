use crate::{
    CreatureVariation,
    tokens::ObjectType,
    traits::{RawObject, RawToken, ToRawFileString},
};

impl ToRawFileString for CreatureVariation {
    fn to_raw_file(&self) -> String {
        let mut file_lines: Vec<String> = Vec::new();

        file_lines.push(ObjectType::CreatureVariation.to_raw_token());
        file_lines.push(String::new());
        file_lines.push(format!("[CREATURE_VARIATION:{}]", self.get_identifier()));

        for token in self.get_tags() {
            file_lines.push(format!(
                "\t[{}:{}]",
                token.0.get_key().unwrap_or_default(),
                token.1
            ));
        }

        file_lines.join("\n") + "\n"
    }
}
