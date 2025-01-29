/// definitive macro for component development
///
/// intended use:
///
/// compose! { Frame (classes = "flex flex-row", border = "true") {
///     Frame (classes = "flex flex-column", border = "false") {
///         Image (src = "/assets/avatar.png", classes = "rounded-25");
///         Frame (classes = "flex flex-column text-align-center", border = "false") {
///             Text (classes = "h2") { "hi, im aenri" };
///             Frame (classes = "flex flex-column text-align-center", border = "false") {
///                 Text (classes = "h4") { "ageless therian puppy developer" };
///                 Text (classes = "h4") { "love any and all, do no harm" };
///                 Text (classes = "h4") { "i love my nine partners!!" };
///             };
///         };
///     };
/// }};
///
#[macro_export]
macro_rules! compose {
    { $($all:tt)* } => {
        #[allow(non_snake_case, unused_mut)] $crate::component! { $($all)* }
    }
}

#[macro_export]
macro_rules! component {
    {
        $component_type:ident ($($args:tt)*) { $($inner:tt)* }
    } => {
        {
            let mut LOCAL_component = $component_type::create(::core::convert::From::from([arguments!($($args)*)]));
            LOCAL_component.insert(::core::convert::From::from([
                component! { boxed $($inner)* }
            ]))
            LOCAL_argument
        }
    };
    {
        $component_type:ident ($($args:tt)*) { $($inner:tt)* }
        $($tail:tt)*
    } => {
        {
            let mut LOCAL_component = $component_type::create(::core::convert::From::from([arguments!($($args)*)]));
            LOCAL_component.insert(::core::convert::From::from([
                component! { boxed $($inner)* }
            ]))
            LOCAL_argument
        },
        component! { $($tail)* }
    };
    {
        boxed $component_type:ident ($($args:tt)*) { $($inner:tt)* }
    } => {
       {
           let mut LOCAL_component = $component_type::create(::core::convert::From::from([arguments!($($args)*)]));
           LOCAL_component.insert(::core::convert::From::from([
               component! { boxed $($inner)* }
           ]))
           let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
           LOCAL_box as Box<dyn Component>
       }
    };
    {
        boxed $component_type:ident ($($args:tt)*) { $($inner:tt)* }
        $($tail:tt)*
    } => {
        {
            let mut LOCAL_component = $component_type::create(::core::convert::From::from([arguments!($($args)*)]));
            LOCAL_component.insert(::core::convert::From::from([
                component! { boxed $($inner)* }
            ]))
            let mut LOCAL_box = ::std::boxed::Box::new(LOCAL_component);
            LOCAL_box as Box<dyn Component>
        },
        component! { boxed $($tail)* }
    };
}
#[macro_export]
macro_rules! arguments {
    (
        $name:ident = $value:expr
    ) => {
        (String::from(stringify!($name)), {
            let mut LOCAL_argument = Argument::new();
            LOCAL_argument.insert($expr);
            LOCAL_argument
        })
    };
    (
        $name:ident
    ) => {

    };
}
