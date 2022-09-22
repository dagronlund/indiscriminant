use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use syn::{parse_macro_input, Data, DeriveInput, Expr, Lit};

use indiscriminant_lib::indiscriminant_bits;
use indiscriminant_lib::indiscriminant_byte_str;
use indiscriminant_lib::indiscriminant_str;

#[proc_macro_attribute]
pub fn indiscriminant_str(args: TokenStream, input: TokenStream) -> TokenStream {
    TokenStream::from(indiscriminant_str::indiscriminant_str(
        TokenStream2::from(args),
        TokenStream2::from(input),
    ))
}

#[proc_macro_attribute]
pub fn indiscriminant_byte_str(args: TokenStream, input: TokenStream) -> TokenStream {
    TokenStream::from(indiscriminant_byte_str::indiscriminant_byte_str(
        TokenStream2::from(args),
        TokenStream2::from(input),
    ))
}

#[proc_macro_attribute]
pub fn indiscriminant_bits(args: TokenStream, input: TokenStream) -> TokenStream {
    TokenStream::from(indiscriminant_bits::indiscriminant_bits(
        TokenStream2::from(args),
        TokenStream2::from(input),
    ))
}

#[proc_macro_attribute]
pub fn indiscriminant(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_cloned = input.clone();
    // Parse enum body
    let input_cloned = parse_macro_input!(input_cloned as DeriveInput);
    let data = match input_cloned.data {
        Data::Enum(data) => data,
        _ => panic!("Attribute not applied to enum!"),
    };
    assert!(data.variants.len() > 0, "Enum is empty of any variants!");

    let v = data.variants.iter().next().unwrap();

    let literal = match &v.discriminant {
        Some((_, Expr::Lit(literal))) => literal,
        _ => panic!("Literal not found for first discriminant!"),
    };

    let args = TokenStream2::from(args);
    let input = TokenStream2::from(input);
    let result = match &literal.lit {
        Lit::Str(_) => indiscriminant_str::indiscriminant_str(args, input),
        Lit::ByteStr(_) => indiscriminant_byte_str::indiscriminant_byte_str(args, input),
        Lit::Byte(_) | Lit::Int(_) => indiscriminant_bits::indiscriminant_bits(args, input),
        _ => panic!("First literal is not string, byte-string, or integer!"),
    };
    TokenStream::from(result)
}
