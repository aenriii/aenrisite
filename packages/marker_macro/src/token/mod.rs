use proc_macro2::Span;
use proc_macro::TokenStream
use quote::quote_spanned;

pub mod process;


pub fn fail(span: Span, txt: impl AsRef<str>) -> TokenStream
{
	let txt = txt.as_ref();
	quote_spanned! { span => compile_error!(#txt) }.into()
}
