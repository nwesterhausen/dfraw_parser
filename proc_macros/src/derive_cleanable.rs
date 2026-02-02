use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Expr, Fields, LitStr, parse_macro_input};

/// Implmentation of the derive macro for `dfraw_parser::traits::Cleanable`
#[allow(clippy::too_many_lines)]
pub fn impl_derive_cleanable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Cleanable only supports named fields"),
        },
        _ => panic!("Cleanable only supports structs"),
    };

    // Find the metadata field
    let metadata_field = fields
        .iter()
        .find(|f| f.ident.as_ref().is_some_and(|i| i == "metadata"));
    let mut special_metadata_handled = false;

    // We only generate the "hide" logic if metadata is an Option.
    // If it is a direct RawMetadata type, we cannot set it to None.
    let metadata_cleaning = metadata_field.map_or_else(
        || quote! {},
        |field| {
            let type_str = quote!(#field.ty).to_string();
            let is_option = type_str.contains("Option");

            if type_str.contains("RawMetadata") && is_option {
                special_metadata_handled = true;
                quote! {
                    let hide_metadata = self.metadata.as_ref().map_or(false, |m| m.is_hidden());
                    if hide_metadata {
                        self.metadata = None;
                    }
                }
            } else {
                quote! {}
            }
        },
    );

    let fields_cleaning: Vec<proc_macro2::TokenStream> = fields.iter().map(|f| {
        let field_name = &f.ident;

        // If we already handled metadata as an Option (hiding it), skip it here.
        // If it's a direct type, let the normal logic below handle it (e.g. recursion).
        if field_name.as_ref().is_some_and(|i| i == "metadata") && special_metadata_handled {
            return quote! {};
        }

        let type_str = quote!(#f.ty).to_string();
        let is_option = type_str.contains("Option");

        let mut is_recursive = false;
        let mut is_ignored = false;
        let mut custom_is_default: Option<syn::Path> = None;
        let mut custom_value: Option<Expr> = None;

        for attr in &f.attrs {
            if attr.path().is_ident("cleanable") || attr.path().is_ident("is_empty") {
                let _ = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("recursive") { is_recursive = true; }
                    else if meta.path.is_ident("ignore") { is_ignored = true; }
                    else if meta.path.is_ident("is_default") {
                        let s: LitStr = meta.value()?.parse()?;
                        custom_is_default = Some(s.parse()?);
                    }
                    else if meta.path.is_ident("value") {
                        custom_value = Some(meta.value()?.parse()?);
                    }
                    Ok(())
                });
            }
        }

        if is_ignored { return quote! {}; }

        // Pruning logic (Setting Option to None based on custom value/check)
        if is_option {
            if let Some(val) = custom_value {
                return quote! {
                    if self.#field_name.as_ref().map_or(true, |v| v == &#val) {
                        self.#field_name = None;
                    }
                };
            }
            if let Some(check_path) = custom_is_default {
                return quote! {
                    if #check_path(&self.#field_name) {
                        self.#field_name = None;
                    }
                };
            }
        }

        // Structural logic (Recursion and Default Trait Pruning)
        if is_option {
            if is_recursive {
                quote! {
                    if let Some(mut val) = self.#field_name.take() {
                        val.clean();
                        if !crate::traits::IsEmpty::is_empty(&val) {
                            self.#field_name = Some(val);
                        }
                    }
                }
            } else {
                quote! {
                    if self.#field_name.as_ref().map_or(false, crate::traits::IsEmpty::is_empty) {
                        self.#field_name = None;
                    }
                }
            }
        } else if is_recursive {
            // Field is a direct type: call clean() in-place
            quote! { self.#field_name.clean(); }
        } else {
            quote! {}
        }
    }).collect();

    let expanded = quote! {
        impl #impl_generics crate::traits::Cleanable for #name #ty_generics #where_clause {
            fn clean(&mut self) {
                #metadata_cleaning
                #(#fields_cleaning)*
            }
        }
    };

    TokenStream::from(expanded)
}
