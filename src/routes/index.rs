use crate::layouts::main_layout;
use html_node::{html, Node};

pub fn index() -> Node {
    main_layout(
        html!(
            <h1> "My Awesome Website" </h1>
        )
    )
}
