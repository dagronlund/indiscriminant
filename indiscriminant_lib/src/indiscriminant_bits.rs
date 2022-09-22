use proc_macro2::{TokenStream, TokenTree};
use quote::*;
use std::collections::HashMap;

use syn::{parse2, Attribute, Data, DeriveInput, Visibility};

use crate::{get_vis, IntegerType};

fn parse_args(args: TokenStream) -> (IntegerType, u8, Option<Option<usize>>) {
    // Parse integer type
    let mut iter = args.into_iter();
    let integer_type = match iter.next() {
        Some(TokenTree::Ident(ident)) => {
            IntegerType::from_str(&ident.to_string()).expect("Invalid integer type argument!")
        }
        _ => panic!("Invalid arguments!"),
    };
    // Check for comma indicating more arguments
    match iter.next() {
        Some(TokenTree::Punct(punct)) if punct.to_string() == "," => {}
        Some(_) => panic!("Macro expected comma!"),
        None => return (integer_type.clone(), integer_type.get_width(), None),
    }
    // Check for explicit bit-width
    let (bit_width, next) = match iter.next() {
        Some(TokenTree::Literal(literal)) => {
            let bit_width = literal
                .to_string()
                .parse::<u8>()
                .expect("Invalid bit-width!");
            assert!(
                bit_width <= integer_type.get_width(),
                "Bit-width {} too large for integer type {}!",
                bit_width,
                integer_type.to_str()
            );
            // Check for comma indicating more arguments
            match iter.next() {
                Some(TokenTree::Punct(punct)) if punct.to_string() == "," => {
                    (bit_width, iter.next())
                }
                Some(_) => panic!("Macro expected comma!"),
                None => return (integer_type, bit_width, None),
            }
        }
        next => (integer_type.get_width(), next),
    };
    // Check for explicit default value
    match (next, iter.next(), iter.next(), iter.next()) {
        (
            Some(TokenTree::Ident(ident)),
            Some(TokenTree::Punct(punct)),
            Some(TokenTree::Literal(literal)),
            None,
        ) => {
            assert!(ident.to_string() == "Default", "Invalid arguments!");
            assert!(punct.to_string() == "=", "Invalid arguments!");
            let default_value = literal
                .to_string()
                .parse::<usize>()
                .expect("Invalid default discriminant value!");
            (integer_type, bit_width, Some(Some(default_value)))
        }
        (Some(TokenTree::Ident(ident)), None, None, None) => {
            assert!(ident.to_string() == "Default", "Invalid arguments!");
            (integer_type, bit_width, Some(None))
        }
        (None, None, None, None) => (integer_type, bit_width, None),
        _ => panic!("Invalid arguments!",),
    }
}

fn generate_code_default(
    name: String,
    integer_type: IntegerType,
    bit_width: u8,
    attrs: &Vec<Attribute>,
    vis: &Visibility,
    variants: HashMap<String, usize>,
    default_variant: Option<usize>,
) -> TokenStream {
    let name = format_ident!("{}", name);
    let itype = format_ident!("{}", integer_type.to_str());

    // Implement functions to convert generated enum to/from integers
    let mut variants_quote = quote!();
    let mut to_matches = quote!();
    let mut from_matches = quote!();
    for (variant_name, discriminant) in &variants {
        let variant_name = format_ident!("{}", variant_name);
        variants_quote.extend(quote! { #variant_name = #discriminant as #itype, });
        to_matches.extend(quote! { #name::#variant_name => #discriminant as #itype, });
        let discriminant = integer_type.quote_discriminant(*discriminant);
        from_matches.extend(quote! { #discriminant => #name::#variant_name, });
    }
    // Handle an explicit default variant
    if let Some(default_variant) = default_variant {
        variants_quote.extend(quote! { Default = #default_variant as #itype, });
        to_matches.extend(quote! { #name::Default => #default_variant as #itype, });
        from_matches.extend(quote! { _ => #name::Default, });
    // Handle not having a default but also not fully covering the native
    // integer space according to the compiler
    } else if variants.len() < (1usize << integer_type.get_width()) {
        let variant_name = format_ident!("{}", variants.iter().next().unwrap().0);
        from_matches.extend(quote! { _ => #name::#variant_name, });
    }

    // Construct resulting struct and impl functions
    let vis = get_vis(vis);
    let bit_mask: usize = (1 << bit_width) - 1;
    let attrs = attrs.iter().map(|attr| quote! { #attr });
    TokenStream::from(quote! {
        #(#attrs)*
        #[repr(#itype)]
        #vis enum #name {
            #variants_quote
        }
        impl #name {
            #vis fn to_int(&self) -> #itype {
                match self {
                    #to_matches
                }
            }
            #vis fn from_int(value: #itype) -> Self {
                let masked_value = #bit_mask as #itype & value;
                match masked_value {
                    #from_matches
                }
            }
        }
    })
}

fn generate_code(
    name: String,
    integer_type: IntegerType,
    bit_width: u8,
    attrs: &Vec<Attribute>,
    vis: &Visibility,
    variants: HashMap<String, usize>,
) -> TokenStream {
    let name = format_ident!("{}", name);
    let itype = format_ident!("{}", integer_type.to_str());

    // Implement functions to convert generated enum to/from integers
    let mut variants_quote = quote!();
    let mut to_matches = quote!();
    let mut from_matches = quote!();
    for (variant_name, discriminant) in variants {
        let variant_name = format_ident!("{}", variant_name);
        variants_quote.extend(quote! { #variant_name = #discriminant as #itype, });
        to_matches.extend(quote! { #name::#variant_name => #discriminant as #itype, });
        let discriminant = integer_type.quote_discriminant(discriminant);
        from_matches.extend(quote! { #discriminant => Some(#name::#variant_name), });
    }
    from_matches.extend(quote! { _ => None, });

    // Construct resulting struct and impl functions
    let vis = get_vis(vis);
    let bit_mask: usize = (1 << bit_width) - 1;
    let attrs = attrs.iter().map(|attr| quote! { #attr });
    TokenStream::from(quote! {
        #(#attrs)*
        #[repr(#itype)]
        #vis enum #name {
            #variants_quote
        }
        impl #name {
            #vis fn to_int(&self) -> #itype {
                match self {
                    #to_matches
                }
            }
            #vis fn from_int(value: #itype) -> Option<Self> {
                let masked_value = #bit_mask as #itype & value;
                match masked_value {
                    #from_matches
                }
            }
        }
    })
}

pub fn indiscriminant_bits(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse argument list into integer type and bit-width
    let (integer_type, bit_width, default_value) = parse_args(args);

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

    let max_variant = 1 << bit_width;
    let mut variants = HashMap::new();
    let mut discriminants = Vec::new();
    let has_default = if let Some(default_value) = default_value {
        if let Some(default_value) = default_value {
            discriminants.push(default_value);
        }
        true
    } else {
        false
    };

    for v in data.variants.iter() {
        let ident = v.ident.to_string();
        if has_default && ident == "Default" {
            panic!("Default variant already provided as argument!");
        }
        let discriminant = match &v.discriminant {
            Some((_, expr)) => match integer_type.from_expr(expr) {
                Ok(discriminant) => discriminant,
                Err(_) => panic!("Non-integer discriminant found!"),
            },
            None => panic!("Discriminant not found for variant {}!", ident),
        };
        assert!(discriminant < max_variant, "Discriminant too big!");
        match discriminants.binary_search(&discriminant) {
            Ok(_) => panic!("Duplicate discriminants found!"),
            Err(pos) => discriminants.insert(pos, discriminant),
        }
        variants.insert(ident, discriminant);
    }

    let default_value = if let Some(default_value) = default_value {
        if let Some(default_value) = default_value {
            Some(default_value)
        } else {
            // Determine what an unused value can be for the default
            let mut unused = None;
            for i in 0..max_variant {
                if !discriminants.contains(&i) {
                    unused = Some(i);
                    break;
                }
            }
            if let Some(unused) = unused {
                Some(unused)
            } else {
                panic!("Default value assuming first unused value but value space is completely covered!")
            }
        }
    } else {
        None
    };

    if default_value.is_some() || discriminants.len() == max_variant {
        generate_code_default(
            input.ident.to_string(),
            integer_type,
            bit_width,
            &input.attrs,
            &input.vis,
            variants,
            default_value,
        )
    } else {
        generate_code(
            input.ident.to_string(),
            integer_type,
            bit_width,
            &input.attrs,
            &input.vis,
            variants,
        )
    }
}
