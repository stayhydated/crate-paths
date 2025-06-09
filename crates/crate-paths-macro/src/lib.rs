mod impls;

use proc_macro::TokenStream;

/// usage:
/// ```rust,ignore
/// path!(std::sync::Arc);
/// ```
///
/// This expands to
/// ```rust,ignore
/// #[allow(non_upper_case_globals)]
/// pub const Arc: crate_paths::Path = {
///     #[allow(unused_imports)]
///     use std::sync::Arc as _;
///
///     crate_paths::Path::new("std::sync::Arc")
/// };
/// ```
#[proc_macro]
pub fn path(input: TokenStream) -> TokenStream {
	impls::path(input)
}

/// Usage:
///
/// ```rust,ignore
/// path_val!(std::sync::Arc);
/// ```
///
/// This expands to:
///
/// ```rust,ignore
/// {
///   #[allow(unused_imports)]
///   use std::sync::Arc as _;
///
///   crate_paths::Path::new("std::sync::Arc")
/// }
/// ```
///
/// Which you would later assign to a constant:
///
/// ```rust,ignore
/// const SOME_CONSTANT: crate_paths::Path = path_val!(std::sync::Arc);
/// ```
#[proc_macro]
pub fn path_val(input: TokenStream) -> TokenStream {
	impls::path_val(input)
}
