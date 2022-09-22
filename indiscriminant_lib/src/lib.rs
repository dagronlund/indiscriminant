pub mod indiscriminant_bits;
pub mod indiscriminant_byte_str;
pub mod indiscriminant_str;

use quote::*;

use syn::{Expr, Lit, Visibility};

type QuoteResult = quote::__private::TokenStream;

pub(crate) fn get_vis(vis: &Visibility) -> QuoteResult {
    match vis {
        Visibility::Public(_) => quote! { pub },
        Visibility::Crate(_) => quote! { pub(crate) },
        _ => quote! {},
    }
}

#[derive(PartialEq, Clone)]
enum IntegerType {
    U8,
    U16,
    U32,
    U64,
    U128,
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

    fn to_str(&self) -> &'static str {
        match self {
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::U128 => "u128",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s {
            "u8" => Some(Self::U8),
            "u16" => Some(Self::U16),
            "u32" => Some(Self::U32),
            "u64" => Some(Self::U64),
            "u128" => Some(Self::U128),
            _ => None,
        }
    }
}
