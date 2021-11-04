use proc_macro::TokenStream;
use quote::*;
use std::collections::HashMap;

use syn::{parse_macro_input, Data, DeriveInput, Expr, Lit, Visibility};

enum IntegerType {
    U8,
    U16,
    U32,
    U64,
    U128,
}

impl IntegerType {
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "u8" => Ok(IntegerType::U8),
            "u16" => Ok(IntegerType::U16),
            "u32" => Ok(IntegerType::U32),
            "u64" => Ok(IntegerType::U64),
            "u128" => Ok(IntegerType::U128),
            _ => Err(()),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            IntegerType::U8 => "u8",
            IntegerType::U16 => "u16",
            IntegerType::U32 => "u32",
            IntegerType::U64 => "u64",
            IntegerType::U128 => "u128",
        }
    }

    fn width_valid(&self, width: u8) -> bool {
        match self {
            IntegerType::U8 => width <= 8,
            IntegerType::U16 => width <= 16,
            IntegerType::U32 => width <= 32,
            IntegerType::U64 => width <= 64,
            IntegerType::U128 => width <= 128,
        }
    }

    fn width_match(&self, width: u8) -> bool {
        match self {
            IntegerType::U8 => width == 8,
            IntegerType::U16 => width == 16,
            IntegerType::U32 => width == 32,
            IntegerType::U64 => width == 64,
            IntegerType::U128 => width == 128,
        }
    }

    fn suffix_valid(&self, s: &str) -> bool {
        match self {
            IntegerType::U8 => s == "u8",
            IntegerType::U16 => s == "u16",
            IntegerType::U32 => s == "u32",
            IntegerType::U64 => s == "u64",
            IntegerType::U128 => s == "u128",
        }
    }

    fn value_valid(&self, value: usize) -> bool {
        match self {
            IntegerType::U8 => value < (1 << 8),
            IntegerType::U16 => value < (1 << 16),
            IntegerType::U32 => value < (1 << 32),
            IntegerType::U64 => true,
            IntegerType::U128 => true,
        }
    }

    fn from_expr(&self, expr: Expr) -> Result<usize, ()> {
        match expr {
            Expr::Lit(lit) => match lit.lit {
                Lit::Byte(b) => Ok(b.value() as usize),
                Lit::Int(i) => {
                    if i.suffix() != "" && !self.suffix_valid(i.suffix()) {
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
}

fn no_discrimination_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse argument list into integer type and bit-width
    let mut i = 0;
    let mut bit_width = 0;
    let mut integer_type: IntegerType = IntegerType::U8;

    for arg in args {
        match i {
            0 => {
                integer_type = match IntegerType::from_str(&arg.to_string()) {
                    Ok(l) => l,
                    _ => panic!("Invalid integer type argument!"),
                }
            }
            1 => {
                if arg.to_string() != "," {
                    panic!("Macro argument comma not found!");
                }
            }
            2 => {
                bit_width = match arg.to_string().parse::<u8>() {
                    Ok(i) => {
                        if !integer_type.width_valid(i) {
                            panic!("Bit-width too large for integer type!");
                        }
                        i
                    }
                    Err(_) => panic!("Invalid bit-width!"),
                }
            }
            _ => panic!("Too many macro arguments!"),
        }
        i += 1;
    }

    // Parse enum body
    let input = parse_macro_input!(input as DeriveInput);
    let max_variants = 1 << bit_width;
    let mut variants = HashMap::new();
    let mut discriminants = Vec::new();
    let mut has_default = false;
    let mut last_discriminant: isize = -1;
    match input.data {
        Data::Enum(data) => {
            for v in data.variants {
                if v.ident.to_string() == "Default" {
                    if has_default {
                        panic!("Default condition not at end of list!");
                    }
                    has_default = true;
                    continue;
                }
                let discriminant = match v.discriminant {
                    Some((_, expr)) => {
                        let discriminant = match integer_type.from_expr(expr) {
                            Ok(discriminant) => discriminant,
                            Err(_) => panic!("Invalid integer discriminant!"),
                        };
                        if v.ident.to_string() == "Default" {
                            panic!("Default value has discriminant!");
                        }
                        if discriminant >= max_variants {
                            panic!("Discriminant too big for bit width!");
                        }
                        last_discriminant = discriminant as isize;
                        discriminant
                    }
                    None => {
                        last_discriminant += 1;
                        last_discriminant as usize
                    }
                };
                variants.insert(v.ident.to_string(), discriminant);
                match discriminants.binary_search(&discriminant) {
                    Ok(_) => {
                        panic!("Duplicate discriminants found!");
                    }
                    Err(pos) => discriminants.insert(pos, discriminant),
                }
            }
        }
        _ => panic!("Attribute not applied to enum!"),
    }

    if has_default && discriminants.len() == max_variants {
        panic!("Cannot have default and all discriminants also covered!");
    }
    if !has_default && discriminants.len() != max_variants {
        panic!("No default and all discriminants are not covered!");
    }
    if !has_default && discriminants.len() == 0 {
        panic!("Enum is empty of any variants!");
    }

    let mut result = quote!();
    let name = format_ident!("{}", input.ident.to_string());
    let itype = format_ident!("{}", integer_type.to_str());

    let mut variants_quote = quote!();
    for (variant_name, discriminant) in variants.clone() {
        let variant_name = format_ident!("{}", variant_name);
        variants_quote.extend(quote! { #variant_name = #discriminant as #itype, });
    }

    if has_default {
        let mut default_discriminant = 0;
        for discriminant in discriminants {
            if default_discriminant != discriminant {
                break;
            }
            default_discriminant += 1;
        }
        variants_quote.extend(quote! { Default = #default_discriminant as #itype, });
    }

    result.extend(quote! { #[repr(#itype)] });

    match input.vis {
        Visibility::Public(_) => result.extend(quote! {
            pub enum #name {
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

    let mut to_int_matches = quote!();
    let mut from_int_matches = quote!();

    for (variant_name, discriminant) in variants.clone() {
        let variant_name = format_ident!("{}", variant_name);
        to_int_matches.extend(quote! { #name::#variant_name => #discriminant as #itype, });

        match integer_type {
            IntegerType::U8 => {
                let discriminant = discriminant as u8;
                from_int_matches.extend(quote! { #discriminant => #name::#variant_name, });
            }
            IntegerType::U16 => {
                let discriminant = discriminant as u16;
                from_int_matches.extend(quote! { #discriminant => #name::#variant_name, });
            }
            IntegerType::U32 => {
                let discriminant = discriminant as u32;
                from_int_matches.extend(quote! { #discriminant => #name::#variant_name, });
            }
            IntegerType::U64 => {
                let discriminant = discriminant as u64;
                from_int_matches.extend(quote! { #discriminant => #name::#variant_name, });
            }
            IntegerType::U128 => {
                let discriminant = discriminant as u128;
                from_int_matches.extend(quote! { #discriminant => #name::#variant_name, });
            }
        }
    }

    let default_variant = if has_default {
        format_ident!("{}", "Default")
    } else {
        format_ident!("{}", variants.keys().next().unwrap())
    };

    let to_int_match = quote! {
        match self {
            #to_int_matches
            _ => { 0 }
        }
    };
    // Do not include _ case if the integer is completely covered
    let from_int_match = if integer_type.width_match(bit_width) && !has_default {
        quote! {
            match masked_value {
                #from_int_matches
            }
        }
    } else {
        quote! {
            match masked_value {
                #from_int_matches
                _ => #name::#default_variant,
            }
        }
    };

    let bit_mask: usize = (1 << bit_width) - 1;

    match input.vis {
        Visibility::Public(_) => {
            result.extend(quote! {
                impl #name {
                    pub fn to_int(&self) -> #itype { #to_int_match }
                    pub fn from_int(value: #itype) -> Self { let masked_value = #bit_mask as #itype & value; #from_int_match }
                }
            });
        }
        _ => {
            result.extend(quote! {
                impl #name {
                    fn to_int(&self) -> #itype { #to_int_match }
                    fn from_int(value: #itype) -> Self { let masked_value = #bit_mask as #itype & value; #from_int_match }
                }
            });
        }
    }

    // println!("{}", result.to_string());
    proc_macro::TokenStream::from(result)
}

#[proc_macro_attribute]
pub fn no_discrimination(args: TokenStream, input: TokenStream) -> TokenStream {
    no_discrimination_impl(args, input)
}

// #[proc_macro_attribute]
// pub fn no_discrimination_safe(args: TokenStream, input: TokenStream) -> TokenStream {
//     no_discrimination_impl(args, input, true)
// }
