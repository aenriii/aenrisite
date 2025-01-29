use std::collections::HashMap;
use crate::argument::Argument;
use super::{Component, Fragment};
use html_escape::encode_safe;


pub struct Frame {
    inner: Fragment,
    attrs: HashMap<String, String>
}

impl Component for Frame {
    fn create(options: HashMap<String, Argument>) -> Self where Self : Sized {
        // filter and downcast

        let mut attrs = HashMap::new();

        for mut opt in options {
            let name = opt.0;
            if let Some(string) = opt.1.try_get::<String>() { attrs.insert(name, string); }
            else if let Some(num) = opt.1.try_get::<usize>() { attrs.insert(name, num.to_string()); }
            else if let Some(true) = opt.1.try_get::<bool>() { attrs.insert(name, String::new()); }
        }
        Self {
            attrs,
            inner: Fragment::create(HashMap::new())
        }
    }

    fn insert(&mut self, children: Vec<Box<dyn Component>>) {
        self.inner.insert(children);
    }

    fn render(&self) -> String {
        let mut rendered = String::from("<div");
        for (attr, val) in &self.attrs {
            rendered.push_str(&format!(" {}", attr));
            if !val.is_empty() {
                rendered.push_str(&format!("=\"{}\"", val));
            }
        }
        rendered.push('>');
        rendered.push_str(&self.inner.render());
        rendered.push_str("</div>");
        rendered
    }
}
