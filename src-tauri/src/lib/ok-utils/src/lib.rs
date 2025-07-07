use proc_macro::TokenStream;
use quote::quote;
use std::env;
use std::path::Path;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn ts_export(
    _attr: TokenStream,
    input: TokenStream,
) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Get the current crate's manifest directory
    let manifest_dir =
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");

    // Find workspace root by looking for Cargo.toml with [workspace]
    let workspace_root = find_workspace_root(&manifest_dir)
        .expect("Could not find workspace root");

    // Create the bindings path relative to workspace root
    let bindings_path = Path::new(&workspace_root)
        .parent()
        .unwrap()
        .join("src")
        .join("shared")
        .join("bindings")
        .to_string_lossy()
        .to_string();

    let path_str = format!(
        "{}{}",
        bindings_path,
        if bindings_path.ends_with('/') {
            ""
        } else {
            "/"
        }
    );

    let expanded = quote! {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, ts_rs::TS)]
        #[ts(export, export_to = #path_str)]
        #input
    };

    TokenStream::from(expanded)
}

fn find_workspace_root(start_dir: &str) -> Option<String> {
    let mut current_dir = Path::new(start_dir);

    loop {
        let cargo_toml = current_dir.join("Cargo.toml");
        if cargo_toml.exists() {
            // Check if this Cargo.toml contains [workspace]
            if let Ok(contents) = std::fs::read_to_string(&cargo_toml) {
                if contents.contains("[workspace]") {
                    return Some(current_dir.to_string_lossy().to_string());
                }
            }
        }

        // Move up one directory
        if let Some(parent) = current_dir.parent() {
            current_dir = parent;
        } else {
            break;
        }
    }

    None
}
