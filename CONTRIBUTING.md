# Contributing to dfraw_parser

Thank you for considering contributing to dfraw_parser! This document provides guidelines to help you contribute effectively.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Documentation Guidelines](#documentation-guidelines)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Pull Request Process](#pull-request-process)

## Code of Conduct

This project follows a standard code of conduct. Please be respectful and constructive in all interactions.

## How to Contribute

### Reporting Bugs

- Check if the issue already exists in the issue tracker
- Use a clear and descriptive title
- Describe the exact steps to reproduce the problem
- Include relevant code samples and error messages
- Specify your environment (OS, Rust version, DF version)

### Suggesting Enhancements

- Use a clear and descriptive title
- Provide a detailed description of the suggested enhancement
- Explain why this enhancement would be useful
- Include code examples if applicable

### Contributing Code

1. Fork the repository
2. Create a new branch (`git checkout -b feature/your-feature-name`)
3. Make your changes
4. Write or update tests as needed
5. Ensure all tests pass
6. Update documentation
7. Commit your changes with clear commit messages
8. Push to your fork
9. Open a pull request

## Development Setup

### Prerequisites

- Rust 1.75 or later
- Cargo
- Git

### Building the Project

```bash
# Clone the repository
git clone https://github.com/nwesterhausen/dfraw_parser.git
cd dfraw_parser

# Build all workspace members
cargo build --workspace

# Run tests
cargo test --workspace

# Check code with clippy
cargo clippy --workspace -- -D warnings
```

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for a specific package
cargo test -p dfraw_parser

# Run a specific test
cargo test test_name
```

## Documentation Guidelines

Good documentation is crucial for library usability. All public APIs must be documented.

### General Documentation Rules

1. **All public items must have documentation comments** (`///` or `//!`)
2. **First line should be a concise summary** (aim for < 80 characters)
3. **Add a blank line before detailed descriptions**
4. **Use proper Markdown formatting**
5. **Include examples for non-trivial functions**

### Documentation Structure

Use these sections in order, as appropriate:

```rust
/// Brief one-line summary.
///
/// More detailed description if needed. Can span multiple paragraphs.
///
/// # Arguments
///
/// * `param1` - Description of first parameter
/// * `param2` - Description of second parameter
///
/// # Returns
///
/// Description of what is returned
///
/// # Errors
///
/// Description of when and why this function returns an error
///
/// # Panics
///
/// Description of when this function might panic (if applicable)
///
/// # Examples
///
/// ```
/// use dfraw_parser::function_name;
///
/// let result = function_name(arg1, arg2);
/// assert_eq!(result, expected);
/// ```
///
/// # Safety
///
/// (Only for unsafe functions) Explain safety invariants
pub fn function_name(param1: Type1, param2: Type2) -> Result<ReturnType, Error> {
    // implementation
}
```

### Module Documentation

Modules should have documentation explaining their purpose and contents:

```rust
//! Brief description of what this module provides.
//!
//! More detailed explanation of the module's purpose and organization.
//!
//! # Examples
//!
//! ```
//! use dfraw_parser::module_name;
//!
//! // Example usage
//! ```
```

### Examples in Documentation

#### Do ✅

```rust
/// Parses a creature definition from a raw file.
///
/// # Arguments
///
/// * `raw_text` - The raw text content to parse
///
/// # Returns
///
/// Returns a `Creature` object or an error if parsing fails.
///
/// # Examples
///
/// ```
/// use dfraw_parser::Creature;
///
/// let raw_text = "[CREATURE:DOG]";
/// let creature = Creature::from_raw(raw_text)?;
/// ```
pub fn from_raw(raw_text: &str) -> Result<Creature, ParserError> {
    // implementation
}
```

#### Don't ❌

```rust
/// Parses creature
pub fn from_raw(raw_text: &str) -> Result<Creature, ParserError> {
    // implementation
}
```

### Documentation for Different Item Types

#### Functions and Methods

```rust
/// Returns the identifier of this creature.
///
/// The identifier is the unique name used in raw files to reference this creature.
///
/// # Examples
///
/// ```
/// # use dfraw_parser::Creature;
/// let creature = Creature::new("DOG");
/// assert_eq!(creature.get_identifier(), "DOG");
/// ```
#[must_use]
pub fn get_identifier(&self) -> &str {
    &self.identifier
}
```

#### Structs and Enums

```rust
/// Represents a parsed creature from Dwarf Fortress raw files.
///
/// A creature definition includes all castes, body parts, behaviors,
/// and other properties defined in the raw files.
#[derive(Debug, Clone)]
pub struct Creature {
    /// The unique identifier for this creature
    pub identifier: String,
    /// Optional description text shown in-game
    pub description: Option<String>,
    // ... other fields
}

/// Represents different material states in Dwarf Fortress.
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

#### Traits

```rust
/// Provides search functionality for game objects.
///
/// Implement this trait for types that should be searchable by
/// generating search strings from their properties.
///
/// # Examples
///
/// ```
/// use dfraw_parser::traits::Searchable;
///
/// struct MyObject {
///     name: String,
///     description: String,
/// }
///
/// impl Searchable for MyObject {
///     fn get_search_vec(&self) -> Vec<String> {
///         vec![self.name.clone(), self.description.clone()]
///     }
/// }
/// ```
pub trait Searchable {
    /// Returns a vector of searchable strings for this object.
    fn get_search_vec(&self) -> Vec<String>;
}
```

See the [documentation reference](./DOCUMENTATION_REFERENCE.md) for more examples of how to
document this library.

### Example Code Attributes

Use these attributes as appropriate:

- `no_run` - Code compiles but shouldn't be executed (e.g., needs external files)
- `ignore` - Skip this example in tests
- `should_panic` - Example is expected to panic
- `compile_fail` - Example should fail to compile (for demonstrating errors)

```rust
/// # Examples
///
/// ```no_run
/// use dfraw_parser::parse;
///
/// // This needs actual game files to run
/// let result = parse(&options);
/// ```
```

### Testing Documentation

Run doc tests to ensure examples work:

```bash
cargo test --doc
```

Generate and view documentation locally:

```bash
cargo doc --no-deps --open
```

## Coding Standards

### Rust Style Guidelines

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for code formatting: `cargo fmt`
- Use `clippy` for linting: `cargo clippy`
- Avoid `unwrap()` in library code (use proper error handling)

### Workspace Lints

This project enforces strict lints:

```toml
[workspace.lints.rust]
unsafe_code = "forbid"
missing_docs = "warn"
unreachable_code = "warn"
unreachable_patterns = "warn"

[workspace.lints.clippy]
enum_glob_use = "deny"
pedantic = "deny"
nursery = "deny"
unwrap_used = "deny"
```

### Naming Conventions

- Use `snake_case` for functions, methods, variables, and modules
- Use `CamelCase` for types (structs, enums, traits)
- Use `SCREAMING_SNAKE_CASE` for constants
- Use descriptive names that indicate purpose

### Error Handling

- Use `Result<T, E>` for operations that can fail
- Define specific error types using `thiserror`
- Provide context in error messages
- Document error conditions in function docs

```rust
/// # Errors
///
/// Returns `ParserError::InvalidToken` if the token format is invalid.
/// Returns `ParserError::IoError` if file reading fails.
pub fn parse_file(path: &Path) -> Result<ParsedFile, ParserError> {
    // implementation
}
```

## Testing Guidelines

### Test Organization

- Unit tests go in a `tests` module within the same file
- Integration tests go in the `tests/` directory
- Use descriptive test names that indicate what is being tested

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_creature_tag() {
        let result = parse_creature_tag("[CREATURE:DOG]");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().identifier, "DOG");
    }

    #[test]
    fn test_parse_invalid_creature_tag_returns_error() {
        let result = parse_creature_tag("[INVALID]");
        assert!(result.is_err());
    }
}
```

### Test Coverage

- Aim for high test coverage of public APIs
- Test both success and error cases
- Test edge cases and boundary conditions
- Use property-based testing for complex logic (consider `proptest`)

## Pull Request Process

### Before Submitting

1. **Ensure tests pass**: `cargo test --workspace`
2. **Run clippy**: `cargo clippy --workspace -- -D warnings`
3. **Format code**: `cargo fmt --all`
4. **Update documentation**: Ensure all public APIs are documented
5. **Add/update tests**: Include tests for new functionality
6. **Update CHANGELOG.md**: Add an entry describing your changes

### PR Description Template

```markdown
## Description

Brief description of what this PR does.

## Motivation

Why is this change needed? What problem does it solve?

## Changes

- List of specific changes made
- Use bullet points

## Testing

How was this tested? What test cases were added?

## Documentation

- [ ] Added/updated documentation comments
- [ ] Updated README if needed
- [ ] Added examples if appropriate

## Checklist

- [ ] Tests pass locally
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation is complete
- [ ] CHANGELOG.md is updated
```

### Review Process

1. Maintainers will review your PR
2. Address any requested changes
3. Once approved, your PR will be merged
4. Celebrate your contribution! 🎉

## Questions?

If you have questions or need help:

- Open an issue with the question
- Check existing issues and discussions
- Review the README and documentation

Thank you for contributing to dfraw_parser!
