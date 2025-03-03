#![feature(let_chains)]
#![allow(dead_code)]
use proc_macro::TokenStream;
mod parse;
pub(crate) mod entities;

#[cfg(test)]
mod tests;

#[proc_macro]
pub fn compose(tt: TokenStream) -> TokenStream {

    tt

}
