#![feature(let_chains)]
pub mod page;
pub mod node;
pub mod tokens;
pub mod transpile;

pub use page::{Page, PageOptions};
pub use node::{Node, HeaderOptions, TextOptions};


use comrak::{parse_document, Arena, ExtensionOptionsBuilder, Options};
pub fn parse_page<'a>(content: String) -> Result<Page<'a>, String> {
    let _arena = Arena::new();

    let options = Options {
        extension: ExtensionOptionsBuilder::default()
            .front_matter_delimiter(Some("---".to_string()))
            .build()
            .unwrap(),
        ..Options::default()
    };
    let root = parse_document(&_arena, &content, &options);
    return Ok(root.into())
}
