use crate::tokens::ModuleInfoToken;

/// A map of the raw module info tokens to their enum variant
pub static MODULE_INFO_TOKENS: phf::Map<&'static str, ModuleInfoToken> = phf::phf_map! {
    "NUMERIC_VERSION" => ModuleInfoToken::NumericVersion { version: 0 },
    "DISPLAYED_VERSION" => ModuleInfoToken::DisplayedVersion { version: String::new() },
    "EARLIEST_COMPATIBLE_NUMERIC_VERSION" => ModuleInfoToken::EarliestCompatibleNumericVersion { version: 0 },
    "EARLIEST_COMPATIBLE_DISPLAYED_VERSION" => ModuleInfoToken::EarliestCompatibleDisplayedVersion { version: String::new() },
    "AUTHOR" => ModuleInfoToken::Author { name: String::new() },
    "NAME" => ModuleInfoToken::Name { name: String::new() },
    "DESCRIPTION" => ModuleInfoToken::Description { description: String::new() },
    "REQUIRES_ID" => ModuleInfoToken::RequiresId { id: String::new() },
    "REQUIRES_ID_BEFORE_ME" => ModuleInfoToken::RequiresIdBeforeMe { id: String::new() },
    "REQUIRES_ID_AFTER_ME" => ModuleInfoToken::RequiresIdAfterMe { id: String::new() },
    "CONFLICTS_WITH_ID" => ModuleInfoToken::ConflictsWithId  { id: String::new() },
};
