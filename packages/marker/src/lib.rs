
// used for cloning references to allow
// recursion without increasing reference
// weight (preventing &T -> &&T)
#![allow(suspicious_double_ref_op)]

// let_chains used for complex if/while statements
//
// vec_into_raw_parts used for managed conversions
// between Vec<T> and &'_ [T] to allow for dynamic
// node creation while scanning markdown ast and
// placing it into a Node struct, which requires
// &'_ [T] as opposed to Vec
//
// str_from_raw_parts is used for converting a
// &String into a &'_ str to be provided to a Node
#![feature(let_chains, vec_into_raw_parts, str_from_raw_parts)]

pub mod def;

pub mod page;
pub mod node;
pub mod tokens;
pub mod transpile;

use comrak::nodes::Ast;
pub use page::{Page, PageOptions};
pub use node::{Node, HeaderOptions, TextOptions};


use comrak::{parse_document, Arena, ExtensionOptionsBuilder, Options};
use comrak::arena_tree::Node as ComrakNode;
use std::borrow::BorrowMut;
use std::cell::RefCell;

type NodeType<'a> = ComrakNode<'a, RefCell<Ast>>;


pub fn parse_page<'a>(content: String) -> Result<Page<'a>, String> {
    let _arena = Arena::new();

    let options = Options {
        extension: ExtensionOptionsBuilder::default()
            .front_matter_delimiter(Some("---".to_string()))
            .build()
            .unwrap(),
        ..Options::default()
    };
    let mut root = parse_document(&_arena, &content, &options);

    log::trace!("parsed document");

    return match Page::try_from_node(root) {
        Ok(page) => Ok(page),
        Err(err) => Err(err.clone())
    }
}
