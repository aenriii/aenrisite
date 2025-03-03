use std::iter::Peekable;

use proc_macro2::{Delimiter, TokenTree};

use crate::entities::Statement;

use super::{stream_until, read_statements, assert_is_block, assert_ident};

pub fn parse_for<T : Iterator<Item = TokenTree>>(tokens: &mut Peekable<T>) -> Statement {
    Statement::For {
        var_name: assert_ident(tokens.next()),
        iterable_expr: {
            let i = assert_ident(tokens.next());
            if i.to_string() != String::from("in") {
                panic!("expected token 'in', found {}", i);
            }
            stream_until(&mut tokens, |tt| {
                println!("reading {:?}", tt);
                if let TokenTree::Group(g) = tt {
                    return g.delimiter() != Delimiter::Brace
                }
                true
            })
        },
        inner: read_statements(assert_is_block(tokens.next()).stream())
    }
}
