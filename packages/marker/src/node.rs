
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
        text: &'a str,
        href: &'a str
    },
    Text {
        options: Option<TextOptions<'a>>,
        text: &'a str
    }

}

pub struct HeaderOptions<'a> {
    pub level: u8,
    pub text: Option<&'a str>,
    pub id: Option<&'a str>
}

pub struct TextOptions<'a> {
    pub style: &'a str,
    pub size: &'a str
}
