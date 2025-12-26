// src/config.rs
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Markdown などの入力
    pub content_dir: PathBuf,
    /// 生成先
    pub output_dir: PathBuf,
    /// テンプレートの場所（テンプレエンジンは後で差し替え可能）
    pub templates_dir: PathBuf,
    /// 例: "https://example.com"（任意）
    pub base_url: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            content_dir: PathBuf::from("content"),
            output_dir: PathBuf::from("dist"),
            templates_dir: PathBuf::from("templates"),
            base_url: None,
        }
    }
}

impl Config {
    pub fn load(root: &Path, config_path: Option<&Path>) -> Result<Self> {
        let path = config_path
            .map(|p| root.join(p))
            .unwrap_or_else(|| root.join("config.toml"));

        if !path.exists() {
            // config.toml が無い場合はデフォルトを採用（プロト段階で便利）
            return Ok(Self::default());
        }

        let s = fs::read_to_string(&path)
            .with_context(|| format!("failed to read config file: {}", path.display()))?;

        let mut cfg: Config = toml::from_str(&s)
            .with_context(|| format!("failed to parse config as TOML: {}", path.display()))?;

        // root 基準に正規化（相対パスを root からのパスにする）
        if cfg.content_dir.is_relative() {
            cfg.content_dir = root.join(&cfg.content_dir);
        }
        if cfg.output_dir.is_relative() {
            cfg.output_dir = root.join(&cfg.output_dir);
        }
        if cfg.templates_dir.is_relative() {
            cfg.templates_dir = root.join(&cfg.templates_dir);
        }

        Ok(cfg)
    }

    pub fn validate(&self) -> Result<()> {
        // 「どのパスが原因か」をエラーメッセージに含める
        anyhow::ensure!(
            self.content_dir.exists(),
            "content_dir not found: {}",
            self.content_dir.display()
        );

        // 必要なら追加（テンプレを使う段階になったら有効化）
        // anyhow::ensure!(
        //     self.templates_dir.exists(),
        //     "templates_dir not found: {}",
        //     self.templates_dir.display()
        // );

        Ok(())
    }
}
