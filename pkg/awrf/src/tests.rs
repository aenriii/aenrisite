extern crate test;
use std::{collections::HashMap, mem::ManuallyDrop};

use test::Bencher;


use super::*;


// #[test]
// fn text_component_works() {
//     use components::Text;
//     let example = String::from(
//         "<span class=\"text\">Hello World!</span>"
//     );

//     let rendered = compose! {
//         Text (classes = "text") {
//             "Hello World!"
//         }
//     };

//     assert_eq!(example, rendered.render())
// }


#[test]
fn test_argument_works() {
    use argument::Argument;

    let mut arg = Argument::new();

    arg.insert("hello world".to_string());

    assert_eq!(arg.try_get::<u8>(), None);
    assert_eq!(arg.try_get::<Vec<String>>(), None);
    assert_eq!(arg.try_get::<String>(), Some("hello world".to_string()))
}
#[test]
fn test_argument_drops() {
    use argument::Argument;

    let mut arg = Argument::new();

    arg.insert("hello world".to_string());

    let mut mdrop = ManuallyDrop::new(arg);
    unsafe { ManuallyDrop::drop(&mut mdrop) };
}

#[bench]
fn bench_argument_works(b: &mut Bencher) {

    b.iter(|| {
        use argument::Argument;

        let mut arg = Argument::new();

        arg.insert("hello world".to_string());

        assert_eq!(arg.try_get::<String>(), Some("hello world".to_string()))
    });

}
#[bench]
fn bench_hashmap_comparison(b: &mut Bencher) {

    b.iter(|| {
        let mut arg = HashMap::new();

        arg.insert("hello world", "hello world".to_string());

        assert_eq!(arg.get("hello world"), Some(&("hello world".to_string())))
    });

}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Complex {
    pub string: String,
    pub other_struct: OtherStruct,
    pub arr: Vec<u8>
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct OtherStruct(u32);

#[bench]
fn bench_complex_struct(b: &mut Bencher) {
    let item = Complex {
        string: "Hello World".to_string(),
        other_struct: OtherStruct(88829),
        arr: vec![8, 22, 13]
    };
    b.iter(|| {
        use argument::Argument;

        let mut arg = Argument::new();

        arg.insert(item.clone());

        assert_eq!(arg.try_get::<Complex>(), Some(item.clone()));
    });
}
#[bench]
fn bench_hashmap_complex_struct(b: &mut Bencher) {
    let item = Complex {
        string: "Hello World".to_string(),
        other_struct: OtherStruct(88829),
        arr: vec![8, 22, 13]
    };
    b.iter(|| {
        let mut arg = HashMap::new();

        arg.insert("hello world", item.clone());

        assert_eq!(arg.get("hello world"), Some(&item.clone()))
    });
}
// #[bench]
// fn bench_default_construct(b: &mut Bencher) {
//     use crate::components::*;
//     use crate::argument::Argument;
//     fn _a() -> Frame {
//          #[allow(non_snake_case, unused_mut)] {
//             let mut LOCAL_component = Frame::create(
//                 core::convert::From::from([
//                     (String::from("class"), {
//                         let mut LOCAL_argument = Argument::new();
//                         LOCAL_argument.insert(String::from("flex flex-row"));
//                         LOCAL_argument
//                     }),
//                     (String::from("border"), {
//                         let mut LOCAL_argument = Argument::new();
//                         LOCAL_argument.insert(true);
//                         LOCAL_argument
//                     })
//                 ])
//             );
//             LOCAL_component.insert(core::convert::From::from([
//                 {
//                     let mut LOCAL_component = Frame::create(
//                         core::convert::From::from([
//                             (String::from("class"), {
//                                 let mut LOCAL_argument = Argument::new();
//                                 LOCAL_argument.insert(String::from("flex flex-column"));
//                                 LOCAL_argument
//                             }),
//                             (String::from("border"), {
//                                 let mut LOCAL_argument = Argument::new();
//                                 LOCAL_argument.insert(false);
//                                 LOCAL_argument
//                             })
//                         ])
//                     );

//                     LOCAL_component.insert(core::convert::From::from([
//                         {
//                             let mut LOCAL_component = Image::create(
//                                 core::convert::From::from([
//                                     (String::from("src"), {
//                                         let mut LOCAL_argument = Argument::new();
//                                         LOCAL_argument.insert(String::from("/assets/avatar.png"));
//                                         LOCAL_argument
//                                     }),
//                                     (String::from("class"), {
//                                         let mut LOCAL_argument = Argument::new();
//                                         LOCAL_argument.insert(String::from("rounded-25"));
//                                         LOCAL_argument
//                                     })
//                                 ])
//                             );
//                             let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
//                             LOCAL_box as Box<dyn Component>
//                         },
//                         {
//                             let mut LOCAL_component = Frame::create(
//                                 core::convert::From::from([
//                                     (String::from("class"), {
//                                         let mut LOCAL_argument = Argument::new();
//                                         LOCAL_argument.insert(String::from("flex flex-column text-align-center"));
//                                         LOCAL_argument
//                                     }),
//                                     (String::from("border"), {
//                                         let mut LOCAL_argument = Argument::new();
//                                         LOCAL_argument.insert(false);
//                                         LOCAL_argument
//                                     })
//                                 ])
//                             );

//                             LOCAL_component.insert(core::convert::From::from([
//                                 {
//                                     let mut LOCAL_component = Text::create(
//                                         core::convert::From::from([
//                                             (String::from("class"), {
//                                                 let mut LOCAL_argument = Argument::new();
//                                                 LOCAL_argument.insert(String::from("h2"));
//                                                 LOCAL_argument
//                                             })
//                                         ])
//                                     );

//                                     LOCAL_component.insert(core::convert::From::from([
//                                         {
//                                             let mut LOCAL_component = String::from("hi, im aenri");
//                                             let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
//                                             LOCAL_box as Box<dyn Component>
//                                         }
//                                     ]));

//                                     let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
//                                     LOCAL_box as Box<dyn Component>
//                                 },
//                                 {
//                                     let mut LOCAL_component = Frame::create(
//                                         core::convert::From::from([
//                                             (String::from("class"), {
//                                                 let mut LOCAL_argument = Argument::new();
//                                                 LOCAL_argument.insert(String::from("flex flex-column text-align-center"));
//                                                 LOCAL_argument
//                                             }),
//                                             (String::from("border"), {
//                                                 let mut LOCAL_argument = Argument::new();
//                                                 LOCAL_argument.insert(false);
//                                                 LOCAL_argument
//                                             })
//                                         ])
//                                     );

//                                     LOCAL_component.insert(core::convert::From::from([
//                                         {
//                                             let mut LOCAL_component = Text::create(
//                                                 core::convert::From::from([
//                                                     (String::from("class"), {
//                                                         let mut LOCAL_argument = Argument::new();
//                                                         LOCAL_argument.insert(String::from("h4"));
//                                                         LOCAL_argument
//                                                     })
//                                                 ])
//                                             );

//                                             LOCAL_component.insert(core::convert::From::from([
//                                                 {
//                                                     let mut LOCAL_component = String::from("ageless therian puppy developer");
//                                                     let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
//                                                     LOCAL_box as Box<dyn Component>
//                                                 }
//                                             ]));

//                                             let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
//                                             LOCAL_box as Box<dyn Component>
//                                         },
//                                         {
//                                             let mut LOCAL_component = Text::create(
//                                                 core::convert::From::from([
//                                                     (String::from("class"), {
//                                                         let mut LOCAL_argument = Argument::new();
//                                                         LOCAL_argument.insert(String::from("h4"));
//                                                         LOCAL_argument
//                                                     })
//                                                 ])
//                                             );

//                                             LOCAL_component.insert(core::convert::From::from([
//                                                 {
//                                                     let mut LOCAL_component = String::from("love any and all, do no harm");
//                                                     let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
//                                                     LOCAL_box as Box<dyn Component>
//                                                 }
//                                             ]));

//                                             let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
//                                             LOCAL_box as Box<dyn Component>
//                                         },
//                                         {
//                                             let mut LOCAL_component = Text::create(
//                                                 core::convert::From::from([
//                                                     (String::from("class"), {
//                                                         let mut LOCAL_argument = Argument::new();
//                                                         LOCAL_argument.insert(String::from("h4"));
//                                                         LOCAL_argument
//                                                     })
//                                                 ])
//                                             );

//                                             LOCAL_component.insert(core::convert::From::from([
//                                                 {
//                                                     let mut LOCAL_component = String::from("i love my ten partners!!");
//                                                     let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
//                                                     LOCAL_box as Box<dyn Component>
//                                                 }
//                                             ]));

//                                             let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
//                                             LOCAL_box as Box<dyn Component>
//                                         },

//                                     ]));

//                                     let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
//                                     LOCAL_box as Box<dyn Component>
//                                 }
//                             ]));

//                             let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
//                             LOCAL_box as Box<dyn Component>
//                         }
//                     ]));

//                     let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
//                     LOCAL_box as Box<dyn Component>

//                 }
//             ]));
//             LOCAL_component
//         }
//     }
//     b.iter(|| {
//         _a().render()
//     });
// }
