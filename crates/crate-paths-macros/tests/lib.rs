#[cfg(test)]
mod tests {
    use crate_paths_macros::{path, path_val};
    use quote::{ToTokens as _, quote};

    path!(std::sync::Arc);

    const ARCH_FRFR: crate_paths::Path = path_val!(std::env::consts::ARCH);

    #[test]
    fn test_path_value() {
        let val_arc = quote! { std::sync::Arc };
        assert_eq!(val_arc.to_string(), Arc.to_token_stream().to_string());

        let val_arch = quote! { std::env::consts::ARCH };
        assert_eq!(
            val_arch.to_string(),
            ARCH_FRFR.to_token_stream().to_string()
        );
    }
}
