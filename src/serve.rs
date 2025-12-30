use anyhow::{bail, Context};
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::error::Result;

pub fn serve_dist(cfg: &Config, host: &str, port: u16) -> Result<()> {
    if !cfg.output_dir.exists() {
        bail!(
            "output_dir not found: {} (run `oxidepress build` first)",
            cfg.output_dir.display()
        );
    }

    let addr = format!("{host}:{port}");
    let server = tiny_http::Server::http(&addr)
        .map_err(|e| anyhow::anyhow!("failed to bind http server: {addr}: {e}"))?;

    eprintln!("Serving {} at http://{addr}", cfg.output_dir.display());

    for request in server.incoming_requests() {
        let url = request.url();
        let file_path = resolve_path(&cfg.output_dir, url);

        match file_to_response(&file_path) {
            Ok(resp) => {
                let _ = request.respond(resp);
            }
            Err(_) => {
                let resp = tiny_http::Response::from_string("Not Found").with_status_code(404);
                let _ = request.respond(resp);
            }
        }
    }

    Ok(())
}

fn resolve_path(dist: &Path, url: &str) -> PathBuf {
    let mut p = url.trim_start_matches('/').to_string();

    if p.is_empty() {
        return dist.join("index.html");
    }

    if p.ends_with('/') {
        p.push_str("index.html");
        return dist.join(p);
    }

    let candidate_dir_index = dist.join(&p).join("index.html");
    if !p.contains('.') && candidate_dir_index.exists() {
        return candidate_dir_index;
    }

    dist.join(p)
}

fn file_to_response(path: &Path) -> Result<tiny_http::Response<Cursor<Vec<u8>>>> {
    let data = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    let mut resp = tiny_http::Response::from_data(data);

    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
        let ct = match ext {
            "html" => "text/html; charset=utf-8",
            "css" => "text/css; charset=utf-8",
            "js" => "application/javascript; charset=utf-8",
            "json" => "application/json; charset=utf-8",
            "svg" => "image/svg+xml",
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            _ => "application/octet-stream",
        };
        resp = resp.with_header(
            tiny_http::Header::from_bytes("Content-Type", ct).expect("valid header"),
        );
    }

    Ok(resp)
}
