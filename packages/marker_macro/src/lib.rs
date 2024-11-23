#![feature(let_chains, proc_macro_span)]
extern crate proc_macro;

use std::{fs, path::PathBuf};

use proc_macro2::{Span, TokenStream, TokenTree::Literal};
use quote::quote;

mod token;

#[proc_macro]
pub fn marker(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {

    log::trace!("proc_macro marker entry");

    let tokens = TokenStream::from(tokens);
    {
        let mut p = PathBuf::new();
        p.push(".");
        match p.canonicalize() {
            Ok(pa) => {
                println!("cwd {}", pa.to_string_lossy())
            },
            Err(er) => {
                println!("what, {}", er)
            }
        }
    }
    let path = match token::process::extract_next_path(tokens, false) {
        Some(path) => if !path.exists() || !path.is_file() {
            return token::fail(Span::call_site(), "Failed to get suitable path from macro invocation")
        } else { path },
        None => return token::fail(Span::call_site(), "Failed to get path from macro invocation")
    };

    log::info!("marker processing file {}", path.to_string_lossy());

    let content = match fs::read_to_string(path) {
        Ok(string) => string,
        Err(err) => return token::fail(Span::call_site(), format!("Failed to read file while performing conversion, {:?}", err))
    };

    log::trace!("passing file content to marker::parse_page");

    return match marker::parse_page(content) {
        Ok(page) => return quote! {{
            use marker::*;
            #page
        }}.into(),
        Err(str) => token::fail(Span::call_site(), format!("Failed to generate Page, {}", str))
    }
}
