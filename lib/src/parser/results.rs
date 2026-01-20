
use uuid::Uuid;

use crate::{ModuleInfo, traits::RawObject};

/// A parsing result that contains the parsed raws and info files.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ParseResult {
    /// The parsed raw objects.
    pub raws: Vec<Box<dyn RawObject>>,
    /// The parsed module info files.
    pub modules: Vec<ModuleInfo>,
}

impl ParseResult {
    /// Get a subset of raws by a `ModuleInfo`'s `object_id`.
    pub fn get_raws_by_module_id(&self, module_id: Uuid) -> Vec<&dyn RawObject> {
        self.raws
            .iter()
            .filter(|r| r.get_module_object_id() == module_id)
            .map(|r| r.as_ref()) // dereference &Box<T> into &T
            .collect()
    }
    /// Get the subset of raws belonging to a specific module.
    pub fn get_raws_for_module(&self, module: &ModuleInfo) -> Vec<&dyn RawObject> {
        self.get_raws_by_module_id(module.get_object_id())
    }
}
