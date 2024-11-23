#[derive(Debug, PartialEq)]
pub enum LengthKind {
    Px,
    Em,
    Rem
}
pub type Length = (u32, LengthKind);

#[derive(Debug, PartialEq)]
pub enum BreakKind {
    Inline,
    Visible,
    Invisible
}

#[derive(Debug, PartialEq)]
pub enum TextStyle {
    None,
    Italic,
    Bold,
    Underlined,
    Strikethrough,
    Wavey, // i love dtf
    Superscript
}
