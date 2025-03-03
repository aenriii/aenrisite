use std::iter::Peekable;

use proc_macro2::{Delimiter, TokenTree};

use crate::entities::Statement;

use super::{stream_until, read_statements, assert_is_block};

pub fn parse_if<T : Iterator<Item = TokenTree>>(tokens: &mut Peekable<T>) -> Statement {
    Statement::If {
        expr: stream_until(tokens, |tt| {
            println!("reading {:?}", tt);
            if let TokenTree::Group(g) = tt {
                return g.delimiter() != Delimiter::Brace
            }
            true
        }),
        inner: read_statements(assert_is_block(tokens.next()).stream())
    }
}
