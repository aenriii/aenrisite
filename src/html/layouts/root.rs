use maud::{html, Markup, DOCTYPE};

pub fn root(
    dom: Markup,
    title: Option<String>,
    additional_stylesheets: Option<Vec<String>>,
) -> Markup {
    let mut styles = additional_stylesheets.unwrap_or(vec![]);
    styles.extend(vec!["/themes.css".to_string(), "/base.css".to_string()]);
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title.unwrap_or(String::from("aenri.loveh.art"))) };
                @for stylesheet in styles {
                    link rel="stylesheet" href=(stylesheet);
                }
            }
            body { (dom) }
        }
    }
}
