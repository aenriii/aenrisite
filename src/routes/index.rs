use std::collections::HashMap;

use actix_web::{get, App, HttpServer, Result as AwResult};
use cached::proc_macro::cached;
use const_random::const_random;
use maud::{html, Markup};

use crate::{collection, frames};

const CACHE_KEY: u128 = const_random!(u128);

#[get("/")]
pub async fn homepage() -> AwResult<Markup> {
    Ok(element(CACHE_KEY))
}

#[inline(always)]
#[cached(key = "u128", convert = r##"{ cache_nonce }"##, time = 60)]
fn element(cache_nonce: u128) -> Markup {
    let tabs: HashMap<String, (Markup, Markup)> = collection! {
        String::from("home") => collection! {
            html! {
                div.home-sidebar-content {
                    img.avatar-image src="/assets/avatar.png" alt="Avatar Image";
                    div.home-sidebar-content-container {
                        span.h1 { ("hi, im aenri") };
                        div.home-sidebar-description {
                            span { ("pup/it / θ∆" )};
                            span { ("bodily 19, ageless" )};
                            span { ("pro-para anti-harm" )};
                            span { ("poly puppy developer" )};
                        };
                        span.h2 { ("links") };
                        div.links {
                            // TODO
                        }
                    }
                }

            },
            html! {
                div.home-content {
                    div.home-blog-post-container {
                        div.home-blog-post-alert {
                            span.text { ("new blog post!") };
                        };
                        div.home-blog-post {
                            div.home-blog-post-header {
                                span.title.h2 { ("you don't need javascript") };
                                span.sep { ("-") };
                                span.date.subtext { ("11/10/2024") };
                            };
                            div.home-blog-post-text {
                                // todo
                                p { ("Blog posts coming soon!") }
                            }
                        };

                    };
                    div.home-content-container {
                        div.home-content-section {
                            div.home-content-section-header {
                                span.h2 { ("about me") }
                            };
                            div.home-content-section-content {
                                p { (r##"im an autistic transfem
                                        developer mostly coding in rust,
                                        kotlin, and typescript."##) };
                                p { (r##"im currently in my third year of
                                        uni, dual-majoring in compsci and
                                        gender studies.
                                        "##) };
                                p { (r##"i live in the midwest us, with
                                        my clocks set to eastern time."##) }
                                p { (r##"i love all of my %n partners <3"##) }
                            };

                        };
                        div.home-content-section {
                            div.home-content-section-header {
                                span.h2 { ("listening to...") }
                            };
                            div.home-content-section-content {
                                // todo
                            };
                        };
                    }
                }
            }
        },
        String::from("projects") => collection! {
            html! {
                // sidebar

            },
            html! {
                // content
            }
        },
        String::from("blog") => collection! {
            html! {
                // sidebar

            },
            html! {
                // content
            }
        },
    };

    frames::homepage(tabs, cache_nonce)
}
