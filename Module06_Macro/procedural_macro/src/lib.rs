use proc_macro::TokenStream; // Import the TokenStream type for procedural macros
use quote::quote; // Import the quote macro for generating code
use syn::{parse_macro_input, Ident}; // Import necessary types from syn

#[proc_macro]
pub fn make_function(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Ident);
    
    let expanded = quote! {
        fn #name() -> &'static str {
            "Who the hell are you proc!"
        }
    };
    
    TokenStream::from(expanded)
}