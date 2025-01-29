use std::collections::HashMap;

use crate::argument::Argument;

use super::Component;

pub struct Image {
    attrs: HashMap<String, String>
}

impl Component for Image {
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
        }
    }

    fn insert(&mut self, _children: Vec<Box<dyn Component>>) {
        unimplemented!();
    }

    fn render(&self) -> String {
        let mut rendered = String::from("<img");
        for (attr, val) in &self.attrs {
            rendered.push_str(&format!(" {}", attr));
            if !val.is_empty() {
                rendered.push_str(&format!("=\"{}\"", val));
            }
        }
        rendered.push_str("/>");
        rendered
    }
}
