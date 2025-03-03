use std::iter::Peekable;

use proc_macro2::{Delimiter, TokenTree};

use crate::entities::Statement;

use super::{stream_until, read_statements, assert_is_block, assert_ident};

pub fn parse_use<T : Iterator<Item = TokenTree>>(tokens: &mut Peekable<T>) -> Statement {
    // patterns:
    // - use {0}
    // - use {0} as {1}
    // - use {0} : {1}
    // - use {0} as {1} : {2}

    let mut usable_name;
    let mut template_name = None;
    let mut casted_type = None;

    let zero = assert_ident(tokens.next());
    // cover our ass with the borrow checker
    usable_name = zero.clone();
    match tokens.peek() {
        Some(TokenTree::Ident(it)) => {
            match it.to_string().as_str() {
                "as" => {
                    // consume
                    tokens.next();
                    // next should be ident
                    usable_name = assert_ident(tokens.next());
                    template_name = Some(zero);
                    // peek to see if next is :

                    match tokens.peek() {
                        Some(TokenTree::Punct(p)) => {
                            match p.as_char() {
                                ':' => {
                                    // consume
                                    tokens.next();
                                    // next should be ident
                                    casted_type = Some(assert_ident(tokens.next()));
                                },
                                _ => {}
                            }
                        },
                        _ => {}
                    }

                },
                _ => {}
            }
        },
        Some(TokenTree::Punct(p)) => {
            match p.as_char() {
                ':' => {
                    // consume
                    tokens.next();
                    // next should be ident
                    usable_name = zero;
                    casted_type = Some(assert_ident(tokens.next()));
                },
                _ => {}
            }
        },
        _ => {
            usable_name = zero;
        }
    };
    Statement::Use {
        usable_name,
        template_name,
        casted_type
    }
}
