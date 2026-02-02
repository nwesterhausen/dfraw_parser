use uuid::Uuid;

use crate::{
    Inorganic,
    metadata::RawMetadata,
    raw_definitions::{ENVIRONMENT_CLASS_TOKENS, INCLUSION_TYPE_TOKENS, INORGANIC_TOKENS},
    tokens::{EnvironmentClassToken, InclusionTypeToken, InorganicToken, ObjectType},
    traits::RawObject,
};

#[typetag::serde]
impl RawObject for Inorganic {
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
    fn get_name(&self) -> &str {
        &self.identifier
    }
    fn get_metadata(&self) -> RawMetadata {
        self.metadata.clone()
    }
    fn get_type(&self) -> ObjectType {
        ObjectType::Inorganic
    }

    fn parse_tag(&mut self, key: &str, value: &str) {
        if INORGANIC_TOKENS.contains_key(key) {
            // For the inorganic tokens, we need to check for (and parse) the MetalOre, ThreadMetal, Environment, and EnvironmentSpecific tokens.
            let token = INORGANIC_TOKENS
                .get(key)
                .unwrap_or(&InorganicToken::Unknown);

            match token {
                InorganicToken::Environment => {
                    // Environment values are like this: "class:type:frequency"
                    let mut split = value.split(':');
                    // Determine class
                    self.environment_class = Some(
                        *ENVIRONMENT_CLASS_TOKENS
                            .get(split.next().unwrap_or(""))
                            .unwrap_or(&EnvironmentClassToken::None),
                    );
                    // Determine type
                    self.environment_inclusion_type = Some(
                        *INCLUSION_TYPE_TOKENS
                            .get(split.next().unwrap_or(""))
                            .unwrap_or(&InclusionTypeToken::None),
                    );
                    // Determine frequency
                    self.environment_inclusion_frequency =
                        Some(split.next().unwrap_or("0").parse::<u32>().unwrap_or(0));
                }
                InorganicToken::EnvironmentSpecific => {
                    if self.environment_class_specific.is_none() {
                        self.environment_class_specific = Some(Vec::new());
                    }
                    if let Some(environment_class_specific) = &mut self.environment_class_specific {
                        // Environment specific values are like this: "value"
                        environment_class_specific.push(String::from(value));
                    }
                }
                InorganicToken::MetalOre => {
                    if self.metal_ore_chance.is_none() {
                        self.metal_ore_chance = Some(Vec::new());
                    }

                    // Metal ore token values are like this: "metal:d100chance"
                    let mut split = value.split(':');
                    let metal = String::from(split.next().unwrap_or(""));
                    let chance = split.next().unwrap_or("0").parse::<u8>().unwrap_or(0);

                    if let Some(metal_ore_chance) = self.metal_ore_chance.as_mut() {
                        metal_ore_chance.push((metal, chance));
                    }
                }
                InorganicToken::ThreadMetal => {
                    if self.thread_metal_chance.is_none() {
                        self.thread_metal_chance = Some(Vec::new());
                    }

                    // Thread metal token values are like this: "metal:d100chance"
                    let mut split = value.split(':');
                    let metal = String::from(split.next().unwrap_or(""));
                    let chance = split.next().unwrap_or("0").parse::<u8>().unwrap_or(0);

                    if let Some(thread_metal_chance) = self.thread_metal_chance.as_mut() {
                        thread_metal_chance.push((metal, chance));
                    }
                }
                _ => {
                    self.add_tag(*token);
                }
            }

            return;
        }

        // Fall through any remaining tags to the material
        self.material.parse_tag(key, value);
    }
    fn get_object_id(&self) -> Uuid {
        self.object_id
    }
}
