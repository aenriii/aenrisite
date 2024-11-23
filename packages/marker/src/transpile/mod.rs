use std::{cell::RefCell, mem::MaybeUninit};

use comrak::nodes::{Ast, NodeValue};

use crate::Page;

mod frontmatter;
pub use frontmatter::frontmatter_to_kv;

pub mod md;

pub(crate) type NodeType<'a> = comrak::arena_tree::Node<'a, RefCell<Ast>>;
