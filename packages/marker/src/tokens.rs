use quote::{quote, ToTokens, TokenStreamExt};

use crate::{HeaderOptions, Node, Page, PageOptions, TextOptions};

impl ToTokens for Page<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let options = &self.options;
        let nodes = self.nodes;

        tokens.append_all(quote! { Page {
            options: #options,
            nodes: &[#(#nodes),*]
        } });
    }
}

impl ToTokens for Node<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use Node::*;
        match self {
            Block {
                header,
                metadata,
                nodes,
            } => {
                let metadata_l = metadata.iter().map(|x| x.0);
                let metadata_r = metadata.iter().map(|x| x.1);
                tokens.append_all(quote! { Node::Block {
                    header: #header,
                    metadata: &[#((#metadata_l, #metadata_r)),*],
                    nodes: &[#(#nodes),*]
                } });
            }
            Divider { visible } => {
                tokens.append_all(quote! { Node::Divider {
                    visible: #visible
                } });
            }
            Inline { nodes } => {
                tokens.append_all(quote! { Node::Inline {
                    nodes: &[#(#nodes),*]
                } });
            }
            Link { nodes, href } => {
                tokens.append_all(quote! { Node::Link {
                    nodes: &[#(#nodes),*],
                    href: #href
                } });
            }
            Text { options, text } => {
                let options = match options {
                    Some(options) => quote! { Some(#options) },
                    None => quote! { None },
                };
                tokens.append_all(quote! { Node::Text {
                    options: #options,
                    text: #text
                }});
            }
        }
    }
}

impl ToTokens for TextOptions<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let style = self.style;
        let size = self.size;
        tokens.append_all(quote! {TextOptions {
            style: #style,
            size: #size
        }});
    }
}

impl ToTokens for HeaderOptions<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let level = self.level;
        let nodes = &self.nodes;
        let id = match self.id {
            Some(txt) => quote! { Some(#txt) },
            None => quote! { None },
        };
        tokens.append_all(quote! {HeaderOptions {
            level: #level,
            nodes: &[#(#nodes),*],
            id: #id
        }});
    }
}

impl ToTokens for PageOptions<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let title = self.title;
        let desc = self.description;
        let metadata_l = self.metadata.iter().map(|x| x.0);
        let metadata_r = self.metadata.iter().map(|x| x.1);
        tokens.append_all(quote! {PageOptions {
            title: #title,
            description: #desc,
            metadata: &[#((#metadata_l, #metadata_r)),*]
        }});
    }
}
#[allow(non_snake_case)]
mod tests {
    use quote::{quote, ToTokens};

    use crate::*;
    #[test]
    fn totokens_impl_serializes_PageOptions() {
        let po = PageOptions {
            title: "the page you get sent when you ask about it",
            description: "an explaination and defense of the radqueer position from my perspective",
            metadata: &[("special", "true"), ("slug", "radqueer")],
        };
        let po_tokens = po.to_token_stream();
        let quote_tokens = quote! {PageOptions {
            title: "the page you get sent when you ask about it",
            description: "an explaination and defense of the radqueer position from my perspective",
            metadata: &[("special", "true"), ("slug", "radqueer")]
        }};
        assert_eq!(po_tokens.to_string(), quote_tokens.to_string())
    }

    #[test]
    fn totokens_impl_serializes_HeaderOptions() {
        let ho = HeaderOptions {
            level: 0,
            nodes: &[Node::Text { options: None, text: "header" }],
            id: None,
        };
        let ho_tokens = ho.to_token_stream();
        let quote_tokens = quote! {HeaderOptions {
            level: 0u8,
            nodes: &[Node::Text { options: None, text: "header" }],
            id: None
        }};
        assert_eq!(ho_tokens.to_string(), quote_tokens.to_string())
    }

    #[test]
    fn totokens_impl_serializes_TextOptions() {
        let to = TextOptions {
            style: "normal",
            size: "subtext",
        };
        let to_tokens = to.to_token_stream();
        let quote_tokens = quote! {TextOptions {
            style: "normal",
            size: "subtext"
        }};

        assert_eq!(to_tokens.to_string(), quote_tokens.to_string())
    }

    #[test]
    fn totokens_impl_serializes_Node() {
        let node = Node::Block {
            header: HeaderOptions {
                level: 0u8,
                nodes: &[],
                id: None
            },
            metadata: &[("alignment", "center")],
            nodes: &[
                Node::Text {
                    options: None,
                    text: "i would like us all to take a deep breath."
                },
                Node::Text {
                    options: Some(TextOptions {
                        style: "normal",
                        size: "subtext"
                    }),
                    text: "bear with me, it'll all make sense soon"
                },
                Node::Inline {
                    nodes: &[
                        Node::Text {
                            options: None,
                            text: "im so cool"
                        },
                        Node::Link {
                            nodes: &[
                                Node::Text {
                                    options: None,
                                    text: "im so cool"
                                }],
                            href: "example.com"
                        }
                    ]
                },
                Node::Divider { visible: true }
            ]
        };
        let node_tokens = node.to_token_stream();
        let quote_tokens = quote! {Node::Block {
            header: HeaderOptions {
                level: 0u8,
                nodes: &[],
                id: None
            },
            metadata: &[("alignment", "center")],
            nodes: &[
                Node::Text {
                    options: None,
                    text: "i would like us all to take a deep breath."
                },
                Node::Text {
                    options: Some(TextOptions {
                        style: "normal",
                        size: "subtext"
                    }),
                    text: "bear with me, it'll all make sense soon"
                },
                Node::Inline {
                    nodes: &[
                        Node::Text {
                            options: None,
                            text: "im so cool"
                        },
                        Node::Link {
                            nodes: &[
                                Node::Text {
                                    options: None,
                                    text: "im so cool"
                                }],
                            href: "example.com"
                        }
                    ]
                },
                Node::Divider { visible: true }
            ]
        }};

        assert_eq!(node_tokens.to_string(), quote_tokens.to_string())
    }

    #[test]
    fn totokens_impl_serializes_Page() {
        let page = Page {
            options: PageOptions {
                title: "the page you get sent when you ask about it",
                description:
                    "an explaination and defense of the radqueer position from my perspective",
                metadata: &[("special", "true"), ("slug", "radqueer")]
            },
            nodes: &[Node::Block {
                header: HeaderOptions {
                    level: 1u8,
                    nodes: &[Node::Text { options: None, text: "what."}],
                    id: None
                },
                metadata: &[],
                nodes: &[
                    Node::Block {
                        header: HeaderOptions {
                            level: 0u8,
                            nodes: &[],
                            id: None
                        },
                        metadata: &[("alignment", "center")],
                        nodes: &[
                            Node::Text {
                                options: None,
                                text: "i would like us all to take a deep breath."
                            },
                            Node::Text {
                                options: Some(TextOptions {
                                    style: "normal",
                                    size: "subtext"
                                }),
                                text: "bear with me, it'll all make sense soon"
                            }
                        ]
                    },
                    Node::Divider { visible: false },
                    Node::Block {
                        header: HeaderOptions {
                            level: 0u8,
                            nodes: &[],
                            id: None
                        },
                        metadata: &[],
                        nodes: &[Node::Inline {
                            nodes: &[
                                Node::Text {
                                    options: None,
                                    text: "yes, i have sources; "
                                },
                                Node::Text {
                                    options: Some(TextOptions {
                                        style: "italic",
                                        size: "text"
                                    }),
                                    text: "academic"
                                },
                                Node::Text {
                                    options: None,
                                    text: " sources."
                                }
                            ]
                        }]
                    }
                ]
            }]
        };
        let page_tokens = page.into_token_stream();
        let quote_tokens = quote! {Page {
            options: PageOptions {
                title: "the page you get sent when you ask about it",
                description: "an explaination and defense of the radqueer position from my perspective",
                metadata: &[("special", "true"), ("slug", "radqueer")]
            },
            nodes: &[Node::Block {
                header: HeaderOptions {
                    level: 1u8,
                    nodes: &[Node::Text { options: None, text: "what."}],
                    id: None
                },
                metadata: &[],
                nodes: &[
                    Node::Block {
                        header: HeaderOptions {
                            level: 0u8,
                            nodes: &[],
                            id: None
                        },
                        metadata: &[("alignment", "center")],
                        nodes: &[
                            Node::Text {
                                options: None,
                                text: "i would like us all to take a deep breath."
                            },
                            Node::Text {
                                options: Some(TextOptions {
                                    style: "normal",
                                    size: "subtext"
                                }),
                                text: "bear with me, it'll all make sense soon"
                            }
                        ]
                    },
                    Node::Divider { visible: false },
                    Node::Block {
                        header: HeaderOptions {
                            level: 0u8,
                            nodes: &[],
                            id: None
                        },
                        metadata: &[],
                        nodes: &[Node::Inline {
                            nodes: &[
                                Node::Text {
                                    options: None,
                                    text: "yes, i have sources; "
                                },
                                Node::Text {
                                    options: Some(TextOptions {
                                        style: "italic",
                                        size: "text"
                                    }),
                                    text: "academic"
                                },
                                Node::Text {
                                    options: None,
                                    text: " sources."
                                }
                            ]
                        }]
                    }
                ]
            }]
        }};

        assert_eq!(page_tokens.to_string(), quote_tokens.to_string());
    }
}
