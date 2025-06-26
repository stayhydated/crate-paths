use crate::item::ItemEntry;
use crate::item_kind::ItemKind;
use scraper::{Html, Selector};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("failed to parse selector: {0}")]
    SelectorParse(#[from] scraper::error::SelectorErrorKind<'static>),
    #[error("empty href : {0}")]
    EmptyHref(String),
    #[error(
        "content is empty. Maybe docs.rs page failed to build the target crate, or the html parser failed."
    )]
    EmptyContent,
}

pub fn parse_html_to_items(
    crate_name: &str,
    html_str: &str,
) -> Result<Vec<ItemEntry>, ParserError> {
    let document = Html::parse_document(html_str);

    // Look for every <a> under <ul class="all-items">.
    // Each link’s href looks like "struct.Arg.html" or "builder/struct.Arg.html",
    // so we can infer ItemKind from the prefix (before the first '.').
    let selector = Selector::parse("ul.all-items li a").map_err(ParserError::SelectorParse)?;

    let mut items_map: HashMap<String, ItemEntry> = HashMap::new();
    for a in document.select(&selector) {
        // "Arg" or "builder::Arg" as link‐text
        let path = a.text().collect::<Vec<_>>().concat();

        // Href attribute, e.g. "struct.Arg.html" or "builder/struct.Arg.html"
        let href = a
            .value()
            .attr("href")
            .ok_or(ParserError::EmptyHref(path.clone()))?;

        // Infer the kind from the filename prefix: split on '/', then split on '.'
        let kind = match href
            .split('/')
            .next_back()
            .and_then(|filename| filename.split('.').next())
        {
            Some("struct") => ItemKind::Struct,
            Some("enum") => ItemKind::Enum,
            Some("trait") => ItemKind::Trait,
            Some("traits") => ItemKind::Trait,
            Some("type") => ItemKind::TypeAlias,
            Some("constant") => ItemKind::Constant,
            Some("fn") => ItemKind::Function,
            Some("macro") => ItemKind::Macro,
            Some("union") => ItemKind::Union,
            Some("mod") => ItemKind::Module,
            Some("static") => ItemKind::Static,
            Some("derive") => ItemKind::ProcDerive,
            Some("attr") => ItemKind::ProcAttribute,
            Some("extern_type") | Some("externtype") => ItemKind::ExternType,
            // rust libs only
            Some("primitive") => ItemKind::Primitive,
            Some("keyword") => ItemKind::Keyword,
            _ => {
                eprintln!("Warning: Could not determine ItemKind for href: {}", href);
                continue;
            },
        };

        let item_name = path.split("::").last().unwrap().to_string();
        let key = format!("{}::{}", crate_name, path);

        if let Some(existing_item) = items_map.get_mut(&key)
            && !existing_item.kinds().contains(&kind)
        {
            let mut new_kinds = existing_item.kinds().clone();
            new_kinds.push(kind);
            *existing_item = ItemEntry::new(
                existing_item.crate_name().clone(),
                existing_item.item_name().clone(),
                existing_item.path().clone(),
                new_kinds,
            );
        } else if !items_map.contains_key(&key) {
            items_map.insert(
                key,
                ItemEntry::new(crate_name.to_owned(), item_name, path, vec![kind]),
            );
        }
    }

    let mut items: Vec<ItemEntry> = items_map.into_values().collect();
    items.sort_by(|a, b| a.path().cmp(b.path()));

    match items.is_empty() {
        true => Err(ParserError::EmptyContent),
        false => Ok(items),
    }
}
