//! ISO 10383 Market Identifier Codes Macros

mod xml;

use proc_macro::TokenStream;

/// Generate the Code enum and static records data.
///
/// # Panics
///
/// - If there was an error reading the given XML file or parsing the contents.
#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    xml::generate(input.into())
        .expect("Could not generate code")
        .into()
}
