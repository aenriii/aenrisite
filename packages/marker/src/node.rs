#[derive(Debug, Clone, Copy)]
pub enum Node<'a> {
    Block {
        header: HeaderOptions<'a>,
        metadata: &'a[(&'a str, &'a str)],
        nodes: &'a[Node<'a>]
    },
    Divider {
        visible: bool,
    },
    Inline {
        nodes: &'a[Node<'a>]
    },
    Link {
        nodes: &'a[Node<'a>],
        href: &'a str
    },
    Text {
        options: Option<TextOptions<'a>>,
        text: &'a str
    }

}
#[derive(Debug, Clone, Copy)]
pub struct HeaderOptions<'a> {
    pub level: u8,
    pub nodes: &'a[Node<'a>],
    pub id: Option<&'a str>
}
#[derive(Debug, Clone, Copy)]
pub struct TextOptions<'a> {
    pub style: &'a str,
    pub size: &'a str
}
