# Documentation Improvement Suggestions for dfraw_parser

## Executive Summary

Analysis of the `dfraw_parser` library reveals **60 missing documentation items** that should be addressed to improve the developer experience and API usability. This document provides specific suggestions and a prioritized action plan.

## Current State Analysis

### Documentation Gaps by Category

| Category | Count | Priority |
|----------|-------|----------|
| Missing method docs | 48 | High |
| Missing associated constants | 4 | Medium |
| Missing module docs | 3 | High |
| Missing associated functions | 2 | Medium |
| Missing variant docs | 2 | Low |
| Missing trait docs | 1 | High |

### Files Requiring Attention

1. **lib/src/parsed_definitions/caste.rs** - 17 getter methods missing docs
2. **lib/src/utilities/mod.rs** - Module-level doc missing
3. **lib/src/traits/token_parser.rs** - Trait-level doc missing
4. **lib/src/parsed_definitions/custom_types/habit_count.rs** - 2 enum variants missing docs
5. **lib/src/parsed_definitions/custom_types/tile_char.rs** - 1 associated function missing doc
6. **lib/src/utilities/*_tag_flags.rs** - 4 associated constants missing docs

## Specific Recommendations

### 1. Add Module-Level Documentation

#### lib/src/utilities/mod.rs
**Current:** No module-level documentation
**Suggestion:** Add comprehensive module documentation

```rust
//! Utility functions and helpers for parsing Dwarf Fortress raw files.
//!
//! This module provides various utilities including:
//! - File operations for reading and writing raw files
//! - Directory lookup functions for Steam and user data directories
//! - Tag lookup tables for creatures, plants, entities, and more
//! - Flag constants for various game object types
//!
//! # Examples
//!
//! Finding the game installation path:
//! ```no_run
//! use dfraw_parser::utilities::find_game_path;
//!
//! if let Some(path) = find_game_path() {
//!     println!("Game found at: {:?}", path);
//! }
//! ```
```

#### lib/src/traits/mod.rs
**Current:** Has basic documentation ("Shared traits amongst various parsed objects.")
**Suggestion:** Expand with usage examples

```rust
//! Shared traits amongst various parsed objects.
//!
//! This module defines core traits implemented by parsed raw file objects:
//! - [`RawObject`] - Base trait for all parsed raw objects
//! - [`Searchable`] - Provides search string functionality
//! - [`TokenParser`] - Helper trait for parsing raw file tokens
//! - [`TagOperations`] - Operations for manipulating tags
//! - [`CreatureVariationRequirements`] - Handling creature variation requirements
//!
//! # Examples
//!
//! Using the Searchable trait:
//! ```
//! use dfraw_parser::traits::Searchable;
//! # use dfraw_parser::Creature;
//! 
//! # let creature = Creature::default();
//! let search_string = creature.get_search_vec();
//! ```
```

### 2. Add Trait Documentation

#### lib/src/traits/token_parser.rs
**Current:** No trait-level documentation
**Suggestion:** Add comprehensive trait documentation

```rust
//! Token parsing utilities for Dwarf Fortress raw file tokens.

/// Trait for parsing tokens from Dwarf Fortress raw files.
///
/// This trait provides helper methods for parsing values from raw file tokens,
/// with special handling for the "NONE" keyword and error cases.
///
/// # Examples
///
/// ```rust
/// use dfraw_parser::traits::TokenParser;
///
/// #[derive(Debug)]
/// struct MyParser;
///
/// impl TokenParser for MyParser {}
///
/// let parser = MyParser;
/// let value: Option<u32> = parser.parse_value("42");
/// assert_eq!(value, Some(42));
/// 
/// // "NONE" returns default value (0)
/// let none_value: Option<u32> = parser.parse_value("NONE");
/// assert_eq!(none_value, Some(0));
/// ```
pub trait TokenParser: Debug {
    // ... existing methods
}
```

### 3. Add Getter Method Documentation (Caste struct)

The `Caste` struct has 17 getter methods without documentation. Here's a pattern to follow:

```rust
/// Returns the identifier of this caste.
///
/// The identifier is the unique name used in the raw files to reference this caste.
#[must_use]
pub fn get_identifier(&self) -> &str {
    &self.identifier
}

/// Returns the description of this caste, if available.
///
/// This is the descriptive text shown in-game for creatures of this caste.
#[must_use]
pub fn get_description(&self) -> Option<&str> {
    self.description.as_deref()
}

/// Returns the pet value of this caste, if specified.
///
/// The pet value affects how desirable this creature is as a pet and influences
/// its trade value.
#[must_use]
pub fn get_pet_value(&self) -> Option<u32> {
    self.pet_value
}

/// Returns the age at which creatures of this caste are considered babies.
///
/// This value is specified in ticks (game time units).
#[must_use]
pub fn get_baby_age(&self) -> Option<u32> {
    self.baby
}

/// Returns the age at which creatures of this caste are considered children.
///
/// This value is specified in ticks (game time units).
#[must_use]
pub fn get_child_age(&self) -> Option<u32> {
    self.child
}

/// Returns the difficulty rating for this caste.
///
/// Higher values indicate more challenging creatures in arena mode or similar contexts.
#[must_use]
pub fn get_difficulty(&self) -> Option<u32> {
    self.difficulty
}

/// Returns the size of eggs laid by this caste, if applicable.
///
/// Measured in cubic centimeters (cm³).
#[must_use]
pub fn get_egg_size(&self) -> Option<u32> {
    self.egg_size
}

/// Returns the population ratio for this caste.
///
/// This determines the relative frequency of this caste in wild populations.
/// For example, a pop_ratio of 50 means this caste appears 50% of the time.
#[must_use]
pub fn get_pop_ratio(&self) -> Option<u32> {
    self.pop_ratio
}

/// Returns the clutch size range for this caste, if it lays eggs.
///
/// Returns a tuple of `[min, max]` eggs per clutch.
#[must_use]
pub fn get_clutch_size(&self) -> Option<[u32; 2]> {
    self.clutch_size
}

/// Returns the litter size range for this caste, if it gives live birth.
///
/// Returns a tuple of `[min, max]` offspring per litter.
#[must_use]
pub fn get_litter_size(&self) -> Option<[u32; 2]> {
    self.litter_size
}

/// Returns the maximum age range for this caste.
///
/// Returns a tuple of `[min, max]` age in game ticks. Creatures die of old age
/// within this range.
#[must_use]
pub fn get_max_age(&self) -> Option<[u32; 2]> {
    self.max_age
}

/// Returns the name used for baby creatures of this caste.
#[must_use]
pub fn get_baby_name(&self) -> Option<&Name> {
    self.baby_name.as_ref()
}

/// Returns the name used for child creatures of this caste.
#[must_use]
pub fn get_child_name(&self) -> Option<&Name> {
    self.child_name.as_ref()
}

/// Returns the name used for adult creatures of this caste.
#[must_use]
pub fn get_caste_name(&self) -> Option<&Name> {
    self.caste_name.as_ref()
}

/// Returns the tile graphics information for this caste.
///
/// This defines the character and colors used to display this caste in ASCII mode.
#[must_use]
pub fn get_tile(&self) -> Option<&Tile> {
    self.tile.as_ref()
}

/// Returns a slice of body sizes for this caste at different life stages.
///
/// Body size affects combat, carrying capacity, and butchering yields.
#[must_use]
pub fn get_body_sizes(&self) -> &[BodySize] {
    self.body_size.as_deref().unwrap_or(&[])
}

/// Returns a slice of creature classes this caste belongs to.
///
/// Creature classes are used for targeting by interactions, syndromes, and other effects.
#[must_use]
pub fn get_creature_classes(&self) -> &[String] {
    self.creature_class.as_deref().unwrap_or(&[])
}

/// Returns a slice of gaits (movement modes) available to this caste.
///
/// Examples include walking, crawling, flying, and swimming.
#[must_use]
pub fn get_gaits(&self) -> &[Gait] {
    self.gaits.as_deref().unwrap_or(&[])
}
```

### 4. Add Enum Variant Documentation

#### lib/src/parsed_definitions/custom_types/habit_count.rs

```rust
/// The 'HABIT_NUM' value which can be a number or "TEST_ALL"
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, specta::Type, Eq)]
pub enum HabitCount {
    /// Test all possible habit values
    TestAll,
    /// Test a specific number of habits
    Specific(u32),
}
```

### 5. Add Associated Function Documentation

#### lib/src/parsed_definitions/custom_types/tile_char.rs

```rust
impl TileCharacter {
    /// Creates a new `TileCharacter` with the default '?' character.
    ///
    /// # Examples
    ///
    /// ```
    /// use dfraw_parser::custom_types::TileCharacter;
    ///
    /// let tile_char = TileCharacter::new();
    /// assert_eq!(tile_char.value, '?');
    /// ```
    pub const fn new() -> Self {
        Self { value: '?' }
    }
}
```

### 6. Add Associated Constants Documentation

For the tag flags in utilities (e.g., `caste_tag_flags.rs`, `creature_tag_flags.rs`, etc.):

```rust
impl CasteTag {
    /// Array of all caste tags that represent boolean flags.
    ///
    /// These tags don't require additional parameters and are either present or absent.
    /// Used for efficient lookup and validation of flag-type tags.
    pub const FLAG_TOKENS: [&CasteTag; 163] = [
        // ... existing items
    ];
}
```

## Best Practices & Standards

### Documentation Format Guidelines

1. **First line should be a concise summary** (under 80 chars if possible)
2. **Add a blank line before detailed description**
3. **Include sections as needed:**
   - `# Arguments` - For function parameters
   - `# Returns` - For return values
   - `# Errors` - For possible error conditions
   - `# Examples` - For usage examples
   - `# Panics` - If the function can panic
   - `# Safety` - For unsafe code

4. **Examples should:**
   - Use `no_run` when actual execution isn't possible
   - Be self-contained when possible
   - Demonstrate typical usage patterns

### Example: Complete Function Documentation

```rust
/// Parses a raw file at the specified path.
///
/// This function reads and parses a single Dwarf Fortress raw file, extracting
/// all object definitions it contains.
///
/// # Arguments
///
/// * `raw_file_path` - Path to the raw file to parse
/// * `options` - Parser configuration options
///
/// # Returns
///
/// Returns a `FileParseResult` containing all parsed objects from the file.
///
/// # Errors
///
/// Returns `ParserError` if:
/// - The file cannot be read
/// - The file contains invalid UTF-8 or Latin-1 encoding
/// - The file structure is malformed
///
/// # Examples
///
/// ```no_run
/// use dfraw_parser::{parse_raw_file, ParserOptions};
/// use std::path::Path;
///
/// let path = Path::new("data/vanilla/objects/creature_standard.txt");
/// let options = ParserOptions::default();
/// 
/// match parse_raw_file(&path, &options) {
///     Ok(result) => println!("Parsed {} objects", result.creatures.len()),
///     Err(e) => eprintln!("Parse error: {}", e),
/// }
/// ```
pub fn parse_raw_file<P: AsRef<Path>>(
    raw_file_path: &P,
    options: &ParserOptions,
) -> Result<FileParseResult, ParserError> {
    // implementation
}
```

## Implementation Priority

### Phase 1: High Priority (Immediate Impact)
1. ✅ Add module documentation for `utilities` and `traits`
2. ✅ Add trait documentation for `TokenParser`
3. ✅ Document all 17 public getter methods in `Caste`

### Phase 2: Medium Priority (API Completeness)
4. ✅ Document associated constants in tag flag implementations
5. ✅ Document associated functions in custom types
6. ✅ Add examples to existing sparse documentation

### Phase 3: Quality Improvements
7. ✅ Add doc tests to verify examples compile
8. ✅ Create CONTRIBUTING.md with documentation guidelines
9. ✅ Consider upgrading `missing_docs` from `warn` to `deny`

## Enforcement Recommendations

### Current Configuration
```toml
[workspace.lints.rust]
missing_docs = "warn"  # Currently only warns
```

### Suggested Configuration (Gradual Approach)

**Option 1: Enforce immediately**
```toml
[workspace.lints.rust]
missing_docs = "deny"  # Fails compilation on missing docs
```

**Option 2: Gradual enforcement**
1. Fix all current issues first
2. Change to `deny` in a future PR
3. Prevent regression via CI checks

### CI Integration

Add to `.github/workflows/clippy.yml`:
```yaml
- name: Check for missing documentation
  run: cargo clippy --all-targets -- -D missing_docs
```

## Benefits of Implementation

1. **Better Developer Experience**: Clear documentation helps users understand the API quickly
2. **Reduced Support Burden**: Well-documented code reduces questions and issues
3. **IDE Support**: Better autocomplete and inline help in IDEs
4. **Generated Documentation**: `cargo doc` produces more useful output
5. **API Stability**: Documentation forces clearer thinking about public interfaces
6. **Onboarding**: New contributors can understand the codebase faster

## Summary Statistics

- **Total items needing documentation**: 60
- **Estimated effort**: 2-4 hours for initial documentation
- **High-value items**: 20 (modules, traits, core public functions)
- **Medium-value items**: 23 (getters, helper methods)
- **Low-value items**: 17 (enum variants, simple functions)

## Next Steps

1. Review and approve this improvement plan
2. Implement Phase 1 (high priority items)
3. Run `cargo doc --no-deps --open` to verify documentation quality
4. Add CI checks to prevent regression
5. Implement Phases 2 and 3 as time permits
