//! State names for materials
use dfraw_parser_proc_macros::{Cleanable, IsEmpty};

/// Represents the name of a materials 3 states (solid, liquid, gas)
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    Default,
    specta::Type,
    PartialEq,
    Eq,
    IsEmpty,
    Cleanable,
)]
#[serde(rename_all = "camelCase", rename = "StateName")]
pub struct StateNames {
    solid: String,
    liquid: String,
    gas: String,
}

impl StateNames {
    /// Returns whether the name is empty
    ///
    /// # Returns
    ///
    /// * `true` if the name is empty, `false` otherwise.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.solid.is_empty() && self.liquid.is_empty() && self.gas.is_empty()
    }
    /// Sets the solid name
    ///
    /// # Arguments
    ///
    /// * `name` - The name to set
    pub fn set_solid(&mut self, name: &str) {
        self.solid = String::from(name);
    }
    /// Sets the liquid name
    ///
    /// # Arguments
    ///
    /// * `name` - The name to set
    pub fn set_liquid(&mut self, name: &str) {
        self.liquid = String::from(name);
    }
    /// Sets the gas name
    ///
    /// # Arguments
    ///
    /// * `name` - The name to set
    pub fn set_gas(&mut self, name: &str) {
        self.gas = String::from(name);
    }
    /// Adds a name from a value
    ///
    /// # Arguments
    ///
    /// * `value` - The value to add (e.g. `ALL_SOLID:STONE`)
    pub fn add_from_value(&mut self, value: &str) {
        // Split the value into a descriptor and value
        let split = value.split(':').collect::<Vec<&str>>();
        let tag_key = match split.first() {
            Some(v) => *v,
            _ => {
                return;
            }
        };
        let tag_value = match split.get(1) {
            Some(v) => *v,
            _ => {
                return;
            }
        };

        match tag_key {
            "ALL_SOLID" | "SOLID" => {
                self.set_solid(tag_value);
            }
            "LIQUID" => {
                self.set_liquid(tag_value);
            }
            "GAS" => {
                self.set_gas(tag_value);
            }
            "ALL" => {
                self.set_solid(tag_value);
                self.set_liquid(tag_value);
                self.set_gas(tag_value);
            }
            _ => (),
        }
    }
    /// Returns the state names as a vector of strings
    ///
    /// # Returns
    ///
    /// * `Vec<String>` - The state names as a vector of strings
    #[must_use]
    pub fn as_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();
        if !self.solid.is_empty() {
            vec.push(self.solid.clone());
        }
        if !self.liquid.is_empty() {
            vec.push(self.liquid.clone());
        }
        if !self.gas.is_empty() {
            vec.push(self.gas.clone());
        }
        vec
    }
}
