use proc_macro2::{TokenStream, TokenTree};
use quote::*;
use std::collections::HashMap;

use syn::{parse2, Attribute, Data, DeriveInput, Expr, Lit, LitStr, Visibility};

use crate::get_vis;

type Span = quote::__private::Span;

fn parse_args(args: TokenStream) -> Option<(String, Span)> {
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
            if s.len() >= 2 && s.starts_with("\"") && s.ends_with("\"") {
                let s = s[1..s.len() - 1].to_string();
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
    variants: HashMap<String, (String, Span)>,
) -> TokenStream {
    let name = format_ident!("{}", name);

    // Implement functions to convert generated enum to/from Option<&'static str>
    let mut variants_quote = quote!();
    let mut to_quotes = quote!();
    let mut from_quotes = quote!();
    for (variant_name, (discriminant, span)) in variants {
        let variant_name = format_ident!("{}", variant_name);
        let discriminant = LitStr::new(&discriminant, span);
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

fn generate_code_default(
    name: String,
    attrs: &Vec<Attribute>,
    vis: &Visibility,
    variants: HashMap<String, (String, Span)>,
    default_variant: (String, Span),
) -> TokenStream {
    let name = format_ident!("{}", name);

    // Implement functions to convert generated enum to/from &'static str
    let mut variants_quote = quote!();
    let mut to_quotes = quote!();
    let mut from_quotes = quote!();
    for (variant_name, (discriminant, span)) in variants {
        let discriminant = LitStr::new(&discriminant, span);
        let variant_name = format_ident!("{}", variant_name);
        variants_quote.extend(quote! { #variant_name, });
        to_quotes.extend(quote! { #name::#variant_name => #discriminant, });
        from_quotes.extend(quote! { #discriminant => #name::#variant_name, });
    }
    let (discriminant, span) = default_variant;
    let discriminant = LitStr::new(&discriminant, span);
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

pub fn indiscriminant_str(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse arguments
    let args = parse_args(args);

    // Parse enum body
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
            Lit::Str(b) => (b.value(), b.span()),
            _ => panic!("Non-string literal found!"),
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
