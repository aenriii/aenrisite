use crate::layouts::tabbed_layout;
use actix_web::{get, Result};
use const_random::const_random;
use maud::{html, Markup};

const DEFAULT_NONCE: u128 = const_random!(u128);
const AUTHENTICATED_NONCE: u128 = const_random!(u128);

#[get("/")]
pub async fn index() -> Result<Markup> {
    Ok(tabbed_layout! {
        nonce = DEFAULT_NONCE
        "About" html! {
            h1 { "about" }
        }
        "Projects" html! {
            h1 { "projects" }
        }
        "Blog" html! {
            h1 { "blog" }
        }
    })
}
