use proc_macro::TokenStream;
use quote::*;
use std::collections::HashMap;

use syn::{parse_macro_input, Data, DeriveInput, Expr, Lit, LitStr, Visibility};

#[proc_macro_attribute]
pub fn no_discrimination_str(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse argument list into integer type and bit-width
    let mut iter = args.into_iter();

    if let Some(_) = iter.next() {
        panic!("Expected zero macro arguments, found some!");
    }

    // Parse enum body
    let input = parse_macro_input!(input as DeriveInput);
    let mut variants = HashMap::new();
    let mut discriminants = Vec::new();
    let data = match input.data {
        Data::Enum(data) => data,
        _ => panic!("Attribute not applied to enum!"),
    };

    for (i, v) in data.variants.iter().enumerate() {
        if v.ident.to_string() == "_Default" {
            if i != data.variants.len() - 1 {
                panic!("Default condition found, but not at end of list!");
            }
            match &v.discriminant {
                None => {}
                _ => panic!("Default value has discriminant!"),
            }
        } else {
            if i == data.variants.len() - 1 {
                panic!("End of list found before default condition!");
            }
            let (discriminant, span) = match &v.discriminant {
                Some((_, expr)) => {
                    let discriminant = match expr {
                        Expr::Lit(lit) => match &lit.lit {
                            Lit::Str(b) => {
                                if b.value().len() == 0 {
                                    panic!("Zero-length byte-strings are reserved for default!");
                                }
                                (b.value(), b.span())
                            }
                            _ => panic!("Non-byte-string literal found!"),
                        },
                        _ => panic!("Literal not found!"),
                    };
                    discriminant
                }
                None => {
                    panic!("Non-default value has no discriminant!");
                }
            };
            match discriminants.binary_search(&discriminant) {
                Ok(_) => panic!("Duplicate discriminants found!"),
                Err(pos) => discriminants.insert(pos, discriminant.clone()),
            }
            variants.insert(v.ident.to_string(), (discriminant, span));
        }
    }

    let mut result = quote!();
    for attr in input.attrs {
        result.extend(quote! { #attr });
    }

    let name = format_ident!("{}", input.ident.to_string());

    let mut variants_quote = quote!();
    for (variant_name, _) in variants.clone() {
        let variant_name = format_ident!("{}", variant_name);
        variants_quote.extend(quote! { #variant_name, });
    }
    variants_quote.extend(quote! { _Default, });

    match input.vis {
        Visibility::Public(_) => result.extend(quote! {
            pub enum #name {
                #variants_quote
            }
        }),
        Visibility::Crate(_) => result.extend(quote! {
            pub(crate) enum #name {
                #variants_quote
            }
        }),
        _ => result.extend(quote! {
            enum #name {
                #variants_quote
            }
        }),
    }

    // Implement functions to convert generated enum to/from integers

    let mut to_str_matches = quote!();
    let mut from_str_matches = quote!();

    for (variant_name, (discriminant, span)) in variants.clone() {
        let discriminant = LitStr::new(&discriminant, span);
        let variant_name = format_ident!("{}", variant_name);
        to_str_matches.extend(quote! { #name::#variant_name => #discriminant, });
        from_str_matches.extend(quote! { #discriminant => #name::#variant_name, });
    }

    let to_str_match = quote! {
        match self {
            #to_str_matches
            #name::_Default => "",
        }
    };
    let from_str_match = quote! {
        match value {
            #from_str_matches
            _ => #name::_Default,
        }
    };

    match input.vis {
        Visibility::Public(_) => {
            result.extend(quote! {
                impl #name {
                    pub fn to_str(&self) -> &'static str { #to_str_match }
                    pub fn from_str(value: &str) -> Self { #from_str_match }
                }
            });
        }
        Visibility::Crate(_) => {
            result.extend(quote! {
                impl #name {
                    pub(crate) fn to_str(&self) -> &'static str { #to_str_match }
                    pub(crate) fn from_str(value: &str) -> Self { #from_str_match }
                }
            });
        }
        _ => {
            result.extend(quote! {
                impl #name {
                    fn to_str(&self) -> &'static str { #to_str_match }
                    fn from_str(value: &str) -> Self { #from_str_match }
                }
            });
        }
    }

    // Can debug with `result.to_string()`
    proc_macro::TokenStream::from(result)
}
