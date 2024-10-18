use maud::{html, Markup, PreEscaped, DOCTYPE};

#[inline(always)]
pub fn base_frame(slot: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (PreEscaped("aenri.loveh.art")) };
                link rel="stylesheet" href="/assets/base.css";
                link rel="stylesheet" href="/assets/home.css";
            body { (slot) }
            }
        }
    }
}
