mod page;

use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::config::Config;
use crate::error::Result;

pub use page::Page;

#[derive(Debug, Clone)]
pub struct Site {
    pub pages: Vec<Page>,
}

impl Site {
    /// ここは「構造を決める」フェーズ：
    /// - content 配下の md を列挙
    /// - 出力先パスを決める
    /// - 文字列として本文を保持する（まだ HTML 化しない）
    pub fn from_fs(cfg: &Config) -> Result<Self> {
        let mut pages = Vec::new();

        for entry in WalkDir::new(&cfg.content_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if path.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }

            let markdown = fs::read_to_string(path)?;
            let rel = path.strip_prefix(&cfg.content_dir).unwrap_or(path);

            // 例: content/blog/a.md -> dist/blog/a/index.html （SSGっぽいURL）
            let output_path = to_output_path(&cfg.output_dir, rel);

            pages.push(Page {
                // source: path.to_path_buf(),
                route: rel.to_path_buf(),
                output_path,
                title: page::infer_title(&markdown, rel),
                markdown,
            });
        }

        // route で安定ソート（ビルドの再現性のため）
        pages.sort_by(|a, b| a.route.cmp(&b.route));

        Ok(Self { pages })
    }
}

fn to_output_path(out_dir: &Path, rel_md: &Path) -> PathBuf {
    let mut rel = rel_md.to_path_buf();
    rel.set_extension(""); // a.md -> a
    // a -> a/index.html
    out_dir.join(rel).join("index.html")
}