use maud::PreEscaped;


pub struct Frame {
    classes: Vec<String>,
    components: Vec<Box<dyn Component>>
}

