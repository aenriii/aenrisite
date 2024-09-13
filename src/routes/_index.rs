use crate::{components::nojs_tabs, layouts::main_layout};

use actix_web::{get, Result};
use const_random::const_random;
use maud::{html, Markup};

const DEFAULT_NONCE: u128 = const_random!(u128);
const AUTHENTICATED_NONCE: u128 = const_random!(u128);

#[get("/")]
pub async fn index() -> Result<Markup> {
    Ok(main_layout(nojs_tabs(
        vec![
            (
                String::from("About"),
                html! {
                    h1 { "About" }
                },
            ),
            (
                String::from("Projects"),
                html! {
                    h1 { "Projects" }
                },
            ),
            (
                String::from("Blog"),
                html! {
                    h1 { "Blog" }
                },
            ),
        ],
        DEFAULT_NONCE,
    )))
}
