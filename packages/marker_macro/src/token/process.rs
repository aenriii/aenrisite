use std::path::{Path, PathBuf};

use litrs::StringLit;
use proc_macro2::{TokenStream, TokenTree};


pub fn extract_next_path(token_stream: TokenStream, exists: bool) -> Option<PathBuf> {

    for token in token_stream.into_iter() {
        match token {
            TokenTree::Literal(lit) => {
                // i love litrs
                if let Ok(string_lit) = StringLit::try_from(lit) {
                    // test path

                    let path = string_lit.into_value();

                    // TODO: actually test if valid path
                    let path = PathBuf::from(path.to_string());
                    if exists {
                        if path.exists() {
                            return Some(path);
                        }
                    } else {
                        return Some(path)
                    }
                }
            },

            _ => {}
        }
    }
    None


}
