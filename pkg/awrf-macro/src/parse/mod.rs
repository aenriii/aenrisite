use std::iter::Peekable;

use proc_macro2::{Delimiter, TokenStream, TokenTree};
use quote::ToTokens;

use crate::entities::Statement;

mod stmt_if;
use stmt_if::parse_if;

mod stmt_for;
use stmt_for::parse_for;

mod stmt_use;
use stmt_use::parse_use;

pub fn read_statements(tt: TokenStream) -> Vec<Statement> {
    let mut statements = vec![];

    let mut tokens: Peekable<_> = tt.into_iter().peekable();
    while let Some(token) = tokens.next() {
        // println!("{:?}", token);
        match token {
            TokenTree::Ident(ident) => {
                match ident.to_string().as_str() {
                    "if" => {
                        statements.push(parse_if(&mut tokens));
                    },
                    "for" => {
                        statements.push(parse_for(&mut tokens))
                    },
                    "use" => {
                        statements.push(parse_use(&mut tokens))
                    },
                    "let" => {
                        statements.push({
                            let name = assert_ident(tokens.next());
                            if let Some(TokenTree::Punct(p)) = tokens.next() {
                                if p.as_char() != '=' {
                                    panic!("expected =, found {}", p.as_char());
                                }
                            } else {
                                panic!("expected =");
                            }
                            Statement::Let {
                                name,
                                expr: stream_until(&mut tokens, |tt| {
                                    if let TokenTree::Punct(p) = tt {
                                        return p.as_char() != ';'
                                    }
                                    true
                                })
                            }
                        });
                    }
                    _ => {
                        // patterns
                        // - {ComponentName} ({args / flags, ...})
                        // - {ComponentName} ({args / flags, ...}) { inner block }

                    }
                }
            },
            TokenTree::Punct(p) => {
                match p.as_char() {
                    ';' => {},
                    _ => unimplemented!()
                }
            },
            _ => unimplemented!()
        }
    }
    statements
}
/// Takes in a TokenTree iterator and a predicate, returns a TokenStream of
/// every token until pred returns false
pub(self) fn stream_until<T : Iterator<Item = TokenTree>>(tt: &mut Peekable<T>, pred: fn (&TokenTree) -> bool) -> TokenStream {
    let mut acc = vec![];
    while let Some(it) = tt.peek() && pred(&it) {
        acc.push(tt.next().unwrap().into_token_stream());
    }
    TokenStream::from_iter(acc)
}

pub(self) fn assert_is_block(tt: Option<TokenTree>) -> proc_macro2::Group {
    match tt {
        Some(TokenTree::Group(g)) => {
            if g.delimiter() != Delimiter::Brace {
                panic!("Expected block, wrong delim")
            }
            return g;
        },
        Some(it) => panic!("Expected block, found {}", it),
        None => panic!("Expected block")
    }
}
pub(self) fn assert_is_paren(tt: Option<TokenTree>) -> proc_macro2::Group {
    match tt {
        Some(TokenTree::Group(g)) => {
            if g.delimiter() != Delimiter::Parenthesis {
                panic!("Expected block (), wrong delim")
            }
            return g;
        },
        Some(it) => panic!("Expected block (), found {}", it),
        None => panic!("Expected block ()")
    }
}

pub(self) fn assert_ident(tt: Option<TokenTree>) -> proc_macro2::Ident {
    match tt {
        Some(TokenTree::Ident(g)) => return g,
        Some(it) => panic!("Expected identifier, found {}", it),
        None => panic!("Expected identifier")
    }
}
