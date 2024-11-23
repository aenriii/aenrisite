use crate::{node::Node, transpile::{self, NodeType}};


#[derive(Debug, Clone, Copy)]
pub struct Page<'a> {
    pub options: PageOptions<'a>,
    pub nodes: &'a [Node<'a>]
}

#[derive(Debug, Clone, Copy)]
pub struct PageOptions<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub metadata: &'a[(&'a str, &'a str)]
}

impl <'a> Page<'a> {
    pub fn try_from_node<'b>(node: &'b NodeType<'b>) -> Result<Self, String> {
        log::trace!("try_from_node received node");
        return Ok(transpile::md::compile_page(node));
    }
}
