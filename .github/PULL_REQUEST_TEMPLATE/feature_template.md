## 概要 / Summary
<!-- JP: 追加した機能を説明。 -->
<!-- EN: Describe the new capability. -->

## 課題・目的 / Problem Goal
<!-- JP: 何を解決する機能か。 -->
<!-- EN: What problem/use case this feature addresses. -->

## 設計メモ / Design Notes
<!-- JP: API/CLIの判断、入出力、ルーティングなど。 -->
<!-- EN: API CLI choices, data flow, routing or output behavior. -->

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
- [ ] 手動シナリオ確認 / Manual scenario verified

手動シナリオ / Manual scenario:
1.
2.

## 関連Issue / Related Issues
- Closes #
