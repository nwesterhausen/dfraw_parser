use crate::{Plant, traits::Searchable};

impl Searchable for Plant {
    fn get_all_names(&self) -> Vec<String> {
        self.get_names().iter().map(|s| (*s).into()).collect()
    }

    fn get_all_descriptions(&self) -> Vec<String> {
        self.get_pref_strings()
            .iter()
            .map(|s| (*s).into())
            .collect()
    }
}
