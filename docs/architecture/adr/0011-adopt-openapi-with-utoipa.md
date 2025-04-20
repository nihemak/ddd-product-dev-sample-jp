# ADR 0011: APIスキーマ定義・管理に OpenAPI (utoipa) を採用

*   **Status**: Accepted
*   **Date**: 2025-04-20
*   **Deciders**: (議論参加者)

## Context and Problem Statement

REST API (ADR 0010) の仕様を明確に定義し、管理する必要がある。これにより、フロントエンドとバックエンド間の連携を円滑にし、APIドキュメントを生成し、ツールの支援を受けられるようにしたい。API仕様の定義・管理方法を決定する必要がある。

## Decision Drivers

*   **仕様の明確性**: APIの契約（エンドポイント、パラメータ、リクエスト/レスポンス形式など）を unambiguous に定義できること。
*   **ドキュメンテーション**: API仕様から人間が読めるドキュメントを自動生成できること。
*   **ツールエコシステム**: コード生成、テスト、バリデーションなどのツールが利用できること。
*   **保守性**: 仕様と実装コードの同期を容易に保てること。
*   **業界標準**: 広く使われている標準的な方法を採用すること。

## Considered Options

1.  **OpenAPI Specification (OAS)**: REST API記述のための業界標準仕様 (YAML/JSON)。
    *   **サブオプション 1a: Spec-First**: OAS定義ファイルを手動で作成・管理し、それに基づいて実装する。
        *   Pros: 設計を最初に確定できる。
        *   Cons: 実装と仕様の同期を手動で保つ必要があり、乖離しやすい。
    *   **サブオプション 1b: Code-First (with `utoipa`)**: Rustコード（型定義、ハンドラ）にアノテーション等を追加し、`utoipa` クレートを使ってOAS定義ファイルを自動生成する。
        *   Pros: コードと仕様の同期を保ちやすい、Rustの型システムを活用できる、ドキュメント生成 (Swagger UI統合) が容易。
        *   Cons: `utoipa` クレートへの依存、アノテーション等の学習コスト。
2.  **API Blueprint / RAML**: OpenAPIと同様の目的を持つ代替仕様。
    *   Cons: OpenAPIほどの普及度やツールサポートがない。
3.  **手書きドキュメント (Markdownなど)**: 独自フォーマットでAPI仕様を記述する。
    *   Cons: 標準化されておらず、ツールの恩恵を受けられない、保守性が低い。

## Decision Outcome

**APIスキーマ定義・管理方法として、OpenAPI Specification を Code-First アプローチ（`utoipa` クレート利用）で採用する。**

理由:
*   **OpenAPIの標準性**: OpenAPIはREST API仕様記述のデファクトスタンダードであり、豊富なツールエコシステムの恩恵を最大限に受けられる。
*   **Code-First (`utoipa`) のメリット**: `utoipa` を利用することで、Rustのコード（特に型定義）を信頼できる情報源 (Single Source of Truth) とし、そこからOpenAPI仕様 (YAML/JSON) とインタラクティブなドキュメント (Swagger UI) を自動生成できる。これにより、仕様と実装の同期を保つという重要な課題を効率的に解決できる。
*   **開発者体験**: コード内で仕様を定義できるため、開発フローがスムーズになる。
*   **Rustエコシステムとの親和性**: `utoipa` はActix WebやAxumといった主要なRust Webフレームワークとの連携をサポートしている。

## Consequences

### Positive:
*   API仕様がコードと同期され、常に最新の状態に保たれやすい。
*   インタラクティブなAPIドキュメント (Swagger UI) を容易に生成・公開できる。
*   OpenAPIの豊富なツール（クライアント生成、テストなど）を活用できる可能性がある。
*   Rustの型システムをAPI仕様定義に活かせる。

### Negative:
*   `utoipa` クレートとその使い方（マクロ、アノテーション）を学習する必要がある。
*   `utoipa` クレートへの依存が発生する。
*   複雑なAPI仕様の場合、アノテーションが冗長になる可能性がある。

## References

*   [OpenAPI Specification](https://swagger.io/specification/)
*   [utoipa - crates.io](https://crates.io/crates/utoipa)
*   ADR 0010: API設計方針として REST を採用 