use derive_more::Display;

/// A `syn::Path` like definition.
///
/// This is a workaround for `syn::Path` since it isn't const.
#[derive(Debug, Display)]
pub struct Path<'a>(&'a str);

impl<'a> Path<'a> {
    pub const fn new(path: &'a str) -> Self {
        Path(path)
    }
}

impl quote::ToTokens for Path<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let parsed: syn::Path = syn::parse_str(self.0).unwrap();
        parsed.to_tokens(tokens);
    }
}

#[cfg(feature = "macros")]
pub use crate_paths_macros::*;
