#![feature(proc_macro_span)]
#![feature(path_file_prefix)]
extern crate proc_macro;

use proc_macro::{Span, TokenStream};

#[proc_macro]
pub fn day(_item: TokenStream) -> TokenStream {
    day_impl().expect("filename must be of the form 'dayXX.rs'")
}

fn day_impl() -> Option<TokenStream> {
    let path = Span::call_site().source().source_file().path();
    let prefix = path.file_prefix().and_then(|filename| filename.to_str())?;
    let day = prefix.strip_prefix("day")?;
    format!("\"{}\"", day).parse().ok()
}
