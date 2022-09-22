use proc_macro2::{TokenStream, TokenTree};
use quote::*;
use std::collections::HashMap;

use syn::{parse2, Attribute, Data, DeriveInput, Expr, Lit, LitByteStr, Visibility};

use crate::get_vis;

type Span = quote::__private::Span;

fn parse_args(args: TokenStream) -> Option<(Vec<u8>, Span)> {
    // Parse argument list into default string if one is given
    let mut iter = args.into_iter();
    match (iter.next(), iter.next(), iter.next(), iter.next()) {
        (
            Some(TokenTree::Ident(ident)),
            Some(TokenTree::Punct(punct)),
            Some(TokenTree::Literal(literal)),
            None,
        ) => {
            if ident.to_string() != "Default" {
                panic!("First argument should be Default");
            }
            if punct.to_string() != "=" {
                panic!("Second argument should be =");
            }
            let s = literal.to_string();
            if s.len() >= 3 && s.starts_with("b\"") && s.ends_with("\"") {
                let s = s[2..s.len() - 1].as_bytes().to_vec();
                Some((s, Span::from(literal.span())))
            } else {
                panic!("Default discriminant not a string!");
            }
        }
        (None, None, None, None) => None,
        _ => panic!("Invalid arguments!"),
    }
}

fn generate_code(
    name: String,
    attrs: &Vec<Attribute>,
    vis: &Visibility,
    variants: HashMap<String, (Vec<u8>, Span)>,
) -> TokenStream {
    let name = format_ident!("{}", name);

    // Implement functions to convert generated enum to/from Option<&'static [u8]>
    let mut variants_quote = quote!();
    let mut to_quotes = quote!();
    let mut from_quotes = quote!();
    for (variant_name, (discriminant, span)) in variants {
        let discriminant = LitByteStr::new(&discriminant, span);
        let variant_name = format_ident!("{}", variant_name);
        variants_quote.extend(quote! { #variant_name, });
        to_quotes.extend(quote! { #name::#variant_name => #discriminant, });
        from_quotes.extend(quote! { #discriminant => Some(#name::#variant_name), });
    }
    from_quotes.extend(quote! { _ => None, });

    // Construct resulting struct and impl functions
    let vis = get_vis(vis);
    let attrs = attrs.iter().map(|attr| quote! { #attr });
    TokenStream::from(quote! {
        #(#attrs)*
        #vis enum #name {
            #variants_quote
        }
        impl #name {
            #vis fn to_byte_str(&self) -> &'static [u8] {
                match self {
                    #to_quotes
                }
            }
            #vis fn from_byte_str(value: &[u8]) -> Option<Self> {
                match value {
                    #from_quotes
                }
            }
        }
    })
}

fn generate_code_default(
    name: String,
    attrs: &Vec<Attribute>,
    vis: &Visibility,
    variants: HashMap<String, (Vec<u8>, Span)>,
    default_variant: (Vec<u8>, Span),
) -> TokenStream {
    let name = format_ident!("{}", name);

    // Implement functions to convert generated enum to/from &'static [u8]
    let mut variants_quote = quote!();
    let mut to_quotes = quote!();
    let mut from_quotes = quote!();
    for (variant_name, (discriminant, span)) in variants {
        let discriminant = LitByteStr::new(&discriminant, span);
        let variant_name = format_ident!("{}", variant_name);
        variants_quote.extend(quote! { #variant_name, });
        to_quotes.extend(quote! { #name::#variant_name => #discriminant, });
        from_quotes.extend(quote! { #discriminant => #name::#variant_name, });
    }
    let (discriminant, span) = default_variant;
    let discriminant = LitByteStr::new(&discriminant, span);
    variants_quote.extend(quote! { Default, });
    to_quotes.extend(quote! { #name::Default => #discriminant, });
    from_quotes.extend(quote! { _ => #name::Default, });

    // Construct resulting struct and impl functions
    let vis = get_vis(vis);
    let attrs = attrs.iter().map(|attr| quote! { #attr });
    TokenStream::from(quote! {
        #(#attrs)*
        #vis enum #name {
            #variants_quote
        }
        impl #name {
            #vis fn to_byte_str(&self) -> &'static [u8] {
                match self {
                    #to_quotes
                }
            }
            #vis fn from_byte_str(value: &[u8]) -> Self {
                match value {
                    #from_quotes
                }
            }
        }
    })
}

pub fn indiscriminant_byte_str(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse arguments
    let args = parse_args(args);

    // Parse enum body
    // let input = parse_macro_input!(input as DeriveInput);
    let input = match parse2::<DeriveInput>(input) {
        Ok(input) => input,
        Err(err) => return TokenStream::from(err.to_compile_error()),
    };
    let data = match input.data {
        Data::Enum(data) => data,
        _ => panic!("Attribute not applied to enum!"),
    };
    assert!(data.variants.len() > 0, "Enum is empty of any variants!");

    // Parse enum variants and discriminants
    let mut variants = HashMap::new();
    let mut discriminants = Vec::new();
    let has_default = if let Some((literal, _)) = &args {
        discriminants.push(literal.clone());
        true
    } else {
        false
    };

    for v in data.variants.iter() {
        let ident = v.ident.to_string();
        let literal = match (ident.as_str(), &v.discriminant) {
            ("Default", _) if has_default => {
                panic!("Default variant already provided as argument!")
            }
            (_, Some((_, Expr::Lit(literal)))) => literal,
            (ident, Some(_)) => panic!("Discriminant is not a literal for variant {}!", ident),
            (ident, None) => panic!("Discriminant not found for variant {}!", ident),
        };
        let (discriminant, span) = match &literal.lit {
            Lit::ByteStr(b) => (b.value(), b.span()),
            _ => panic!("Non-byte-string literal found!"),
        };
        match discriminants.binary_search(&discriminant) {
            Ok(_) => panic!("Duplicate discriminants found!"),
            Err(pos) => discriminants.insert(pos, discriminant.clone()),
        }
        variants.insert(ident.to_string(), (discriminant, span));
    }

    if let Some((literal, span)) = args {
        generate_code_default(
            input.ident.to_string(),
            &input.attrs,
            &input.vis,
            variants,
            (literal, span),
        )
    } else {
        generate_code(input.ident.to_string(), &input.attrs, &input.vis, variants)
    }
}
