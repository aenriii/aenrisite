use std::{cell::RefCell, mem::MaybeUninit};

use comrak::nodes::{Ast, NodeValue};

use crate::Page;

mod frontmatter;
pub use frontmatter::frontmatter_to_kv;

type NodeType<'a> = comrak::arena_tree::Node<'a, RefCell<Ast>>;

struct PageIntermediateRepr {
    frontmatter: Vec<(String, String)>,

}

impl <'a> TryInto<Page<'a>> for &'a mut NodeType<'a> {
    type Error = String;

    fn try_into(self) -> Result<Page<'a>, Self::Error> {
        let intermediate_repr = extract_data(self)?;
        return construct(intermediate_repr);
    }
}


fn extract_data<'a>(from: &'a mut NodeType<'a>) -> Result<PageIntermediateRepr, String> {
    let mut frontmatter = MaybeUninit::uninit();

    let mut decendants = from.descendants();
    while let Some(node) = decendants.next() {
        let data = node.data.borrow();
        match data.value.clone() {
            NodeValue::FrontMatter(fm) => {
                frontmatter.write(frontmatter_to_kv(fm, "---".to_string()));
            }

            _ => {
                println!("Not dealing with {:?}", data.value);
            }
        }
    }

    todo!();
}

fn next_node<'a>(from: &'a mut NodeType<'a>) -> crate::Node<'a> {
    todo!();
}

fn construct<'a>(with: PageIntermediateRepr) -> Result<Page<'a>, String> {
    todo!();
}
