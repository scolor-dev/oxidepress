use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Debug, Clone)]
pub struct Page {
    // pub source: PathBuf,
    pub route: PathBuf,
    pub output_path: PathBuf,
    pub title: String,
    pub meta: PageMeta,
    pub markdown: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageMeta {
    pub title: Option<String>,
    pub draft: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub date: Option<String>,
}

pub fn parse_front_matter(markdown: &str) -> Result<(PageMeta, String)> {
    let mut lines = markdown.lines();
    let Some(first) = lines.next() else {
        return Ok((PageMeta::default(), String::new()));
    };
    if first.trim_end() != "+++" {
        return Ok((PageMeta::default(), markdown.to_string()));
    }

    let mut toml_lines = Vec::new();
    let mut found_end = false;
    for line in lines.by_ref() {
        if line.trim_end() == "+++" {
            found_end = true;
            break;
        }
        toml_lines.push(line);
    }

    if !found_end {
        return Ok((PageMeta::default(), markdown.to_string()));
    }

    let toml_str = toml_lines.join("\n");
    let meta = if toml_str.trim().is_empty() {
        PageMeta::default()
    } else {
        toml::from_str(&toml_str)?
    };
    let body = lines.collect::<Vec<_>>().join("\n");
    Ok((meta, body))
}

pub fn infer_title(markdown: &str, fallback_route: &std::path::Path) -> String {
    for line in markdown.lines() {
        if let Some(rest) = line.strip_prefix("# ") {
            let t = rest.trim();
            if !t.is_empty() {
                return t.to_string();
            }
        }
    }
    fallback_route
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Untitled")
        .to_string()
}
