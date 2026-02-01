//! Token parsing utilities for Dwarf Fortress raw file tokens.

use std::fmt::Debug;
use std::str::FromStr;

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
    /// Internal Helper: Parses a string into T.
    /// specialized logic: If parsing fails and the value is "NONE", returns T::default() (0).
    fn parse_value<T>(&self, value: &str) -> Option<T>
    where
        T: FromStr + Default,
        <T as FromStr>::Err: Debug,
    {
        match value.parse::<T>() {
            Ok(v) => Some(v),
            Err(_) if value == "NONE" => {
                // "NONE" failed to parse (likely expecting a number), so we default to 0
                Some(T::default())
            }
            Err(e) => {
                tracing::warn!(
                    "parse_value: failed to parse '{value}' for tag {:?}: {e:?}",
                    self
                );
                None
            }
        }
    }

    /// Helper: Parses a vector of values.
    /// Usage: `[BODY:HEAD:TORSO]` -> `parse_vector(..., |p| Self::Body(p))`
    fn parse_vector<T, F, S>(&self, values: &[&str], builder: F) -> Option<S>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
        F: Fn(Vec<T>) -> S,
    {
        let parsed: Vec<T> = values
            .iter()
            .map(|&v| v.parse::<T>())
            .collect::<Result<_, _>>()
            .map_err(|e| tracing::warn!("parse_vector: failed to parse: {e:?}"))
            .ok()?;
        Some(builder(parsed))
    }

    /// Helper: Parses a key (String) and a value.
    /// Usage: `[SKILL_LEARN_RATE:SWORD:100]` -> `parse_key_value(..., |k, v| Self::SkillLearnRate{"Sword", 100})`
    fn parse_key_value<T, F, S>(&self, values: &[&str], builder: F) -> Option<S>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
        F: Fn(String, T) -> S,
    {
        if values.len() < 2 {
            return None;
        }
        let key = values[0].to_string();
        let val = values[1].parse().ok()?;
        Some(builder(key, val))
    }

    /// Helper: Parses a Label (String) + Variable number of values.
    /// Usage: [TAG:LABEL:1:2:3...] -> parse_labeled_vector(..., |label, vec| ...)
    fn parse_labeled_vector<Y, T, F, S>(&self, values: &[&str], builder: F) -> Option<S>
    where
        Y: FromStr + Default + Debug,
        <Y as FromStr>::Err: Debug,
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
        F: Fn(Y, Vec<T>) -> S,
    {
        if values.is_empty() {
            tracing::warn!("parse_labeled_vector: missing label for tag {:?}", self);
            return None;
        }

        let label = self.parse_value(values.first()?)?;

        // Try to parse the rest of the slice into a Vec<T>
        let parsed_args: Vec<T> = values[1..]
                .iter()
                .map(|&v| v.parse::<T>())
                .collect::<Result<_, _>>()
                .map_err(|e| tracing::warn!("parse_labeled_vector: failed to parse args for tag {:?} label '{label:?}': {e:?}", self))
                .ok()?;

        Some(builder(label, parsed_args))
    }

    /// Helper: Parses exactly one value.
    /// Usage: `[BABY:1]` -> `parse_single(..., |v| Self::Baby{age: v})`
    fn parse_single<T, F, S>(&self, values: &[&str], builder: F) -> Option<S>
    where
        T: FromStr + Default,
        <T as FromStr>::Err: Debug,
        F: Fn(T) -> S,
    {
        let val = self.parse_value(values.first()?)?;
        Some(builder(val))
    }

    /// Helper: Verifies a tag has no arguments (Flags).
    /// Usage: `[FLIER]` -> `parse_flag(..., Self::Flier)`
    fn parse_flag<S>(&self, values: &[&str], instance: S) -> Option<S> {
        if !values.is_empty() {
            tracing::warn!(
                "Found values for tag expected to be used as a flag: {self:?} {values:?}"
            );
        }
        Some(instance)
    }

    /// Helper: Parses exactly N values into an array.
    /// Supports numbers and non-copy structs like Strings
    /// Usage: `[COLOR:0:0:1]` -> `parse_array(..., |[r,g,b]| ...)`
    fn parse_array<T, const N: usize, F, S>(&self, values: &[&str], builder: F) -> Option<S>
    where
        T: FromStr + Default + Debug,
        <T as FromStr>::Err: Debug,
        F: Fn([T; N]) -> S,
    {
        if values.len() < N {
            tracing::warn!(
                "parse_array: expected {} args, found {} for tag {:?}",
                N,
                values.len(),
                self
            );
            return None;
        }

        // Collect exactly N items into a Vec
        // This handles the parsing and the "NONE" logic via our custom parse_value
        let parsed_vec: Vec<T> = values
            .iter()
            .take(N)
            .map(|v| self.parse_value(v))
            .collect::<Option<Vec<T>>>()?;

        // Convert the Vec into an Array
        // This can move Strings into the array without needing Copy
        match parsed_vec.try_into() {
            Ok(arr) => Some(builder(arr)),
            Err(_) => {
                // This branch should practically never be hit due to .take(N)
                tracing::warn!(
                    "parse_array: failed to convert vec to array for tag {:?}",
                    self
                );
                None
            }
        }
    }

    /// Helper: Parses a Label + N values.
    /// Updated to support non-Copy types.
    /// Usage: [TAG:LABEL:1:2:3...] -> parse_labeled_vector(..., |label, vec| ...)
    /// Usage: [TAG:10:Something:Else...] -> parse_labeled_vector(..., |label, vec| ...)
    fn parse_labeled_array<Y, T, const N: usize, F, S>(
        &self,
        values: &[&str],
        builder: F,
    ) -> Option<S>
    where
        Y: FromStr + Default + Debug,
        <Y as FromStr>::Err: Debug,
        T: FromStr + Default + Debug,
        <T as FromStr>::Err: Debug,
        F: Fn(Y, [T; N]) -> S,
    {
        if values.len() < N + 1 {
            tracing::warn!(
                "parse_labeled_array: expected {} args, found {} for tag {:?}",
                N + 1,
                values.len(),
                self
            );
            return None;
        }

        let label = self.parse_value(values.first()?)?;

        // Same logic: Collect N items (skipping the label) into a Vec
        let parsed_vec: Vec<T> = values[1..]
            .iter()
            .take(N)
            .map(|v| self.parse_value(v))
            .collect::<Option<Vec<T>>>()?;

        match parsed_vec.try_into() {
            Ok(arr) => Some(builder(label, arr)),
            Err(_) => None,
        }
    }

    /// Helper: Parses a list of values where the LAST value is distinct (the "Tail").
    /// Usage: `[BLOOD:MAT:SUBMAT:STATE]` -> `Body=[MAT, SUBMAT], Tail=STATE`
    fn parse_vector_with_tail<T, U, F, S>(&self, values: &[&str], builder: F) -> Option<S>
    where
        T: std::str::FromStr + Default, // Type for the Body elements (e.g. String)
        U: std::str::FromStr + Default, // Type for the Tail element (e.g. String or u32)
        <T as std::str::FromStr>::Err: std::fmt::Debug,
        <U as std::str::FromStr>::Err: std::fmt::Debug,
        F: Fn(Vec<T>, U) -> S, // Builder gets (Body, Tail)
    {
        if values.len() < 2 {
            tracing::warn!(
                "parse_vector_with_tail: expected at least 2 args, found {} for tag {:?}",
                values.len(),
                self
            );
            return None;
        }

        // Parse the Tail (Last Element)
        let tail_str = values.last()?;
        let tail = self.parse_value::<U>(tail_str)?;

        // Parse the Body (All elements except the last)
        let body_values = &values[..values.len() - 1];
        let body: Vec<T> = body_values
            .iter()
            .map(|&v| self.parse_value::<T>(v))
            .collect::<Option<Vec<T>>>()?;

        Some(builder(body, tail))
    }
}

// Impls for each tag so we can use it for parsing.
impl TokenParser for crate::tokens::CasteToken {}
impl TokenParser for crate::tokens::CreatureToken {}
impl TokenParser for crate::tokens::PlantToken {}
impl TokenParser for crate::tokens::ConditionToken {}
impl TokenParser for crate::tokens::EntityToken {}
impl TokenParser for crate::tokens::PositionToken {}
