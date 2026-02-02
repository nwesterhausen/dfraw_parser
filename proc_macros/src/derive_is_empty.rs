use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Expr, Fields, LitStr, parse_macro_input};

/// Implmentation of the derive macro for `dfraw_parser::traits::IsEmpty`
pub fn impl_derive_is_empty(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("IsEmpty only supports named fields"),
        },
        _ => panic!("IsEmpty only supports structs"),
    };

    let has_identifier = fields
        .iter()
        .any(|f| f.ident.as_ref().is_some_and(|i| i == "identifier"));

    let is_empty_impl = if has_identifier {
        quote! { self.identifier.is_empty() }
    } else {
        let checks = fields.iter().filter_map(|f| {
            let field_name = &f.ident;
            let type_str = quote!(#f.ty).to_string();
            let is_option = type_str.contains("Option");

            let mut is_ignored = false;
            let mut only_if_none = false;
            let mut custom_is_default: Option<syn::Path> = None;
            let mut custom_value: Option<Expr> = None;

            for attr in &f.attrs {
                if attr.path().is_ident("is_empty") || attr.path().is_ident("cleanable") {
                    let _ = attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("ignore") {
                            is_ignored = true;
                        } else if meta.path.is_ident("only_if_none") {
                            only_if_none = true;
                        } else if meta.path.is_ident("is_default") {
                            let s: LitStr = meta.value()?.parse()?;
                            custom_is_default = Some(s.parse()?);
                        } else if meta.path.is_ident("value") {
                            custom_value = Some(meta.value()?.parse()?);
                        }
                        Ok(())
                    });
                }
            }

            // metadata should not contribute to whether a component is "empty"
            if is_ignored || field_name.as_ref().is_some_and(|i| i == "metadata") {
                None
            } else if let Some(val) = custom_value {
                if is_option {
                    Some(quote! { self.#field_name.as_ref().map_or(true, |v| v == &#val) })
                } else {
                    Some(quote! { self.#field_name == #val })
                }
            } else if let Some(check_path) = custom_is_default {
                Some(quote! { #check_path(&self.#field_name) })
            } else if only_if_none {
                Some(quote! { self.#field_name.is_none() })
            } else {
                Some(quote! { crate::traits::IsEmpty::is_empty(&self.#field_name) })
            }
        });

        quote! { #(#checks)&&* }
    };

    let expanded = quote! {
        impl #impl_generics crate::traits::IsEmpty for #name #ty_generics #where_clause {
            fn is_empty(&self) -> bool {
                #is_empty_impl
            }
        }
    };

    TokenStream::from(expanded)
}
