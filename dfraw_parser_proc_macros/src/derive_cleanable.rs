use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitStr, parse_macro_input};

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

    // Find the metadata field and check its type string to avoid errors on non-RawMetadata fields
    let metadata_field = fields
        .iter()
        .find(|f| f.ident.as_ref().is_some_and(|i| i == "metadata"));
    let mut special_metadata_handled = false;

    let metadata_cleaning = metadata_field.map_or_else(
        || quote! {},
        |field| {
            let type_str = quote!(#field.ty).to_string();

            let mut is_ignored = false;
            for attr in &field.attrs {
                if attr.path().is_ident("cleanable") || attr.path().is_ident("is_empty") {
                    let _ = attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("ignore") {
                            is_ignored = true;
                        }
                        Ok(())
                    });
                }
            }

            if !is_ignored && type_str.contains("RawMetadata") {
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

    let fields_cleaning: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|f| {
            let field_name = &f.ident;

            // Skip metadata if we handled it specially above
            if field_name.as_ref().is_some_and(|i| i == "metadata") && special_metadata_handled {
                return quote! {};
            }

            let type_str = quote!(#f.ty).to_string();
            let is_option = type_str.contains("Option");

            let mut is_recursive = false;
            let mut only_if_none = false;
            let mut is_ignored = false;
            let mut custom_is_default: Option<syn::Path> = None;

            for attr in &f.attrs {
                if attr.path().is_ident("cleanable") || attr.path().is_ident("is_empty") {
                    let _ = attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("recursive") { is_recursive = true; }
                        else if meta.path.is_ident("only_if_none") { only_if_none = true; }
                        else if meta.path.is_ident("ignore") { is_ignored = true; }
                        else if meta.path.is_ident("is_default") {
                            let s: LitStr = meta.value()?.parse()?;
                            custom_is_default = Some(s.parse()?);
                        }
                        Ok(())
                    });
                }
            }

            if is_ignored { return quote! {}; }

            // Handle custom check override
            if let Some(check_path) = custom_is_default {
                return quote! {
                    if #check_path(&self.#field_name) {
                        self.#field_name = None;
                    }
                };
            }

            if is_option {
                if only_if_none {
                    if is_recursive {
                        quote! {
                            if let Some(val) = self.#field_name.as_mut() {
                                val.clean();
                            }
                        }
                    } else {
                        quote! {}
                    }
                } else if is_recursive {
                    // Use take() pattern to clean recursively and put back if not empty
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
                quote! { self.#field_name.clean(); }
            } else {
                quote! {}
            }
        })
        .collect();

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
