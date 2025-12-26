use std::fs;

use pulldown_cmark::{html, Options, Parser};
use tera::{Context, Tera};

use crate::config::Config;
use crate::error::Result;
use crate::site::{Page, Site};

/// build は「生成する」フェーズ：
/// - Markdown -> HTML
/// - template 適用
/// - dist へ書き込み
pub fn build(cfg: &Config, site: &Site, clean: bool) -> Result<()> {
    if clean && cfg.output_dir.exists() {
        fs::remove_dir_all(&cfg.output_dir)?;
    }
    fs::create_dir_all(&cfg.output_dir)?;

    // templates/** を全部読む（例: templates/page.html）
    let glob = format!("{}/**/*", cfg.templates_dir.display());
    let tera = Tera::new(&glob)?;

    // 静的ファイルがあるなら templates_dir/../static みたいに分けてもいいが、ここは最小。
    for page in &site.pages {
        render_page(&tera, page, cfg)?;
    }

    Ok(())
}

fn render_page(tera: &Tera, page: &Page, cfg: &Config) -> Result<()> {
    let body_html = markdown_to_html(&page.markdown);

    let mut ctx = Context::new();
    ctx.insert("title", &page.title);
    ctx.insert("content", &body_html);

    // base_url がある場合だけ使えるようにしておく
    if let Some(base_url) = &cfg.base_url {
        ctx.insert("base_url", base_url);
    }

    // templates/page.html を想定（存在しないとエラー）
    let rendered = tera.render("page.html", &ctx)?;

    if let Some(parent) = page.output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&page.output_path, rendered)?;

    Ok(())
}

fn markdown_to_html(md: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(md, options);
    let mut out = String::new();
    html::push_html(&mut out, parser);
    out
}
