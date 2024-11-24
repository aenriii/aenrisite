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
                    img.avatar-image src="/assets/avatar.webp" alt="Avatar Image";
                    div.home-sidebar-content-container {
                        span.h1 { ("hi, im aenri") };
                        div.home-sidebar-description {
                            span { ("pup/it / θ∆" ) };
                            span { ("bodily 19, ageless" ) };
                            span { ("pro-para anti-harm" ) };
                            span { ("poly puppy developer" ) };
                        };
                        span.h2 { ("links") };
                        div.links {
                            a href="https://discord.com/users/98133204636028928" target="_blank" {
                                img.link-img src="/assets/discord.svg" alt="@aenriii on discord";
                            }
                            a href="https://signal.me/#eu/s2xWaaUelilwvDaVg6xjIC73BHzWC_CQuy47fhTEn-eTLI94cMpK3Ls7E_hkOPUL" target="_blank" {
                                img.link-img src="/assets/signal.svg" alt="@aenri.11 on signal";
                            }
                            a href="https://bsky.app/profile/aenri.loveh.art" target="_blank" {
                                img.link-img src="/assets/bsky.svg" alt="aenri.loveh.art on bsky";
                            }
                            a href="https://x.com/aenri_" target="_blank" {
                                img.link-img src="/assets/twitter.svg" alt="@aenri_ on twitter";
                            }
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
                                span.title.h2 { ("writing a dynamic site without clientside code") };
                                span.sep { ("-") };
                                span.date.subtext { ("2024/11/23") };
                            };
                            div.home-blog-post-text {
                                // todo
                                p { (" a nearly tall tale of svelte, rust, and probably like, 200+ hours") }
                                small { ("Blog posts coming soon!") }
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
                                p { (r##"im currently out of school, working
                                        various jobs to pull me up to where
                                        i feel comfortable continuing my
                                        adventures.
                                        "##) };
                                p { (r##"i live in the midwest us, with
                                        my clocks set to eastern time."##) }
                                p { (r##"i love all of my 6 partners <3"##) }
                            };

                        };
                        div.home-content-section {
                            div.home-content-section-header {
                                span.h2 { ("listening to...") }
                            };
                            div.home-content-section-content {
                                p { ("This section is still under construction, come back later!") }
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
                div.under-construction {
                    span.h2 { ("Under construction") }
                    em { ("come back later!") }
                }
            }
        },
        String::from("blog") => collection! {
            html! {
                // sidebar

            },
            html! {
                div.under-construction {
                    span.h2 { ("Under construction") }
                    em { ("come back later!") }
                }
            }
        },
    };

    frames::homepage(tabs, cache_nonce)
}
