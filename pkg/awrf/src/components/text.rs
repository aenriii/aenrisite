use std::collections::HashMap;
use super::Component;
use html_escape::encode_safe;
use maud::html;
pub struct Text {
    contents: String,
    tag_name: String,
    attrs: HashMap<String, String>
}
impl Component for Text {
    fn create(options: HashMap<String, crate::argument::Argument>) -> Self where Self : Sized {
        let mut attrs = HashMap::new();
        let mut tag_name = String::from("span");
        for mut opt in options {
            let name = opt.0;

            if &name == "tag" {
                if let Some(string) = opt.1.try_get::<String>() {
                    tag_name = string;
                }
            }

            if let Some(string) = opt.1.try_get::<String>() { attrs.insert(name, string); }
            else if let Some(num) = opt.1.try_get::<usize>() { attrs.insert(name, num.to_string()); }
            else if let Some(true) = opt.1.try_get::<bool>() { attrs.insert(name, String::new()); }
        }

        Self {
            attrs,
            tag_name,
            contents: String::new()
        }

    }

    fn insert(&mut self, children: Vec<Box<dyn Component>>) {
        for child in children {
            self.contents.push_str(&child.render());
        }
    }

    fn render(&self) -> String {
        let mut rendered = format!("<{}", self.tag_name);
        for (attr, val) in &self.attrs {
            rendered.push_str(&format!(" {}", attr));
            if !val.is_empty() {
                rendered.push_str(&format!("=\"{}\"", val));
            }
        }
        rendered.push('>');
        rendered.push_str(&self.contents);
        rendered.push_str(&format!("</{}>", self.tag_name));
        rendered
    }
}
