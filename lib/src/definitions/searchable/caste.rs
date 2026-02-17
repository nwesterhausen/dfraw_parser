use crate::{Caste, tokens::CasteToken, traits::Searchable};

impl Searchable for Caste {
    fn get_all_names(&self) -> Vec<String> {
        self.tokens
            .iter()
            .flat_map(|token| {
                match token {
                    CasteToken::Name { name } => name.as_vec(),
                    CasteToken::BabyName { name } => name.as_vec(),
                    CasteToken::ChildName { name } => name.as_vec(),
                    // Ignore all other tokens
                    _ => Vec::new(),
                }
            })
            .collect()
    }

    fn get_all_descriptions(&self) -> Vec<String> {
        vec![
            self.tokens
                .iter()
                .find_map(|token| match token {
                    CasteToken::Description { description } => Some(description.clone()),
                    _ => None,
                })
                .unwrap_or_default(),
        ]
    }
}
