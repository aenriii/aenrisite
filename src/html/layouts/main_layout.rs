use super::root;
use cached::proc_macro::cached;
use maud::{html, Markup};
#[inline(always)]
// #[cached(key = "String", convert = r#"{ children.clone().into_string() }"#)]
pub fn main_layout(children: Markup) -> Markup {
    root(
        html! {
            div id="layout" { (children) }
        },
        None,
        Some(vec!["/main_layout.css".to_string()]),
    )
}
