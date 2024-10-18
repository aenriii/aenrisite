use cached::proc_macro::cached;
use maud::{html, Markup, PreEscaped};
use std::collections::HashMap;
use super::base_frame;

#[inline(always)]
#[cached(
    key = "u128",
    convert = r##"{ cache_nonce }"##,
    time = 60
)]
pub fn homepage( tabs: HashMap<String, (Markup, Markup)>, cache_nonce: u128 ) -> Markup {

    let (
        home_sidebar,
        home_content
    ) = tabs.get("home").expect("'home' tab should be defined!");
    let (
        projects_sidebar,
        projects_content
    ) = tabs.get("projects").expect("'projects' tab should be defined!");
    let (
        blog_sidebar,
        blog_content
    ) = tabs.get("blog").expect("'blog' tab should be defined!");

    base_frame(html! {
        div id="content-host" {
            div.sidebar {
                div.tabs {
                    div style="display: none;" {
                        input type="radio" id="tab-home" name="tabs" value="home" checked;
                        input type="radio" id="tab-projects" name="tabs" value="projects";
                        input type="radio" id="tab-blog" name="tabs" value="blog";
                    };
                    label.tab for="tab-home" { "home" };
                    span.tab { "/" };
                    label.tab for="tab-projects" { "projects" };
                    span.tab { "/" };
                    label.tab for="tab-blog" { "blog" };
                };
                div.sidebar-content {
                    div.togglable for="home" { (home_sidebar) };
                    div.togglable for="projects" { (projects_sidebar) };
                    div.togglable for="blog" { (blog_sidebar) };
                }
            };
            div.content {
                div.togglable for="home" { (home_content) };
                div.togglable for="projects" { (projects_content) };
                div.togglable for="blog" { (blog_content) };
            }
        }
    })
}
