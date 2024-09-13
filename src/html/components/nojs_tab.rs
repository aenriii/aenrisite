use cached::proc_macro::cached;
use maud::{Markup, PreEscaped};
#[inline(always)]
#[cached(key = "u128", convert = " { nonce } ", time = 15)]
pub fn nojs_tabs(tabs: Vec<(String, Markup)>, nonce: u128) -> Markup {
    let nonce_uuid = uuid::Uuid::from_u128(nonce);
    let stylesheet = get_page_data(tabs.clone().into_iter().map(|x| x.0).collect(), nonce);
    maud::html! {
        style {
            (PreEscaped(stylesheet))
        };
        div class=(format!("tabs-{}", nonce_uuid)) {
            div class="row" {
                div style="display: none;" {
                    @for tab_enum in tabs.clone().into_iter().enumerate() {
                        @let tab = tab_enum.1;
                        @if (tab_enum.0 == 0) {
                            input type="radio" id=(format!("sel-{}-{}", tab.0, nonce_uuid)) name=(nonce_uuid) value=(tab.0) checked;
                        }
                        @else {
                            input type="radio" id=(format!("sel-{}-{}", tab.0, nonce_uuid)) name=(nonce_uuid) value=(tab.0);
                        }
                    }
                };
                @for tab_enum in tabs.clone().into_iter().enumerate() {
                    label for=(format!("sel-{}-{}", tab_enum.1.0, nonce_uuid)) class="section-title" {
                        (tab_enum.1.0);
                    };

                    @if &(tab_enum.0 + 1) < &tabs.len() {
                        span class="section-title" { "/" };
                    };
                }
            };
            div class="container" style="display: contents;" {
                @for tab in &tabs {
                    div id=(format!("tab-{}-{}", tab.0, nonce_uuid)) {
                        (tab.1)
                    }
                }
            }
        }
    }
}

#[cached::proc_macro::cached(key = "u128", convert = r#" { nonce } "#, sync_writes = "true")]
#[inline(always)]
fn get_page_data(tab_names: Vec<String>, nonce: u128) -> String {
    let tab_style_rand = uuid::Uuid::from_u128(nonce);
    let mut stylesheet = String::with_capacity(512);

    stylesheet.push_str(&format!(
        r#"
       .tabs-{} > .row > label {{  color: var(--text); }}
       "#,
        tab_style_rand
    ));

    for tab in tab_names.clone() {
        stylesheet.push_str(&format!(
            r#"
            .tabs-{}:has(#sel-{}-{}:checked) > .container > :not(#tab-{}-{}) {{ display: none; }}
            .tabs-{}:has(#sel-{}-{}:checked) > .row > label:not([for=sel-{}-{}]) {{ color: var(--subtext-variant); }}
            .tabs-{}:has(#sel-{}-{}:checked) > .row > label:not([for=sel-{}-{}]):hover {{ color: var(--text); }}
            "#,
            tab_style_rand, tab, tab_style_rand, tab, tab_style_rand,
            tab_style_rand, tab, tab_style_rand, tab, tab_style_rand,
            tab_style_rand, tab, tab_style_rand, tab, tab_style_rand
        ));
    }

    stylesheet
}
