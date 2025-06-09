use crate::item::ItemEntry;
use std::{
    collections::BTreeMap,
    io::Write,
    process::{Command, Stdio},
};

#[derive(Clone, Debug, Default)]
struct ModuleScope {
    items: Vec<ItemEntry>,
    submodules: BTreeMap<String, ModuleScope>,
}

pub struct ModTree {
    items: Vec<ItemEntry>,
}

impl ModTree {
    pub fn new(items: Vec<ItemEntry>) -> Self {
        Self { items }
    }

    pub fn to_string_list(&self) -> String {
        self.items
            .iter()
            .map(|item| {
                let kind_str = format!("{:?}", item.kinds());
                format!("• {} {}", item.path(), kind_str)
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn to_rust_module_string(&self) -> String {
        let mut root_scope = ModuleScope::default();

        for item_entry in &self.items {
            // item_entry.path is the path within the crate, e.g., "module::submodule::ItemName" or "ItemName"
            let path_components: Vec<&str> = item_entry.path().split("::").collect();

            let mut current_scope = &mut root_scope;

            for &module_name_str in path_components
                .iter()
                .take(path_components.len().saturating_sub(1))
            {
                current_scope = current_scope
                    .submodules
                    .entry(module_name_str.to_string())
                    .or_default();
            }

            current_scope.items.push(item_entry.clone());
        }

        let mut lines = Vec::<String>::new();
        lines.push("#![allow(dead_code, non_upper_case_globals)]".to_owned());

        format_scope_recursive(&root_scope, &mut lines);

        let raw = if lines.is_empty() {
            String::new()
        } else {
            lines.join(" ")
        };

        format_with_rustfmt(&raw).unwrap_or(raw)
    }
}

fn format_scope_recursive(scope: &ModuleScope, lines: &mut Vec<String>) {
    if !scope.items.is_empty() {
        for item_entry in &scope.items {
            lines.push(item_entry.into_writable());
        }
    }

    for (module_name, sub_scope) in &scope.submodules {
        lines.push(format!("pub mod {} {{", module_name));
        format_scope_recursive(sub_scope, lines);
        lines.push('}'.to_string());
    }
}

fn format_with_rustfmt(code: &str) -> std::io::Result<String> {
    let mut child = Command::new("rustfmt")
        .args(["--emit", "stdout"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(code.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(std::io::Error::other(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}
