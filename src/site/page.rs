use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Page {
    // pub source: PathBuf,
    pub route: PathBuf,
    pub output_path: PathBuf,
    pub title: String,
    pub markdown: String,
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
