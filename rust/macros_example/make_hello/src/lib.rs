extern crate proc_macro;

use proc_macro::TokenStream;
//use quote::quote;

#[proc_macro_attribute]
pub fn make_hello(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate

    // Build the trait implementation
    item
}
