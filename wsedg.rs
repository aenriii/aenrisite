// let radqueer_blog_page = marker!("../pages/radqueer.md", crate::blog::constructor)

// // turns into

// let radqueer_blog_page = {
//   // page(options: PageOptions, root_block: [commontype])
//   crate::blog::constructor::page(PageOptions {
//     title: String::from("the page you get sent when you ask about it"),
//     description: String::from("an explaination and defense of the radqueer position from my perspective"),
//     metadata: collection! {
//       special => String::from("true"),
//       slug => String::from("radqueer")
//     }},
//     // block(header_level: u8, header_text: String, id: Option<String>, children: Vec<[commontype]>
//     vec![
//       crate::blog::constructor::block(
//         1,
//         "what.",
//         None,
//         vec![
//           // paragraph(children), effectively a div
//           crate::blog::constructor::paragraph(vec![
//             crate::blog::constructor::text(TextOptions { alignment: String::from("center"), size: String::from("text"), ... }, "i would like us all to take a deep breath."),
//             crate::blog::constructor::text(TextOptions { alignment: String::from("center"), size: String::from("subtext"), ... }, "bear with me, it'll all make sense soon"),
//           ]),
//           // divider(visible: bool)
//           crate::blog::constructor::divider(false),
//           crate::blog::constructor::span(vec![
//             crate::blog::constructor::text(TextOptions { alignment: String::from("left"), size: String::from("text"), ... }, "yes, i have sources; "),
//             crate::blog::constructor::text(TextOptions { alignment: String::from("left"), size: String::from("text"), italic: true, ... }, "academic"),
//             crate::blog::constructor::text(TextOptions { alignment: String::from("left"), size: String::from("text"), ... }, " sources"),
//           ]),
//         ]
//       )
//     ];
//   );
// }

const RADQUEER_PAGE: Page = marker!("../pages/radqueer.md");

// into

const RADQUEER_PAGE: Page = Page {
    options: PageOptions {
        title: "the page you get sent when you ask about it",
        description: "an explaination and defense of the radqueer position from my perspective",
        metadata: &[("special", "true"), ("slug", "radqueer")],
    },
    nodes: &[Node::Block {
        header: HeaderOptions {
            level: 1, // can be left to 0 to disable header for this block
            text: Some("what."),
            id: None,
        },
        special: &[],
        nodes: &[
            Node::Block {
                header: HeaderOptions {
                    level: 0,
                    text: None,
                    id: None,
                },
                special: &[("alignment", "center")],
                nodes: &[
                    Node::Text {
                        options: None,
                        text: "i would like us all to take a deep breath.",
                    },
                    Node::Text {
                        options: Some(TextOptions {
                            style: "normal",
                            size: "subtext",
                        }),
                        text: "bear with me, it'll all make sense soon",
                    },
                ],
            },
            Node::Divider { visible: false },
            Node::Block {
                header: HeaderOptions {
                    level: 0,
                    text: None,
                    id: None,
                },
                special: &[],
                nodes: &[Node::Inline {
                    options: None,
                    nodes: &[
                        Node::Text {
                            options: None,
                            text: "yes, i have sources; ",
                        },
                        Node::Text {
                            options: Some(TextOptions {
                                style: "italic",
                                size: "text",
                            }),
                            text: "academic",
                        },
                        Node::Text {
                            options: None,
                            text: " sources.",
                        },
                    ],
                }],
            },
        ],
    }],
};
