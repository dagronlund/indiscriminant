use proc_macro::TokenStream;
use quote::*;

use syn::{parse_macro_input, Data, DeriveInput, Expr, Lit, Visibility};

use no_discrimination_impl_str::*;

#[no_discrimination_str()]
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
            IntegerType::U8 => 8,
            IntegerType::U16 => 16,
            IntegerType::U32 => 32,
            IntegerType::U64 => 64,
            IntegerType::U128 => 128,
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
}

#[proc_macro_attribute]
pub fn no_discrimination_bits(args: TokenStream, input: TokenStream) -> TokenStream {
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

    // Parse enum body
    let input = parse_macro_input!(input as DeriveInput);
    let max_variants = 1 << bit_width;
    let mut variants = Vec::new();
    let mut discriminants = Vec::new();
    let mut has_default = false;
    let mut last_discriminant: isize = -1;
    let data = match input.data {
        Data::Enum(data) => data,
        _ => panic!("Attribute not applied to enum!"),
    };

    for (i, v) in data.variants.iter().enumerate() {
        if v.ident.to_string() == "_Default" {
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
            variants.push(("_Default".to_owned(), default_discriminant));
            discriminants.push(default_discriminant);
            has_default = true;
        } else {
            let discriminant = match &v.discriminant {
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

        if variant_name == "_Default" {
            from_matches.extend(quote! { _ => #name::#variant_name, });
        } else {
            match integer_type {
                IntegerType::U8 => {
                    let discriminant = *discriminant as u8;
                    from_matches.extend(quote! { #discriminant => #name::#variant_name, });
                }
                IntegerType::U16 => {
                    let discriminant = *discriminant as u16;
                    from_matches.extend(quote! { #discriminant => #name::#variant_name, });
                }
                IntegerType::U32 => {
                    let discriminant = *discriminant as u32;
                    from_matches.extend(quote! { #discriminant => #name::#variant_name, });
                }
                IntegerType::U64 => {
                    let discriminant = *discriminant as u64;
                    from_matches.extend(quote! { #discriminant => #name::#variant_name, });
                }
                IntegerType::U128 => {
                    let discriminant = *discriminant as u128;
                    from_matches.extend(quote! { #discriminant => #name::#variant_name, });
                }
            }
        }
    }

    // Do not include _ case if the integer is completely covered
    let from_match_default = if bit_width == integer_type.get_width() || has_default {
        quote! {}
    } else {
        // We need to pick something to appease the compiler but this will never be used
        let default_variant = format_ident!("{}", variants.iter().next().unwrap().0);
        quote! { _ => #name::#default_variant, }
    };

    // Figure out visibility
    let vis = match input.vis {
        Visibility::Public(_) => quote! { pub },
        Visibility::Crate(_) => quote! { pub(crate) },
        _ => quote! {},
    };

    // Construct resulting struct and impl functions, can debug with `result.to_string()`
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
                    #from_match_default
                }
            }
        }
    })
}
