#![allow(soft_unstable)]
extern crate test;
use crate::layouts::root;
use html_node::html;
use test::Bencher;

#[bench]
fn basic_html(b: &mut Bencher) {
    b.iter(|| {
        root(
            html! (
                <p> "Hello World!" </p>
            ),
            None,
            None,
        )
    })
}
