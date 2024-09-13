macro_rules! tabbed_layout {
    (
        nonce = $nonce:ident
        $( $tab_title:literal $blk:expr )+
    ) => {
        crate::layouts::main_layout(crate::components::tabs(
            vec![
                $((
                    String::from($tab_title),
                    $blk
                )),*
            ],
            $nonce
        ))
    };
}

pub(crate) use tabbed_layout;
