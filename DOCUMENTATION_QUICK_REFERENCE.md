# Documentation Quick Reference

A quick reference guide for documenting public functions in dfraw_parser.

## Common Patterns

### Simple Getter

```rust
/// Returns the identifier of this object.
///
/// The identifier is the unique name used in raw files.
#[must_use]
pub fn get_identifier(&self) -> &str {
    &self.identifier
}
```

### Getter Returning Option

```rust
/// Returns the description of this object, if available.
///
/// The description is the text shown in-game when examining this object.
#[must_use]
pub fn get_description(&self) -> Option<&str> {
    self.description.as_deref()
}
```

### Method with Parameters

```rust
/// Adds a tag to this object.
///
/// # Arguments
///
/// * `tag` - The tag to add
///
/// # Examples
///
/// ```
/// # use dfraw_parser::Creature;
/// let mut creature = Creature::new("DOG");
/// creature.add_tag(tag);
/// ```
pub fn add_tag(&mut self, tag: Tag) {
    self.tags.push(tag);
}
```

### Function Returning Result

```rust
/// Parses a raw file and returns the parsed objects.
///
/// # Arguments
///
/// * `path` - Path to the raw file
///
/// # Returns
///
/// Returns a `ParseResult` containing all parsed objects.
///
/// # Errors
///
/// Returns `ParserError::IoError` if the file cannot be read.
/// Returns `ParserError::InvalidFormat` if the file format is invalid.
///
/// # Examples
///
/// ```no_run
/// use dfraw_parser::parse_file;
/// use std::path::Path;
///
/// let result = parse_file(Path::new("creature.txt"))?;
/// # Ok::<(), dfraw_parser::ParserError>(())
/// ```
pub fn parse_file(path: &Path) -> Result<ParseResult, ParserError> {
    // implementation
}
```

### Associated Function (Constructor)

```rust
/// Creates a new creature with the given identifier.
///
/// # Arguments
///
/// * `identifier` - The unique identifier for this creature
///
/// # Examples
///
/// ```
/// use dfraw_parser::Creature;
///
/// let creature = Creature::new("DOG");
/// assert_eq!(creature.get_identifier(), "DOG");
/// ```
#[must_use]
pub fn new(identifier: impl Into<String>) -> Self {
    Self {
        identifier: identifier.into(),
        ..Default::default()
    }
}
```

### Trait Definition

```rust
/// Provides search functionality for parsed objects.
///
/// Types implementing this trait can be searched by generating
/// search strings from their properties.
pub trait Searchable {
    /// Returns a vector of searchable strings.
    ///
    /// These strings typically include names, descriptions, and other
    /// searchable attributes of the object.
    fn get_search_vec(&self) -> Vec<String>;
}
```

### Enum

```rust
/// Represents the state of a material.
#[derive(Debug, Clone, Copy)]
pub enum MaterialState {
    /// Solid state (e.g., ice, stone)
    Solid,
    /// Liquid state (e.g., water, magma)
    Liquid,
    /// Gas state (e.g., steam, miasma)
    Gas,
}
```

### Module

```rust
//! Utilities for parsing and manipulating creature definitions.
//!
//! This module provides functions for working with creature raw files,
//! including parsing, validation, and transformation utilities.
//!
//! # Examples
//!
//! ```
//! use dfraw_parser::creatures;
//!
//! // Example usage
//! ```
```

### Const

```rust
/// Default maximum age for creatures in ticks.
///
/// This value is used when no specific max age is defined.
pub const DEFAULT_MAX_AGE: u32 = 1_000_000;
```

## Section Keywords

Use these sections in your documentation:

- `# Arguments` - Parameter descriptions
- `# Returns` - Return value description
- `# Errors` - Error conditions for Result types
- `# Panics` - When the function might panic
- `# Safety` - Safety requirements (unsafe functions only)
- `# Examples` - Usage examples

## Code Example Attributes

- `no_run` - Compiles but doesn't execute (needs external resources)
- `ignore` - Skip in documentation tests
- `should_panic` - Expected to panic
- `compile_fail` - Should fail to compile

```rust
/// ```no_run
/// // This needs actual game files
/// let result = parse_game_files();
/// ```
```

## Quick Tips

1. **First line is special** - Should be a complete sentence, ends with period
2. **Keep it concise** - Aim for clarity over verbosity
3. **Examples are valuable** - Show typical usage
4. **Document errors** - List what can go wrong
5. **Link to related items** - Use [backticks] for type references
6. **Test your examples** - Run `cargo test --doc`

## Linking to Types

Use square brackets to link to other types:

```rust
/// Returns a [`Creature`] object.
///
/// See also [`parse_file`] for parsing from files.
```

## Common Phrases for Dwarf Fortress Context

- "The identifier is the unique name used in raw files"
- "Measured in game ticks"
- "Specified in the raw file using [TAG:VALUE]"
- "This value affects how the game processes this object"
- "Returns `None` if not specified in the raw definition"

## Before Committing

Run these commands:

```bash
# Check for missing docs
cargo clippy --workspace -- -D missing_docs

# Test documentation examples
cargo test --doc

# View generated docs
cargo doc --no-deps --open
```
