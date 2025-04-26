# ADR 0017: Web フレームワークを Axum に変更

*   **Status**: Accepted
*   **Date**: 2025-04-26
*   **Deciders**: (あなたと私)

## Context and Problem Statement

開発初期段階で `lib.rs` に Actix-Web ベースのコード (テスト用 `run`/`spawn_app` 関数など) が存在した一方、`main.rs` では Axum を利用して API サーバーの実装を開始していた。これにより、プロジェクト内で Web フレームワークが混在し、統一性がなくメンテナンス性が低い状態になっていた。

## Decision Drivers

*   **統一性**: プロジェクト内で使用する Web フレームワークを一つに絞る。
*   **メンテナンス性**: コードベースをシンプルに保ち、将来的な改修を容易にする。
*   **エコシステム**: `tokio` および `tower` エコシステムとの親和性が高いフレームワークを選択する。
*   **モダン性**: 現在活発に開発されているフレームワークを選択する。

## Considered Options

1.  **Axum に統一**: `main.rs` での実装に合わせ、`lib.rs` およびテストコードから Actix-Web 依存を排除する。
2.  **Actix-Web に統一**: `lib.rs` の実装に合わせ、`main.rs` を Actix-Web ベースに書き直す。

## Decision Outcome

**選択肢1「Axum に統一」を採用する。**

理由:
*   `main.rs` で既に Axum ベースの実装が進んでいた。
*   Axum は `tokio` 上に構築され、`tower` ミドルウェアエコシステムを直接活用できるため、非同期 Rust プロジェクトとの親和性が高い。
*   コミュニティでの採用例も増えており、モダンな選択肢である。

**具体的な変更:**
*   `lib.rs` から Actix-Web 関連のコード (`run`, `spawn_app`, `HttpServer`, `App` など) と依存関係を削除。
*   `tests/health_check.rs` を修正し、テスト内で Axum アプリケーションを起動するように変更。
*   `Cargo.toml` に `axum`, `tower-http` などの依存関係を追加。
*   `main.rs` のサーバー起動部分を `axum::serve` を使うように修正。

## Consequences

### Positive:
*   Web フレームワークが Axum に統一され、コードベースの整合性が向上した。
*   `tower` ミドルウェアを容易に利用できるようになった。

### Negative:
*   Actix-Web ベースで実装されていた OpenAPI/Swagger UI 機能 (`utoipa`, `utoipa-swagger-ui`) を Axum ベースで再実装・再設定する必要がある。 (`utoipa` は Axum もサポートしているため、設定変更で対応可能。)
*   `README.md` などのドキュメントで Actix-Web に言及している箇所を修正する必要がある。

## References

*   [Axum GitHub Repository](https://github.com/tokio-rs/axum)
*   `main.rs`, `lib.rs`, `tests/health_check.rs`
*   `Cargo.toml`
*   (関連 ADR があれば) 