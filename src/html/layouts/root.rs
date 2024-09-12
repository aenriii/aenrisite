use html_node::{html, text, Node};

#[inline(always)]
pub fn root(dom: Node, title: Option<String>, additional_stylesheets: Option<Vec<String>>) -> Node {
    let mut stylesheets = vec!["/themes.css".to_string(), "/base.css".to_string()];
    if let Some(styles) = additional_stylesheets {
        stylesheets.extend(styles);
    }
    html!(
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>{text!("{}", title.unwrap_or("aenri.loveh.art".to_string()))}</title>
                {
                    stylesheets.into_iter().map(|item| html! {
                        <link rel="stylesheet" href=text!("{}", item) />
                    })
                }
            </head>
            <body>
                {dom}
            </body>
        </html>
    )
}
