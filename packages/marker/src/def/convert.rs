use super::common::TextStyle;
use crate::def as fin;
use crate::def::intermed as int;

macro_rules! reslice {
    ($vec:ident, $type:ty) => {
        unsafe {
            let parts = $vec.into_raw_parts();
            ::std::slice::from_raw_parts(parts.0 as *const $type, parts.1)
        }

    };
}

macro_rules! restring {
    ($str:ident) => {
        unsafe {
            let parts = $str.clone().into_raw_parts();
            ::std::str::from_raw_parts(parts.0, parts.1)
        }
    };
}

impl <'a> From<int::Page> for fin::Page<'a> {
    fn from(value: int::Page) -> Self {
        fin::Page {
            metadata: value.metadata.into(),
            children: {
                let v = value.children
                    .iter()
                    .map(|x| (*x).clone().into())
                    .collect::<Vec<fin::Node<'a>>>();
                reslice!(v, fin::Node<'a>)
            }
        }
    }
}

impl <'a> From<fin::Page<'a>> for int::Page {
    fn from(value: fin::Page<'a>) -> Self {
        int::Page {
            metadata: value.metadata.into(),
            children: value.children
                .iter()
                .map(|x| x.clone().into())
                .collect()
        }
    }
}

impl <'a> From<int::PageMetadata> for fin::PageMetadata<'a> {
    fn from(value: int::PageMetadata) -> Self {
        let title = value.title;
        let desc = value.description;
        let flags = {
            let v = value.flags
                .iter()
                .map(|x| restring!(x))
                .collect::<Vec<&'a str>>();
            reslice!(v, &'a str)
        };
        let options = {
            let v = value.options
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect::<Vec<_>>();
            reslice!(v, (&'a str, &'a str))
        };
        fin::PageMetadata {
            title: restring!(title),
            description: restring!(desc),
            flags,
            options
        }
    }
}

impl <'a> From<fin::PageMetadata<'a>> for int::PageMetadata {
    fn from(value: fin::PageMetadata<'a>) -> Self {
        let flags = {
            value.flags
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
        };
        let options = {
            value.options
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<Vec<_>>()
        };

        int::PageMetadata {
            title: value.title.to_string(),
            description: value.description.to_string(),
            flags,
            options
        }
    }
}


impl <'a> From<int::Node> for fin::Node<'a> {
    fn from(value: int::Node) -> Self {
        use int::Node as iNode;
        use fin::Node as fNode;

        match value {
            iNode::Container {
                flags, options, children
             } => {
                let flags = {
                    let v = flags
                        .iter()
                        .map(|x| restring!(x))
                        .collect::<Vec<&'a str>>();
                    reslice!(v, &'a str)
                };
                let options = {
                    let v = options
                        .iter()
                        .map(|(k, v)| (k.as_str(), v.as_str()))
                        .collect::<Vec<_>>();
                    reslice!(v, (&'a str, &'a str))
                };
                let children = {
                    let v = children
                        .iter()
                        .map(|x| (*x).clone().into())
                        .collect::<Vec<fin::Node<'a>>>();
                    reslice!(v, fin::Node<'a>)
                };
                fNode::Container { flags: flags, options: options, children: children }
             },
            iNode::FlexContainer {
                flags, options, inline, gap, children
            } => {
                let flags = {
                    let v = flags
                        .iter()
                        .map(|x| restring!(x))
                        .collect::<Vec<&'a str>>();
                    reslice!(v, &'a str)
                };
                let options = {
                    let v = options
                        .iter()
                        .map(|(k, v)| (k.as_str(), v.as_str()))
                        .collect::<Vec<_>>();
                    reslice!(v, (&'a str, &'a str))
                };
                let children = {
                    let v = children
                        .iter()
                        .map(|x| (*x).clone().into())
                        .collect::<Vec<fin::Node<'a>>>();
                    reslice!(v, fin::Node<'a>)
                };
                fNode::FlexContainer { flags, options, inline, gap, children }
            },
            iNode::GridContainer {
                flags, options, wrap, gap, children
            } => {
                let flags = {
                    let v = flags
                        .iter()
                        .map(|x| restring!(x))
                        .collect::<Vec<&'a str>>();
                    reslice!(v, &'a str)
                };
                let options = {
                    let v = options
                        .iter()
                        .map(|(k, v)| (k.as_str(), v.as_str()))
                        .collect::<Vec<_>>();
                    reslice!(v, (&'a str, &'a str))
                };
                let children = {
                    let v = children
                        .iter()
                        .map(|x| (*x).clone().into())
                        .collect::<Vec<fin::Node<'a>>>();
                    reslice!(v, fin::Node<'a>)
                };
                fNode::GridContainer { flags, options, wrap, gap, children }
            },
            iNode::Header {
                level, text, subheader_text
            } => {
                fNode::Header { level, text: restring!(text), subheader_text: subheader_text.map(|x| restring!(x)) }
            },
            iNode::Break {
                kind } => fNode::Break { kind },
            iNode::CodeBlock {
                flags, options, lang, code
            } => {
                let flags = {
                    let v = flags
                        .iter()
                        .map(|x| restring!(x))
                        .collect::<Vec<&'a str>>();
                    reslice!(v, &'a str)
                };
                let options = {
                    let v = options
                        .iter()
                        .map(|(k, v)| (k.as_str(), v.as_str()))
                        .collect::<Vec<_>>();
                    reslice!(v, (&'a str, &'a str))
                };

                fNode::CodeBlock { flags, options, lang: lang.map(|x| restring!(x)), code: restring!(code) }
            },
            iNode::Paragraph {
                flags, options, children
             } => {
                let flags = {
                    let v = flags
                        .iter()
                        .map(|x| restring!(x))
                        .collect::<Vec<&'a str>>();
                    reslice!(v, &'a str)
                };
                let options = {
                    let v = options
                        .iter()
                        .map(|(k, v)| (k.as_str(), v.as_str()))
                        .collect::<Vec<_>>();
                    reslice!(v, (&'a str, &'a str))
                };
                let children = {
                    let v = children
                        .iter()
                        .map(|x| (*x).clone().into())
                        .collect::<Vec<fin::Node<'a>>>();
                    reslice!(v, fin::Node<'a>)
                };
                fNode::Paragraph { flags, options, children }
             },
            iNode::Text {
                flags, options, styles, content
            } => {
                let flags = {
                    let v = flags
                        .iter()
                        .map(|x| restring!(x))
                        .collect::<Vec<&'a str>>();
                    reslice!(v, &'a str)
                };
                let options = {
                    let v = options
                        .iter()
                        .map(|(k, v)| (k.as_str(), v.as_str()))
                        .collect::<Vec<_>>();
                    reslice!(v, (&'a str, &'a str))
                };
                let styles = reslice!(styles, TextStyle);
                let content = restring!(content);

                fNode::Text { flags, options, styles, content }
            },
            iNode::Link {
                flags, options, children, href
            } => {
                let flags = {
                    let v = flags
                        .iter()
                        .map(|x| restring!(x))
                        .collect::<Vec<&'a str>>();
                    reslice!(v, &'a str)
                };
                let options = {
                    let v = options
                        .iter()
                        .map(|(k, v)| (k.as_str(), v.as_str()))
                        .collect::<Vec<_>>();
                    reslice!(v, (&'a str, &'a str))
                };
                let children = {
                    let v = children
                        .iter()
                        .map(|x| (*x).clone().into())
                        .collect::<Vec<fin::Node<'a>>>();
                    reslice!(v, fin::Node<'a>)
                };
                fNode::Link { flags, options, children, href: restring!(href) }
            },
            iNode::Mixin {
                kind, params
            } => {
                let params = {
                    let v = params
                        .iter()
                        .map(|x| restring!(x))
                        .collect::<Vec<&'a str>>();
                    reslice!(v, &'a str)
                };
                fNode::Mixin { kind: restring!(kind), params }
            },
        }
    }
}

impl <'a> From<fin::Node<'a>> for int::Node {
    fn from(value: fin::Node<'a>) -> Self {
        use int::Node as iNode;
        use fin::Node as fNode;

        match value {
            fNode::Container {
                flags, options, children
            } => {
                let flags = {
                    flags
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                };
                let options = {
                    options
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect::<Vec<_>>()
                };
                let children = {
                    children
                        .iter()
                        .map(|x| (*x).clone().into())
                        .collect::<Vec<_>>()
                };

                iNode::Container { flags, options, children }
            },
            fNode::FlexContainer {
                flags, options, inline, gap, children
            } => {
                let flags = {
                    flags
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                };
                let options = {
                    options
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect::<Vec<_>>()
                };
                let children = {
                    children
                        .iter()
                        .map(|x| (*x).clone().into())
                        .collect::<Vec<_>>()
                };
                iNode::FlexContainer { flags, options, inline, gap, children }
            },
            fNode::GridContainer {
                flags, options, wrap, gap, children
            } => {
                let flags = {
                    flags
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                };
                let options = {
                    options
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect::<Vec<_>>()
                };
                let children = {
                    children
                        .iter()
                        .map(|x| (*x).clone().into())
                        .collect::<Vec<_>>()
                };
                iNode::GridContainer { flags, options, wrap, gap, children }
            },
            fNode::Header {
                level, text, subheader_text
            } => {
                iNode::Header { level, text: text.to_string(), subheader_text: subheader_text.map(|x| x.to_string()) }
            },
            fNode::Break {
                kind
            } => {
                iNode::Break { kind }
            },
            fNode::CodeBlock {
                flags, options, lang, code
            } => {
                let flags = {
                    flags
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                };
                let options = {
                    options
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect::<Vec<_>>()
                };
                let lang = lang.map(|x| x.to_string());
                iNode::CodeBlock { flags, options, lang, code: code.to_string() }
            },
            fNode::Paragraph {
                flags, options, children
            } => {
                let flags = {
                    flags
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                };
                let options = {
                    options
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect::<Vec<_>>()
                };
                let children = {
                    children
                        .iter()
                        .map(|x| (*x).clone().into())
                        .collect::<Vec<_>>()
                };
                iNode::Paragraph { flags, options, children }
            },
            fNode::Text {
                flags, options, styles, content
            } => {
                let flags = {
                    flags
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                };
                let options = {
                    options
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect::<Vec<_>>()
                };
                let styles = styles
                    .iter()
                    .map(|x| x.clone())
                    .collect::<Vec<_>>();
                iNode::Text { flags, options, styles, content: content.to_string() }
            },
            fNode::Link {
                flags, options, children, href
            } => {
                let flags = {
                    flags
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                };
                let options = {
                    options
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect::<Vec<_>>()
                };
                let children = {
                    children
                        .iter()
                        .map(|x| (*x).clone().into())
                        .collect::<Vec<_>>()
                };
                iNode::Link { flags, options, children, href: href.to_string() }
            },
            fNode::Mixin {
                kind, params
            } => {
                let params = {
                    params
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                };
                iNode::Mixin { kind: kind.to_string(), params }
            },
        }
    }
}
