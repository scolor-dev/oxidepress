## 概要 / Summary
<!-- JP: 既存挙動の改善内容。 -->
<!-- EN: What existing behavior was improved. -->

## 現状の課題 / Current Limitation
<!-- JP: 現在の不便さ・弱点。 -->
<!-- EN: Current pain point. -->

## 改善内容 / Improvement
<!-- JP: 何をどう改善したか。 -->
<!-- EN: What changed and why this is better. -->

## 影響範囲 / Affected Scope
- [ ] CLI (`src/cli.rs`)
- [ ] ビルド処理 / Build pipeline (`src/build.rs`)
- [ ] 開発サーバー / Dev server (`src/serve.rs`)
- [ ] サイト/ページ解析 / Site page parsing (`src/site/*`)
- [ ] 設定 / Config (`src/config.rs`)
- [ ] ドキュメント・テンプレート・コンテンツ / Docs templates content

## 確認項目 / Validation
- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test`
- [ ] 改善前後の挙動確認 / Behavior comparison performed

## 関連Issue / Related Issues
- Closes #
