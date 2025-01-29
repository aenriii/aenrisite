use std::collections::HashMap;

use crate::argument::Argument;

mod fragment;
pub use fragment::Fragment;

mod frame;
pub use frame::Frame;

mod text;
pub use text::Text;

mod image;
pub use image::Image;

pub trait Component {
    fn create(options: HashMap<String, Argument>) -> Self where Self : Sized;
    fn insert(&mut self, children: Vec<Box<dyn Component>>);
    fn render(&self) -> String;
}

impl Component for String {
    fn create(options: HashMap<String, Argument>) -> Self where Self : Sized {
        return String::new();
    }
    fn insert(&mut self, children: Vec<Box<dyn Component>>) {
        for child in children {
            self.push_str(&child.render());
        }
    }
    fn render(&self) -> String {
        self.clone()
    }
}



// compose! { Frame (class = "flex flex-row", border = true) {
//     Frame (class = "flex flex-column", border = false) {
//         Image (src = "/assets/avatar.png", class = "rounded-25");
//         Frame (class = "flex flex-column text-align-center", border = false) {
//             Text (class = "h2") { "hi, im aenri" };
//             Frame (class = "flex flex-column text-align-center", border = false) {
//                 Text (class = "h4") { "ageless therian puppy developer" };
//                 Text (class = "h4") { "love any and all, do no harm" };
//                 Text (class = "h4") { "i love my ten partners!!" };
//             };
//         };
//     };
// }};
fn _a() {
    let _component = #[allow(non_snake_case, unused_mut)] {
        let mut LOCAL_component = Frame::create(
            core::convert::From::from([
                (String::from("class"), {
                    let mut LOCAL_argument = Argument::new();
                    LOCAL_argument.insert(String::from("flex flex-row"));
                    LOCAL_argument
                }),
                (String::from("border"), {
                    let mut LOCAL_argument = Argument::new();
                    LOCAL_argument.insert(true);
                    LOCAL_argument
                })
            ])
        );
        LOCAL_component.insert(core::convert::From::from([
            {
                let mut LOCAL_component = Frame::create(
                    core::convert::From::from([
                        (String::from("class"), {
                            let mut LOCAL_argument = Argument::new();
                            LOCAL_argument.insert(String::from("flex flex-column"));
                            LOCAL_argument
                        }),
                        (String::from("border"), {
                            let mut LOCAL_argument = Argument::new();
                            LOCAL_argument.insert(false);
                            LOCAL_argument
                        })
                    ])
                );

                LOCAL_component.insert(core::convert::From::from([
                    {
                        let mut LOCAL_component = Image::create(
                            core::convert::From::from([
                                (String::from("src"), {
                                    let mut LOCAL_argument = Argument::new();
                                    LOCAL_argument.insert(String::from("/assets/avatar.png"));
                                    LOCAL_argument
                                }),
                                (String::from("class"), {
                                    let mut LOCAL_argument = Argument::new();
                                    LOCAL_argument.insert(String::from("rounded-25"));
                                    LOCAL_argument
                                })
                            ])
                        );
                        let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
                        LOCAL_box as Box<dyn Component>
                    },
                    {
                        let mut LOCAL_component = Frame::create(
                            core::convert::From::from([
                                (String::from("class"), {
                                    let mut LOCAL_argument = Argument::new();
                                    LOCAL_argument.insert(String::from("flex flex-column text-align-center"));
                                    LOCAL_argument
                                }),
                                (String::from("border"), {
                                    let mut LOCAL_argument = Argument::new();
                                    LOCAL_argument.insert(false);
                                    LOCAL_argument
                                })
                            ])
                        );

                        LOCAL_component.insert(core::convert::From::from([
                            {
                                let mut LOCAL_component = Text::create(
                                    core::convert::From::from([
                                        (String::from("class"), {
                                            let mut LOCAL_argument = Argument::new();
                                            LOCAL_argument.insert(String::from("h2"));
                                            LOCAL_argument
                                        })
                                    ])
                                );

                                LOCAL_component.insert(core::convert::From::from([
                                    {
                                        let mut LOCAL_component = String::from("hi, im aenri");
                                        let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
                                        LOCAL_box as Box<dyn Component>
                                    }
                                ]));

                                let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
                                LOCAL_box as Box<dyn Component>
                            },
                            {
                                let mut LOCAL_component = Frame::create(
                                    core::convert::From::from([
                                        (String::from("class"), {
                                            let mut LOCAL_argument = Argument::new();
                                            LOCAL_argument.insert(String::from("flex flex-column text-align-center"));
                                            LOCAL_argument
                                        }),
                                        (String::from("border"), {
                                            let mut LOCAL_argument = Argument::new();
                                            LOCAL_argument.insert(false);
                                            LOCAL_argument
                                        })
                                    ])
                                );

                                LOCAL_component.insert(core::convert::From::from([
                                    {
                                        let mut LOCAL_component = Text::create(
                                            core::convert::From::from([
                                                (String::from("class"), {
                                                    let mut LOCAL_argument = Argument::new();
                                                    LOCAL_argument.insert(String::from("h4"));
                                                    LOCAL_argument
                                                })
                                            ])
                                        );

                                        LOCAL_component.insert(core::convert::From::from([
                                            {
                                                let mut LOCAL_component = String::from("ageless therian puppy developer");
                                                let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
                                                LOCAL_box as Box<dyn Component>
                                            }
                                        ]));

                                        let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
                                        LOCAL_box as Box<dyn Component>
                                    },
                                    {
                                        let mut LOCAL_component = Text::create(
                                            core::convert::From::from([
                                                (String::from("class"), {
                                                    let mut LOCAL_argument = Argument::new();
                                                    LOCAL_argument.insert(String::from("h4"));
                                                    LOCAL_argument
                                                })
                                            ])
                                        );

                                        LOCAL_component.insert(core::convert::From::from([
                                            {
                                                let mut LOCAL_component = String::from("love any and all, do no harm");
                                                let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
                                                LOCAL_box as Box<dyn Component>
                                            }
                                        ]));

                                        let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
                                        LOCAL_box as Box<dyn Component>
                                    },
                                    {
                                        let mut LOCAL_component = Text::create(
                                            core::convert::From::from([
                                                (String::from("class"), {
                                                    let mut LOCAL_argument = Argument::new();
                                                    LOCAL_argument.insert(String::from("h4"));
                                                    LOCAL_argument
                                                })
                                            ])
                                        );

                                        LOCAL_component.insert(core::convert::From::from([
                                            {
                                                let mut LOCAL_component = String::from("i love my ten partners!!");
                                                let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
                                                LOCAL_box as Box<dyn Component>
                                            }
                                        ]));

                                        let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
                                        LOCAL_box as Box<dyn Component>
                                    },

                                ]));

                                let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
                                LOCAL_box as Box<dyn Component>
                            }
                        ]));

                        let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
                        LOCAL_box as Box<dyn Component>
                    }
                ]));

                let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
                LOCAL_box as Box<dyn Component>

            }
        ]));
        LOCAL_component
    };
}
