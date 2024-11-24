
/// intermediary versions of page objects, these are easier
/// to work with when constructing a page from source but
/// are not able to be "const" variables
pub mod intermed;

/// structs / enums that do not need separate implementations
/// for intermediary and final implementations
pub mod common;

/// module defining implementations for converting between
/// intermediary and final versions of the struct, this also
/// makes it easier to dispose of &'_ [T] as they are converted
/// into Vec<T>
pub mod convert;

/// module implementing ToTokens for all finalized types, allowing
/// marker_macro to simply invoke the trait's functions for its
/// functionality
pub mod tokenize;

/// module defining generalized errors
pub mod errors;


use common::{
    BreakKind, Length, TextStyle
};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Page<'a> {
    pub metadata: PageMetadata<'a>,
    pub children: &'a[Node<'a>]
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PageMetadata<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub flags: &'a [&'a str],
    pub options: &'a [(&'a str, &'a str)]
}
#[derive(Debug, PartialEq, Clone)]
pub enum Node<'a> {
    Container {
        flags: &'a [&'a str],
        options: &'a [(&'a str, &'a str)],
        children: &'a[Node<'a>]
    },
    FlexContainer {
        flags: &'a [&'a str],
        options: &'a [(&'a str, &'a str)],
        inline: bool,
        gap: Length,
        children: &'a[Node<'a>]
    },
    GridContainer {
        flags: &'a [&'a str],
        options: &'a [(&'a str, &'a str)],
        wrap: bool,
        gap: Length,
        children: &'a[Node<'a>]
    },
    Header {
        level: u8,
        text: &'a str,
        subheader_text: Option<&'a str>
    },
    Break {
        kind: BreakKind
    },
    CodeBlock {
        flags: &'a [&'a str],
        options: &'a [(&'a str, &'a str)],
        lang: Option<&'a str>,
        code: &'a str
    },
    Paragraph {
        flags: &'a [&'a str],
        options: &'a [(&'a str, &'a str)],
        children: &'a[Node<'a>]
    },
    Text {
        flags: &'a [&'a str],
        options: &'a [(&'a str, &'a str)],
        styles: &'a [TextStyle],
        content: &'a str
    },
    Link {
        flags: &'a [&'a str],
        options: &'a [(&'a str, &'a str)],
        children: &'a[Node<'a>],
        href: &'a str
    },
    Mixin {
        kind: &'a str,
        params: &'a [&'a str]
    }
}
