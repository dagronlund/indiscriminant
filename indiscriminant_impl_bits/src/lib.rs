use proc_macro::TokenStream;
use quote::*;

use syn::{parse_macro_input, Data, DeriveInput, Expr, Lit, Visibility};

use indiscriminant_impl_str::*;

type QuoteResult = quote::__private::TokenStream;

fn get_vis(vis: &Visibility) -> QuoteResult {
    match vis {
        Visibility::Public(_) => quote! { pub },
        Visibility::Crate(_) => quote! { pub(crate) },
        _ => quote! {},
    }
}

#[indiscriminant_str()]
#[derive(PartialEq)]
enum IntegerType {
    U8 = "u8",
    U16 = "u16",
    U32 = "u32",
    U64 = "u64",
    U128 = "u128",
}

impl IntegerType {
    fn get_width(&self) -> u8 {
        match self {
            Self::U8 => 8,
            Self::U16 => 16,
            Self::U32 => 32,
            Self::U64 => 64,
            Self::U128 => 128,
        }
    }

    fn value_valid(&self, value: usize) -> bool {
        match self {
            Self::U8 => value < (1 << 8),
            Self::U16 => value < (1 << 16),
            Self::U32 => value < (1 << 32),
            Self::U64 => true,
            Self::U128 => true,
        }
    }

    fn from_expr(&self, expr: &Expr) -> Result<usize, ()> {
        match expr {
            Expr::Lit(lit) => match &lit.lit {
                Lit::Byte(b) => Ok(b.value() as usize),
                Lit::Int(i) => {
                    if i.suffix() != "" && self.to_str() != i.suffix() {
                        return Err(());
                    }
                    let value = match i.base10_parse::<usize>() {
                        Ok(value) => Ok(value),
                        _ => Err(()),
                    }?;

                    if !self.value_valid(value) {
                        return Err(());
                    }

                    Ok(value)
                }
                _ => Err(()),
            },
            _ => Err(()),
        }
    }

    fn quote_discriminant(&self, discriminant: usize) -> QuoteResult {
        match self {
            Self::U8 => {
                let discriminant = discriminant as u8;
                quote! { #discriminant }
            }
            Self::U16 => {
                let discriminant = discriminant as u16;
                quote! { #discriminant }
            }
            Self::U32 => {
                let discriminant = discriminant as u32;
                quote! { #discriminant }
            }
            Self::U64 => {
                let discriminant = discriminant as u64;
                quote! { #discriminant }
            }
            Self::U128 => {
                let discriminant = discriminant as u128;
                quote! { #discriminant }
            }
        }
    }
}

fn parse_args(args: TokenStream) -> (IntegerType, u8) {
    // Parse argument list into integer type and bit-width
    let mut iter = args.into_iter();

    let integer_type = if let Some(arg) = iter.next() {
        match IntegerType::from_str(&arg.to_string()) {
            Some(integer_type) => integer_type,
            None => panic!("Invalid integer type argument!"),
        }
    } else {
        panic!("Expected two macro arguments, found none!");
    };

    if let Some(arg) = iter.next() {
        if arg.to_string() != "," {
            panic!("Macro argument comma not found!");
        }
    } else {
        panic!("Expected comma between arguments, found nothing!");
    }

    let bit_width = if let Some(arg) = iter.next() {
        match arg.to_string().parse::<u8>() {
            Ok(bit_width) => {
                if bit_width > integer_type.get_width() {
                    panic!("Bit-width too large for integer type!");
                }
                bit_width
            }
            Err(_) => panic!("Invalid bit-width!"),
        }
    } else {
        panic!("Expected two macro arguments, found one!");
    };

    if let Some(arg) = iter.next() {
        if arg.to_string() == "," {
            if let Some(_) = iter.next() {
                panic!("Expected two macro arguments, found more!");
            }
        } else {
            panic!("Expected two macro arguments, found more!");
        }
    }

    (integer_type, bit_width)
}

#[proc_macro_attribute]
pub fn indiscriminant_bits_default(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse argument list into integer type and bit-width
    let (integer_type, bit_width) = parse_args(args);

    // Parse enum body
    let input = parse_macro_input!(input as DeriveInput);
    let max_variants = 1 << bit_width;
    let mut variants = Vec::new();
    let mut discriminants = Vec::new();
    let mut has_default = false;
    let data = match input.data {
        Data::Enum(data) => data,
        _ => panic!("Attribute not applied to enum!"),
    };

    for (i, v) in data.variants.iter().enumerate() {
        if v.ident.to_string() == "Default" {
            if i != data.variants.len() - 1 {
                panic!("Default condition present but not at end of list!");
            }
            match &v.discriminant {
                None => {}
                _ => panic!("Default value has discriminant!"),
            }
            if discriminants.len() == max_variants {
                panic!("Cannot have default and all discriminants also covered!");
            }
            let mut default_discriminant = 0;
            for discriminant in &discriminants {
                if default_discriminant != *discriminant {
                    break;
                }
                default_discriminant += 1;
            }
            variants.push(("Default".to_owned(), default_discriminant));
            discriminants.push(default_discriminant);
            has_default = true;
        } else {
            let discriminant = match &v.discriminant {
                Some((_, expr)) => {
                    let discriminant = match integer_type.from_expr(expr) {
                        Ok(discriminant) => discriminant,
                        Err(_) => panic!("Invalid integer discriminant!"),
                    };
                    if discriminant >= max_variants {
                        panic!("Discriminant too big for bit width!");
                    }
                    discriminant
                }
                None => panic!("Non-default value missing discriminant!"),
            };
            variants.push((v.ident.to_string(), discriminant));
            match discriminants.binary_search(&discriminant) {
                Ok(_) => {
                    panic!("Duplicate discriminants found!");
                }
                Err(pos) => discriminants.insert(pos, discriminant),
            }
        }
    }

    if !has_default && discriminants.len() < max_variants {
        panic!("No default and all discriminants are not covered!");
    }
    if discriminants.len() == 0 {
        panic!("Enum is empty of any variants!");
    }

    let name = format_ident!("{}", input.ident.to_string());
    let itype = format_ident!("{}", integer_type.to_str());

    let mut variants_quote = quote!();
    for (variant_name, discriminant) in variants.clone() {
        let variant_name = format_ident!("{}", variant_name);
        variants_quote.extend(quote! { #variant_name = #discriminant as #itype, });
    }

    // Implement functions to convert generated enum to/from integers
    let mut to_matches = quote!();
    let mut from_matches = quote!();
    for (variant_name, discriminant) in &variants {
        let variant_name = format_ident!("{}", variant_name);
        to_matches.extend(quote! { #name::#variant_name => #discriminant as #itype, });

        if variant_name == "Default" {
            from_matches.extend(quote! { _ => #name::#variant_name, });
        } else {
            let discriminant = integer_type.quote_discriminant(*discriminant);
            from_matches.extend(quote! { #discriminant => #name::#variant_name, });
        }
    }
    // Do not include _ case if the integer is completely covered
    if bit_width < integer_type.get_width() && !has_default {
        // We need to pick something to appease the compiler but this will never be used
        let default_variant = format_ident!("{}", variants.iter().next().unwrap().0);
        from_matches.extend(quote! { _ => #name::#default_variant, });
    }

    // Construct resulting struct and impl functions, can debug with `result.to_string()`
    let vis = get_vis(&input.vis);
    let bit_mask: usize = (1 << bit_width) - 1;
    let attrs = input.attrs.iter().map(|attr| quote! { #attr });
    proc_macro::TokenStream::from(quote! {
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

#[proc_macro_attribute]
pub fn indiscriminant_bits(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse argument list into integer type and bit-width
    let (integer_type, bit_width) = parse_args(args);

    // Parse enum body
    let input = parse_macro_input!(input as DeriveInput);
    let max_variants = 1 << bit_width;
    let mut variants = Vec::new();
    let mut discriminants = Vec::new();
    let data = match input.data {
        Data::Enum(data) => data,
        _ => panic!("Attribute not applied to enum!"),
    };

    for (_, v) in data.variants.iter().enumerate() {
        let discriminant = match &v.discriminant {
            Some((_, expr)) => {
                let discriminant = match integer_type.from_expr(expr) {
                    Ok(discriminant) => discriminant,
                    Err(_) => panic!("Invalid integer discriminant!"),
                };
                if discriminant >= max_variants {
                    panic!("Discriminant too big for bit width!");
                }
                discriminant
            }
            None => panic!("Non-default value missing discriminant!"),
        };
        variants.push((v.ident.to_string(), discriminant));
        match discriminants.binary_search(&discriminant) {
            Ok(_) => {
                panic!("Duplicate discriminants found!");
            }
            Err(pos) => discriminants.insert(pos, discriminant),
        }
    }

    if discriminants.len() == 0 {
        panic!("Enum is empty of any variants!");
    }

    let name = format_ident!("{}", input.ident.to_string());
    let itype = format_ident!("{}", integer_type.to_str());

    let mut variants_quote = quote!();
    for (variant_name, discriminant) in variants.clone() {
        let variant_name = format_ident!("{}", variant_name);
        variants_quote.extend(quote! { #variant_name = #discriminant as #itype, });
    }

    // Implement functions to convert generated enum to/from integers
    let mut to_matches = quote!();
    let mut from_matches = quote!();
    for (variant_name, discriminant) in &variants {
        let variant_name = format_ident!("{}", variant_name);
        to_matches.extend(quote! { #name::#variant_name => #discriminant as #itype, });
        let discriminant = integer_type.quote_discriminant(*discriminant);
        from_matches.extend(quote! { #discriminant => Some(#name::#variant_name), });
    }
    if variants.len() < max_variants {
        from_matches.extend(quote! { _ => None, });
    }

    // Construct resulting struct and impl functions, can debug with `result.to_string()`
    let vis = get_vis(&input.vis);
    let bit_mask: usize = (1 << bit_width) - 1;
    let attrs = input.attrs.iter().map(|attr| quote! { #attr });
    proc_macro::TokenStream::from(quote! {
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
