use proc_macro2::{Ident, TokenStream, TokenTree};


pub enum Statement {
    If {
        expr: TokenStream,
        inner: Vec<Statement>
    },
    For {
        var_name: Ident,
        iterable_expr: TokenStream,
        inner: Vec<Statement>
    },
    Use {
        usable_name: Ident,
        template_name: Option<Ident>,
        casted_type: Option<Ident>
    },
    Let {
        name: Ident,
        expr: TokenStream
    },
    Component {
        component_name: TokenTree,
        arguments: Option<TokenStream>,
        inner: Vec<Statement>
    }
}

