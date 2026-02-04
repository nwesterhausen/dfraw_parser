use crate::{Creature, tokens::CreatureToken, traits::Searchable};

impl Searchable for Creature {
    fn get_all_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self
            .tokens
            .iter()
            .flat_map(|token| {
                match token {
                    CreatureToken::Name { name }
                    | CreatureToken::GeneralBabyName { name }
                    | CreatureToken::GeneralChildName { name } => name.as_vec(),
                    // Ignore all other tokens
                    _ => Vec::new(),
                }
            })
            .collect();

        for caste in &self.castes {
            names.extend(caste.get_all_names())
        }

        names
    }

    fn get_all_descriptions(&self) -> Vec<String> {
        self.castes
            .iter()
            .flat_map(|caste| caste.get_all_descriptions())
            .collect()
    }
}
