use crate::node::Node;



pub struct Page<'a> {
    pub options: PageOptions<'a>,
    pub nodes: &'a [Node<'a>]
}

pub struct PageOptions<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub metadata: &'a[(&'a str, &'a str)]
}
