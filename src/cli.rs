use clap::{Parser, Subcommand};
use anyhow::{bail, Context};
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::Result;

#[derive(Parser, Debug)]
#[command(
    name = "oxidepress",
    version,
    about = "oxidepress - a tiny static site generator"
)]
pub struct Cli {
    /// プロジェクトルート（省略時はカレント）
    #[arg(long, global = true)]
    pub root: Option<PathBuf>,

    /// 設定ファイル（省略時は root/config.toml）
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// 新規プロジェクトを作成（ディレクトリ作成 + テンプレ配置）
    New {
        /// 作成するプロジェクトディレクトリ名
        name: String,

        /// 既存でも上書きする（危険なので実装側で厳密に扱う）
        #[arg(long)]
        force: bool,
    },

    /// カレント（または --root）を既存ディレクトリから初期化
    Init {
        /// 既存ファイルがあっても生成する
        #[arg(long)]
        force: bool,
    },

    /// content を dist にビルド
    Build {
        /// 既存 dist を削除してから生成
        #[arg(long)]
        clean: bool,
    },

    /// ローカルサーバで配信（必要なら watch も）
    Serve {
        /// bind 先アドレス
        #[arg(long, default_value = "127.0.0.1")]
        host: String,

        /// ポート
        #[arg(long, default_value_t = 3000)]
        port: u16,

        /// ファイル変更を監視して自動ビルド（後で実装）
        #[arg(long)]
        watch: bool,
    },

    /// dist を削除
    Clean,
}

impl Cli {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }

    pub fn run(self) -> Result<()> {
        let root = self
            .root
            .unwrap_or(std::env::current_dir().context("failed to get current dir")?);

       let config_path = self.config;

        match self.command {
            Command::New { name, force } => {
                let dir = root.join(&name);
                new_project(&dir, force)
                    .with_context(|| format!("failed to create new project: {}", dir.display()))
            }
            Command::Init { force } => {
                init_project(&root, force)
                    .with_context(|| format!("failed to init project: {}", root.display()))
            }
            Command::Build { clean } => {
                // let cfg = crate::config::Config::load(&root, self.config.as_deref())?;
                // cfg.validate()?;
                // let site = crate::site::Site::from_fs(&cfg)?;
                // crate::build::build(&cfg, &site, clean)?;
                let cfg = crate::config::Config::load(&root, config_path.as_deref())?;
                cfg.validate()?;

                let site = crate::site::Site::from_fs(&cfg)?;
                crate::build::build(&cfg, &site, clean)?;

                Ok(())
            }
            Command::Clean => {
                let cfg = crate::config::Config::load(&root, config_path.as_deref())?;
                if cfg.output_dir.exists() {
                    fs::remove_dir_all(&cfg.output_dir).with_context(|| {
                        format!("failed to remove output_dir: {}", cfg.output_dir.display())
                    })?;
                }
                Ok(())
            }
            Command::Serve { host, port, watch } => {
                let _cfg = crate::config::Config::load(&root, config_path.as_deref())?;
                let _ = (host, port, watch);
                bail!("serve is not implemented yet")
            }
        }
    }
}








// ---- file-local helpers (cli.rs 内で完結) ----

fn new_project(project_dir: &Path, force: bool) -> Result<()> {
    if project_dir.exists() {
        if !force {
            bail!(
                "project dir already exists: {} (use --force to overwrite)",
                project_dir.display()
            );
        }
    } else {
        fs::create_dir_all(project_dir)
            .with_context(|| format!("failed to create dir: {}", project_dir.display()))?;
    }

    // 必要最低限の構造
    fs::create_dir_all(project_dir.join("content"))?;
    fs::create_dir_all(project_dir.join("templates"))?;

    write_if_absent_or_force(&project_dir.join("config.toml"), DEFAULT_CONFIG_TOML, force)?;
    write_if_absent_or_force(&project_dir.join("templates/page.html"), DEFAULT_PAGE_HTML, force)?;
    write_if_absent_or_force(&project_dir.join("content/index.md"), DEFAULT_INDEX_MD, force)?;

    Ok(())
}

fn init_project(root: &Path, force: bool) -> Result<()> {
    fs::create_dir_all(root.join("content"))?;
    fs::create_dir_all(root.join("templates"))?;

    write_if_absent_or_force(&root.join("config.toml"), DEFAULT_CONFIG_TOML, force)?;
    write_if_absent_or_force(&root.join("templates/page.html"), DEFAULT_PAGE_HTML, force)?;

    Ok(())
}

fn write_if_absent_or_force(path: &Path, content: &str, force: bool) -> Result<()> {
    if path.exists() && !force {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)
        .with_context(|| format!("failed to write file: {}", path.display()))?;
    Ok(())
}

const DEFAULT_CONFIG_TOML: &str = r#"
content_dir = "content"
output_dir = "dist"
templates_dir = "templates"
# base_url = "https://example.com"
"#;

const DEFAULT_PAGE_HTML: &str = r#"<!doctype html>
<html lang="ja">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width,initial-scale=1" />
    <title>{{ title }}</title>
  </head>
  <body>
    <main>
      <h1>{{ title }}</h1>
      <article>{{ content | safe }}</article>
    </main>
  </body>
</html>
"#;

const DEFAULT_INDEX_MD: &str = r#"# Hello oxidepress

It works.
"#;