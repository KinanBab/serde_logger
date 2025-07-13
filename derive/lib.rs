extern crate quote;
extern crate proc_macro;
extern crate syn;

use quote::quote;
use proc_macro::TokenStream;
use syn::{parse_macro_input, parse_str, ItemStruct, Lit, MacroDelimiter, Meta};

#[proc_macro_derive(SerializesFor)]
pub fn serialize_for(input: TokenStream) -> TokenStream {
    let struct_ = parse_macro_input!(input as ItemStruct);

    // Find the path/name of the remote type.
    let mut remote_struct_name = None;
    for attr in struct_.attrs.iter() {
        if let Meta::List(metalist) = &attr.meta {
            if metalist.path.is_ident("serde") {
                if let MacroDelimiter::Paren(_) = metalist.delimiter {
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("remote") {
                            let expr: Lit = meta.value()?.parse()?;
                            if let Lit::Str(expr) = expr {
                                let path: syn::Path = parse_str(&expr.value())?;
                                remote_struct_name = Some(path);
                            } else {
                                return Err(syn::Error::new_spanned(expr, "expected string literal"));
                            }
                        }
                        Ok(())
                    }).expect("Cannot parse #[serde(remote = \"..\")]");
                }
            }
        }
    }
    let remote_struct_name = remote_struct_name.expect("Cannot #[serde(remote = \"..\")]");

    // Struct name and generics
    let struct_name = struct_.ident.clone();
    let (impl_generics, ty_generics, where_clause) = struct_.generics.split_for_impl();

    // Produce new struct definition and the impl block for getters.
    let tokens = quote! {
        impl #impl_generics ::serde_logger::SerializesFor<#remote_struct_name #ty_generics> for #struct_name #ty_generics #where_clause {
            fn serialize_for<S: ::serde::Serializer>(t: &#remote_struct_name #ty_generics, s: S) -> Result<S::Ok, S::Error> {
                Self::serialize(t, s)
            }
        }
    };

    tokens.into()
}