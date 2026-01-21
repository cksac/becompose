//! BECOMPOSE Procedural Macros
//!
//! This crate provides the `#[composable]` attribute macro for defining
//! composable functions in the BECOMPOSE framework.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Marks a function as a composable function.
///
/// Composable functions are the building blocks of BECOMPOSE UIs.
/// They describe UI elements and can be recomposed when state changes.
///
/// # Example
///
/// ```rust
/// use becompose::prelude::*;
///
/// #[composable]
/// fn greeting(name: &str) {
///     text(format!("Hello, {}!", name));
/// }
/// ```
#[proc_macro_attribute]
pub fn composable(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let expanded = expand_composable(input);
    TokenStream::from(expanded)
}

fn expand_composable(input: ItemFn) -> TokenStream2 {
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;

    let fn_name = &sig.ident;
    let inputs = &sig.inputs;
    let output = &sig.output;
    let generics = &sig.generics;
    let where_clause = &sig.generics.where_clause;

    // Generate a unique type ID based on function name
    let type_id_str = fn_name.to_string();

    quote! {
        #(#attrs)*
        #vis fn #fn_name #generics (#inputs) #output #where_clause {
            use becompose::composition::CompositionContext;

            let __ctx = CompositionContext::current();
            let __node_id = __ctx.start_group(#type_id_str, None);

            let __result = (|| {
                #block
            })();

            __ctx.end_group(__node_id);
            __result
        }
    }
}
