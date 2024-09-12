use html_node::{html, text, Node};
use super::root;
#[inline(always)]
pub fn main_layout(children: Node) -> Node {
    root(
        html!(
            <div id="layout"> {children} </div>
        ),
        None, Some(vec!["/main_layout.css".to_string()])
    )
}
