#![feature(let_chains, proc_macro_span)]
extern crate proc_macro;

use std::fs;

use comrak::{nodes::NodeValue, parse_document, Arena, ExtensionOptionsBuilder, Options};
use proc_macro2::{Span, TokenStream, TokenTree::Literal};
use quote::ToTokens;

mod token;

#[proc_macro]
pub fn marker(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tokens = TokenStream::from(tokens);

    let path = match token::process::extract_next_path(tokens, true) {
        Some(path) => if !path.exists() || !path.is_file() {
            return token::fail(Span::call_site(), "Failed to get suitable path from macro invocation")
        } else { path },
        None => return token::fail(Span::call_site(), "Failed to get suitable path from macro invocation")
    };

    let content = match fs::read_to_string(path) {
        Ok(string) => string,
        Err(err) => return token::fail(Span::call_site(), format!("Failed to read file while performing conversion, {:?}", err))
    };

    return match marker::parse_page(content) {
        Ok(page) => return quote! {{
            use marker::*;
            #page
        }}.into(),
        Err(str) => token::fail(Span::call_site(), format!("Failed to generate Page, {}", str))
    }
}
