use comrak::nodes::NodeValue;

use crate::{node, HeaderOptions, Node, NodeType, Page, PageOptions};

use core::{mem::discriminant as variant_id, str};
use std::slice;

use super::frontmatter_to_kv;


macro_rules! accumulate_nodes {
    {
        from $iter:ident;
        until $value:ident;
        dont skip;
    } => {
        {
            let mut accum = vec![];
            while let Some(node) = $iter.peek() &&
                  variant_id(&node.data.borrow().value) != variant_id($value)
            {
                if let Some(node) = $iter.next() {

                    accum.push(node.clone());
                }
            }
            seek_nodes(accum)
        }
    };
    {
        from $iter:ident;
        until $value:ident;
        skip;
    } => {
        {
            let mut accum = vec![];
            while let Some(node) = $iter.peek() &&
                  variant_id(&node.data.borrow().value) != variant_id( $value )
            {
                if let Some(node) = $iter.next() {

                    accum.push(node.clone());
                }
            }
            $iter.next();
            seek_nodes(accum)
        }
    };
}

macro_rules! reslice {
    ($vec:ident, $type:ty) => {
        unsafe {
            let parts = $vec.into_raw_parts();
            slice::from_raw_parts(parts.0 as *const $type, parts.1)
        }

    };
}
macro_rules! restring {
    ($str:ident) => {
        unsafe {
            let parts = $str.clone().into_raw_parts();
            str::from_raw_parts(parts.0, parts.1)
        }
    };
}



pub fn compile_page<'func, 'node, 'page>(node: &'func NodeType<'node>) -> Page<'page>
    where 'func : 'node {
    let mut frontmatter = Vec::new();
    let mut nodelist = node.descendants().collect::<Vec<_>>();
    if let Some(node) = nodelist.get(1) && let NodeValue::FrontMatter(fm) = &node.data.borrow().value {
        frontmatter = frontmatter_to_kv(fm.to_string(), "---".to_string());
    }

    let title = frontmatter
        .iter()
        .find(|(k, v)| k == &"title".to_string())
        .map_or("No title".to_string(), |x| x.1.clone());
    let descr = frontmatter
        .iter()
        .find(|(k, v)| k == &"description".to_string())
        .map_or("No description".to_string(), |x| x.1.clone());
    let rest = frontmatter
        .iter_mut()
        .filter_map(|(k,v)| {
            if k == &"title".to_string() || k == &"description".to_string() {
                None
            } else {
                let k = k.clone();
                let v = v.clone();
                Some((restring!(k), restring!(v)))
            }
        })
        .collect::<Vec<_>>();

    let page_opts = PageOptions {
        title: restring!(title),
        description: restring!(descr),
        metadata: reslice!(rest, (&str, &str))
    };

    let nodes = seek_nodes(nodelist);

    return Page { options: page_opts, nodes: reslice!(nodes, Node<'_>) };
}

fn seek_nodes<'func, 'node, 'page>(nodelist: Vec<&'func NodeType<'node>>) -> Vec<Node<'page>> {

    log::trace!("seeking new node");
    let mut nodes = vec![];
    let mut iter = nodelist.iter().peekable();
    while let Some(node) = iter.next() {
        let value = &node.data.borrow_mut().value;

        match value {
            NodeValue::Document | NodeValue::FrontMatter(..) => {}
            NodeValue::Heading(ref head_opts) => {
                println!("implemented: Heading");
                let header_nodes = accumulate_nodes! {
                    from iter;
                    until value;
                    skip;
                };

                let block_nodes = accumulate_nodes! {
                    from iter;
                    until value;
                    dont skip;
                };

                nodes.push(Node::Block {
                    header: HeaderOptions {
                        level: head_opts.level,
                        id: None,
                        nodes: reslice!(header_nodes, Node<'_>)

                    },
                    metadata: &[],
                    nodes: reslice!(block_nodes, Node<'_>)
                });
            },
            NodeValue::Text(ref t) => {
                println!("implemented: Text ({})", t);
                nodes.push(Node::Text { options: None, text: restring!(t) });
            }
            _ => {
                println!("Not implemented: {:?}", value)
            }
        }
    }


    nodes
}
