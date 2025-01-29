use std::collections::HashMap;
use crate::argument::Argument;

use super::Component;

/// Fragment is an invisible struct, much like the
/// <> element in JSX. It has its use-cases, I
/// promise.
pub struct Fragment {
    wrapper: bool,
    contents: Vec<Box<dyn Component>>
}

impl Component for Fragment {
    fn create(mut options: HashMap<String, Argument>) -> Self where Self : Sized {
        let wrapper = !options.get_mut("wrapper").is_none_or(|x| {
            if let Some(true) = x.try_get::<bool>() { return true } false
        });
        Self {
            wrapper,
            contents: vec![]
        }
    }
    fn insert(&mut self, children: Vec<Box<dyn Component>>) {
        for child in children {
            self.contents.push(child);
        }
    }
    fn render(&self) -> String {
        let mut string = String::new();
        if self.wrapper { string.push_str("<div>"); }
        for child in &self.contents {
            string.push_str(&child.render());
        }
        if self.wrapper { string.push_str("</div>"); }
        string
    }
}
