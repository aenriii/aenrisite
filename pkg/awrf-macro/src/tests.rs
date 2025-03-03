#![allow(unused_variables)]
use quote::quote;
use proc_macro2::{Group, Ident, Punct, TokenStream, TokenTree};

use crate::parse::read_statements;
use crate::entities::Statement;

#[test]
fn parse_if_works() {
    let source = quote! {
        if true {

        }
    };

    let statements = read_statements(source.clone());

    // check
    if let Some(Statement::If { expr, inner }) = statements.into_iter().next() {

        let control_token = extract_ident(quote!{ true }.into_iter().next().unwrap());
        let test_token = extract_ident(expr.into_iter().next().unwrap());

        assert_eq!(control_token, test_token);

        assert!(inner.is_empty())
    } else {
        assert!(false)
    }
}

#[test]
fn parse_for_works() {
    let source = quote! {
        for item in items.iter() {

        }
    };

    if let Some(Statement::For {
            var_name,
            iterable_expr,
            inner
        }) = read_statements(source).into_iter().next() {
        let control_name = extract_ident(quote!{ item }.into_iter().next().unwrap());
        let control_iterable_expr = quote! { items.iter() };

        assert_eq!(var_name, control_name);
        assert!(cmp_ts(iterable_expr, control_iterable_expr));
        assert!(inner.is_empty());

    }
    else {
        assert!(false);
    }
}

#[test]
fn parse_use_works() {
    // type 1
    let type_one = {
        let source = quote! {
            use something;
        };
        if let Some(Statement::Use {
            usable_name,
            template_name,
            casted_type
        }) = read_statements(source).into_iter().next() {
            assert!(template_name.is_none());
            assert!(casted_type.is_none());

            let control_name = extract_ident(quote!{ something }.into_iter().next().unwrap());
            assert_eq!(usable_name, control_name);
        }
        else {
            assert!(false);
        }

    };
    let type_two = {
        let source = quote! {
            use something_else as something;
        };
        if let Some(Statement::Use {
            usable_name,
            template_name,
            casted_type
        }) =  read_statements(source).into_iter().next() {
            assert!(casted_type.is_none());

            let control_name = extract_ident(quote!{ something }.into_iter().next().unwrap());
            assert_eq!(usable_name, control_name);

            let control_template_name = Some(extract_ident(quote!{ something_else }.into_iter().next().unwrap()));
            assert_eq!(template_name, control_template_name);
        }
        else {
            assert!(false);
        }
    };
    let type_three = {
        let source = quote! {
            use something : u8;
        };
        if let Some(Statement::Use {
            usable_name,
            template_name,
            casted_type
        }) = read_statements(source).into_iter().next() {
            assert!(template_name.is_none());

            let control_name = extract_ident(quote!{ something }.into_iter().next().unwrap());
            assert_eq!(usable_name, control_name);

            let control_casted_type = Some(extract_ident(quote!{ u8 }.into_iter().next().unwrap()));
            assert_eq!(casted_type, control_casted_type);
        }
        else {
            assert!(false);
        }

    };

    let type_four = {
        let source = quote! {
            use something_else as something: u8;
        };
        if let Some(Statement::Use {
            usable_name,
            template_name,
            casted_type
        }) =  read_statements(source).into_iter().next() {

            let control_name = extract_ident(quote!{ something }.into_iter().next().unwrap());
            assert_eq!(usable_name, control_name);

            let control_template_name = Some(extract_ident(quote!{ something_else }.into_iter().next().unwrap()));
            assert_eq!(template_name, control_template_name);

            let control_casted_type = Some(extract_ident(quote!{ u8 }.into_iter().next().unwrap()));
            assert_eq!(casted_type, control_casted_type);

        }
        else {
            assert!(false);
        }
    };
}

#[test]
fn parse_let_works() {
    let source = quote!{
        let something = me & u;
    };

    if let Some(Statement::Let {
        name,
        expr
    }) = read_statements(source).into_iter().next() {
        let control_name = extract_ident(quote!{ something }.into_iter().next().unwrap());
        let control_expr = quote! { me & u };

        assert_eq!(name, control_name);
        assert!(cmp_ts(expr, control_expr));
    }
}


fn extract_ident(tt: TokenTree) -> Ident {
    match tt {
        TokenTree::Ident(g) => g,
        _ => panic!("what")
    }
}
fn extract_punct(tt: TokenTree) -> Punct {
    match tt {
        TokenTree::Punct(g) => g,
        _ => panic!("what")
    }
}
fn extract_group(tt: TokenTree) -> Group {
    match tt {
        TokenTree::Group(g) => g,
        _ => panic!("what")
    }
}

fn cmp_ts(left: TokenStream, right: TokenStream) -> bool {
    let mut left = left.into_iter().peekable();
    let mut right = right.into_iter().peekable();
    while let Some(l) = left.next() && let Some(r) = right.next() {
        match (l, r) {
            (TokenTree::Group(l), TokenTree::Group(r)) => {
                if !cmp_ts(l.stream(), r.stream()) {
                    return false
                }
            },
            (TokenTree::Ident(l), TokenTree::Ident(r)) => {
                if l != r {
                    return false
                }
            },
            (TokenTree::Punct(l), TokenTree::Punct(r)) => {
                if l.as_char() != r.as_char() {
                    return false;
                }
            },
            (TokenTree::Literal(l), TokenTree::Literal(r)) => {
                // might be wrong sometimes, ok for tests though

                if l.to_string() != r.to_string() {
                    return false;
                }
            },
            _ => return false
        }
    }
    true
}
