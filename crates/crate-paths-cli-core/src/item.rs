use crate::item_kind::ItemKind;
use check_keyword::CheckKeyword;
use getset::Getters;
use inflector::Inflector;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, PartialEq, Serialize)]
#[getset(get = "pub")]
pub struct ItemEntry {
    crate_name: String,
    item_name: String,
    path: String,
    kinds: Vec<ItemKind>,
}

impl ItemEntry {
    pub fn new(crate_name: String, item_name: String, path: String, kinds: Vec<ItemKind>) -> Self {
        Self {
            crate_name,
            item_name,
            path,
            kinds,
        }
    }

    pub fn safe_name(&self) -> String {
        match self.item_name.is_keyword() {
            true => self.item_name.clone().into_safe(),
            false => self.item_name.clone(),
        }
    }

    pub fn full_path(&self) -> String {
        format!("{}::{}", self.crate_name, self.path)
    }

    pub fn into_writable(&self) -> String {
        let kinds_str = self
            .kinds
            .iter()
            .map(|k| k.to_string().to_sentence_case().to_lowercase())
            .collect::<Vec<_>>()
            .join(" + ");
        let item_kind = format!("pub {} `{}`", kinds_str, self.item_name());
        format!(
            r#"
			/// {}
			pub const {}: crate_paths::Path = crate_paths::Path::new("{}");
			"#,
            item_kind,
            self.safe_name(),
            self.full_path()
        )
    }
}
