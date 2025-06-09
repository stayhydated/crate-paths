use check_keyword::CheckKeyword;
use proc_macro::TokenStream;
use quote::quote;

pub fn path_val(input: TokenStream) -> TokenStream {
    let syn_path = syn::parse_macro_input!(input as syn::Path);

    let path_str = &syn_path
        .segments
        .iter()
        .map(|seg| seg.ident.to_string())
        .collect::<Vec<_>>()
        .join("::");

    let expanded = quote! {
        {
          #[allow(unused_imports)]
          use #syn_path as _;
            crate_paths::Path::new(#path_str)
        }
    };

    expanded.into()
}

pub fn path(input: TokenStream) -> TokenStream {
    let input_clone = input.clone();
    let syn_path = syn::parse_macro_input!(input as syn::Path);

    let item_name = &syn_path.segments.last().unwrap().ident.to_string();

    let safe_name = match item_name.is_keyword() {
        true => format!("r#{}", item_name),
        false => item_name.to_string(),
    };

    let safe_ident = syn::parse_str::<syn::Ident>(&safe_name).unwrap();

    let value: proc_macro2::TokenStream = path_val(input_clone).into();

    let expanded = quote! {
        #[allow(non_upper_case_globals)]
        pub const #safe_ident: crate_paths::Path = #value;
    };

    expanded.into()
}
