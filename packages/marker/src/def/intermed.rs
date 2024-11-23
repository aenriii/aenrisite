use super::common::{
    BreakKind, Length, TextStyle
};

#[derive(Debug, Default, PartialEq)]
pub struct Page {
    pub metadata: PageMetadata,
    pub children: Vec<Node>
}

#[derive(Debug, Default, PartialEq)]
pub struct PageMetadata {
    pub title: String,
    pub description: String,
    pub flags: Vec<String>,
    pub options: Vec<(String, String)>
}
#[derive(Debug, PartialEq)]
pub enum Node {
    Container {
        flags: Vec<String>,
        options: Vec<(String, String)>,
        children: Vec<Node>
    },
    FlexContainer {
        flags: Vec<String>,
        options: Vec<(String, String)>,
        inline: bool,
        gap: Length,
        children: Vec<Node>
    },
    GridContainer {
        flags: Vec<String>,
        options: Vec<(String, String)>,
        wrap: bool,
        gap: Length,
        children: Vec<Node>
    },
    Header {
        level: u8,
        text: String,
        subheader_text: Option<String>
    },
    Break {
        kind: BreakKind
    },
    CodeBlock {
        flags: Vec<String>,
        options: Vec<(String, String)>,
        lang: Option<String>,
        code: String
    },
    Paragraph {
        flags: Vec<String>,
        options: Vec<(String, String)>,
        children: Vec<Node>
    },
    Text {
        flags: Vec<String>,
        options: Vec<(String, String)>,
        styles: Vec<TextStyle>,
        content: String
    },
    Link {
        flags: Vec<String>,
        options: Vec<(String, String)>,
        children: Vec<Node>
    },
    Mixin {
        kind: String,
        params: Vec<String>
    }
}
