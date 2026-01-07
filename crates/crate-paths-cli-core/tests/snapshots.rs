use crate_paths_cli_core::parser::parse_html_to_items;
use crate_paths_cli_core::tree::ModTree;

fn load_fixture(name: &str) -> String {
    let path = format!(
        "{}/tests/fixtures/{}.html",
        env!("CARGO_MANIFEST_DIR"),
        name
    );
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read fixture {}: {}", path, e))
}

#[test]
fn test_sample_crate_parsing() {
    let html = load_fixture("sample_crate");
    let items = parse_html_to_items("sample_crate", &html).unwrap();
    let tree = ModTree::new(items);

    insta::assert_snapshot!("sample_crate", tree.to_rust_module_string());
}

#[test]
fn test_sample_crate_list() {
    let html = load_fixture("sample_crate");
    let items = parse_html_to_items("sample_crate", &html).unwrap();
    let tree = ModTree::new(items);

    insta::assert_snapshot!("sample_crate_list", tree.to_string_list());
}

#[test]
fn test_derive_crate_parsing() {
    let html = load_fixture("derive_crate");
    let items = parse_html_to_items("derive_crate", &html).unwrap();
    let tree = ModTree::new(items);

    insta::assert_snapshot!("derive_crate", tree.to_rust_module_string());
}

#[test]
fn test_derive_crate_list() {
    let html = load_fixture("derive_crate");
    let items = parse_html_to_items("derive_crate", &html).unwrap();
    let tree = ModTree::new(items);

    insta::assert_snapshot!("derive_crate_list", tree.to_string_list());
}
