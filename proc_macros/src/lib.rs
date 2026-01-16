//! Provides macros used in the `dfraw_parser` library

mod derive_cleanable;
mod derive_is_empty;

use proc_macro::TokenStream;

/// Derives `dfraw_parser::traits::Cleanable`
///
/// This macro generates the `clean(&mut self)` method, which is used to reduce
/// the memory footprint of a struct by pruning "empty" values (setting `Option`
/// fields to `None`).
///
/// ### Attributes (`#[cleanable(...)]` or `#[is_empty(...)]`)
/// - `recursive`: Calls `.clean()` on the field's value. Required for nested structs.
/// - `ignore`: Prevents the macro from touching this field during the cleaning process.
/// - `is_default = "path"`: Points to a custom function `fn(&T) -> bool` to determine
///   if a field should be pruned.
/// - `value = <expr>`: prunes the field if it matches the provided value (e.g., `500` or `[0,0]`).
///
/// ### Behavior
/// - `Option<T>`: If the inner value is "empty" (according to `IsEmpty`), the field is set to `None`.
/// - `metadata`: If a field is named `metadata` and is a `RawMetadata` type, it is
///   automatically set to `None` if `is_hidden()` returns true.
#[proc_macro_derive(Cleanable, attributes(cleanable, is_empty))]
pub fn derive_cleanable(input: TokenStream) -> TokenStream {
    derive_cleanable::impl_derive_cleanable(input)
}

/// Derives `dfraw_parser::traits::IsEmpty` for a struct.
///
/// This macro determines if a struct is "empty", which is used by [`Cleanable`]
/// and can be used for optimized serialization (e.g., `skip_serializing_if`).
///
/// ### Attributes (`#[is_empty(...)]`
/// - `ignore`: Skips the field when calculating if the struct is empty.
/// - `only_if_none`: For `Option<T>`, returns true only if the field is `None`,
///   ignoring whether the inner value is empty.
/// - `is_default = "path"`: Uses a custom function to determine if the field is empty.
/// - `value = <expr>`: Returns true if the field matches the provided value.
///
/// ### Behavior
/// - **Identifier Heuristic**: If the struct has a field named `identifier`,
///   `is_empty` simply checks `self.identifier.is_empty()`.
/// - **Component Heuristic**: Otherwise, it returns true only if **all**
///   non-ignored fields are empty.
#[proc_macro_derive(IsEmpty, attributes(is_empty))]
pub fn derive_is_empty(input: TokenStream) -> TokenStream {
    derive_is_empty::impl_derive_is_empty(input)
}

// mod derive_searchable;
// #[proc_macro_derive(Searchable)]
// pub fn derive_searchable(input: TokenStream) -> TokenStream {
//     derive_searchable::impl_derive_searchable(input)
// }
