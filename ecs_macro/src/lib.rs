use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput); // 1. Use syn to parse the input tokens into a syntax tree.

    // get the name of the type we want to implement the trait for
    let name = &input.ident;

    let expanded = quote! {
      impl crate::components::Component for #name {

      }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(GameEvent)]
pub fn game_event_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput); // 1. Use syn to parse the input tokens into a syntax tree.

    // get the name of the type we want to implement the trait for
    let name = &input.ident;

    let expanded = quote! {
      impl crate::events::GameEvent for #name {

      }
    };

    TokenStream::from(expanded)
}
