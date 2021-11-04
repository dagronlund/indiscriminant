use proc_macro::TokenStream;
use quote::*;
use std::collections::HashMap;

use syn::{parse_macro_input, Data, DeriveInput, Expr, Lit, LitStr, Visibility};

type QuoteResult = quote::__private::TokenStream;

fn get_vis(vis: &Visibility) -> QuoteResult {
    match vis {
        Visibility::Public(_) => quote! { pub },
        Visibility::Crate(_) => quote! { pub(crate) },
        _ => quote! {},
    }
}

#[proc_macro_attribute]
pub fn no_discrimination_str_default(args: TokenStream, input: TokenStream) -> TokenStream {
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
        if v.ident.to_string() == "Default" {
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

    let name = format_ident!("{}", input.ident.to_string());

    let mut variants_quote = quote!();
    for (variant_name, _) in variants.clone() {
        let variant_name = format_ident!("{}", variant_name);
        variants_quote.extend(quote! { #variant_name, });
    }
    variants_quote.extend(quote! { Default, });

    // Implement functions to convert generated enum to/from integers
    let mut to_quotes = quote!();
    let mut from_quotes = quote!();
    for (variant_name, (discriminant, span)) in variants.clone() {
        let discriminant = LitStr::new(&discriminant, span);
        let variant_name = format_ident!("{}", variant_name);
        to_quotes.extend(quote! { #name::#variant_name => #discriminant, });
        from_quotes.extend(quote! { #discriminant => #name::#variant_name, });
    }
    to_quotes.extend(quote! { #name::Default => "", });
    from_quotes.extend(quote! { _ => #name::Default, });

    // Construct resulting struct and impl functions, can debug with `result.to_string()`
    let vis = get_vis(&input.vis);
    let attrs = input.attrs.iter().map(|attr| quote! { #attr });
    proc_macro::TokenStream::from(quote! {
        #(#attrs)*
        #vis enum #name {
            #variants_quote
        }
        impl #name {
            #vis fn to_str(&self) -> &'static str {
                match self {
                    #to_quotes
                }
            }
            #vis fn from_str(value: &str) -> Self {
                match value {
                    #from_quotes
                }
            }
        }
    })
}

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

    for (_, v) in data.variants.iter().enumerate() {
        let (discriminant, span) = match &v.discriminant {
            Some((_, expr)) => {
                let discriminant = match expr {
                    Expr::Lit(lit) => match &lit.lit {
                        Lit::Str(b) => (b.value(), b.span()),
                        _ => panic!("Non-byte-string literal found!"),
                    },
                    _ => panic!("Literal not found!"),
                };
                discriminant
            }
            None => {
                panic!("Value has no discriminant!");
            }
        };
        match discriminants.binary_search(&discriminant) {
            Ok(_) => panic!("Duplicate discriminants found!"),
            Err(pos) => discriminants.insert(pos, discriminant.clone()),
        }
        variants.insert(v.ident.to_string(), (discriminant, span));
    }

    let name = format_ident!("{}", input.ident.to_string());

    let mut variants_quote = quote!();
    for (variant_name, _) in variants.clone() {
        let variant_name = format_ident!("{}", variant_name);
        variants_quote.extend(quote! { #variant_name, });
    }

    // Implement functions to convert generated enum to/from integers
    let mut to_quotes = quote!();
    let mut from_quotes = quote!();
    for (variant_name, (discriminant, span)) in variants.clone() {
        let discriminant = LitStr::new(&discriminant, span);
        let variant_name = format_ident!("{}", variant_name);
        to_quotes.extend(quote! { #name::#variant_name => #discriminant, });
        from_quotes.extend(quote! { #discriminant => Some(#name::#variant_name), });
    }
    from_quotes.extend(quote! { _ => None, });

    // Construct resulting struct and impl functions, can debug with `result.to_string()`
    let vis = get_vis(&input.vis);
    let attrs = input.attrs.iter().map(|attr| quote! { #attr });
    proc_macro::TokenStream::from(quote! {
        #(#attrs)*
        #vis enum #name {
            #variants_quote
        }
        impl #name {
            #vis fn to_str(&self) -> &'static str {
                match self {
                    #to_quotes
                }
            }
            #vis fn from_str(value: &str) -> Option<Self> {
                match value {
                    #from_quotes
                }
            }
        }
    })
}
