//! Provides macros used in the `dfraw_parser` library

mod derive_cleanable;
mod derive_is_empty;

use proc_macro::TokenStream;

/// Derives `dfraw_parser::traits::Cleanable`
#[proc_macro_derive(Cleanable, attributes(cleanable, is_empty))]
pub fn derive_cleanable(input: TokenStream) -> TokenStream {
    derive_cleanable::impl_derive_cleanable(input)
}

/// Derives `dfraw_parser::traits::IsEmpty`
#[proc_macro_derive(IsEmpty, attributes(is_empty))]
pub fn derive_is_empty(input: TokenStream) -> TokenStream {
    derive_is_empty::impl_derive_is_empty(input)
}

// mod derive_searchable;
// #[proc_macro_derive(Searchable)]
// pub fn derive_searchable(input: TokenStream) -> TokenStream {
//     derive_searchable::impl_derive_searchable(input)
// }
