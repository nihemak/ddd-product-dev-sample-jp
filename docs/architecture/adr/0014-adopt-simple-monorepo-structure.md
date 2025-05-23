# ADR 0014: シンプルなモノレポ構成を採用

*   **Status**: Accepted
*   **Date**: 2025-04-20
*   **Deciders**: (議論参加者)

## Context and Problem Statement

プロジェクトにはバックエンド (Rust API)、フロントエンド (Next.js SPA)、ドキュメントが含まれる。これらを単一の Git リポジトリで管理する（モノレポ）ことを前提とし、その内部構成方法を決定する必要がある。専用のモノレポ管理ツール（Turborepo, Nx など）を利用するか、シンプルなディレクトリ構造にするかを検討する。

## Decision Drivers

*   **シンプルさ・学習コスト**: 構成が理解しやすく、追加ツールの学習コストが低いこと。
*   **管理容易性**: プロジェクト全体の管理がしやすいこと。
*   **ツール連携**: OpenAPI 仕様生成 (バックエンド) とクライアントコード生成 (フロントエンド) の連携が容易であること。
*   **拡張性**: 将来的に必要になった場合に、モノレポ管理ツール等を導入できること。
*   **サンプルとしての明快さ**: 各技術要素の構成が分かりやすいこと。

## Considered Options

1.  **シンプルなディレクトリ構成**: ルート直下に役割ごとのディレクトリ (`backend/`, `frontend/`, `docs/` など) を配置する。
    *   Pros: 非常にシンプルで理解しやすい、追加のツール不要、OpenAPIファイル等の相互参照が容易（相対パス）。
    *   Cons: プロジェクト間の共通コード共有の仕組みがない、ビルド/テストの横断実行には工夫が必要。
2.  **モノレポ管理ツール (Turborepo, Nx) の利用**: 専用ツールで依存関係管理、キャッシュ、タスク実行を行う。
    *   Pros: 効率的なビルド/テスト、依存関係管理の強化、共通パッケージ管理が容易。
    *   Cons: ツールの学習コスト、設定の複雑化、Rustプロジェクトとの連携に工夫が必要な場合がある。

## Decision Outcome

**選択肢1「シンプルなディレクトリ構成」を採用する。**

理由:
*   **シンプルさ優先**: 現時点ではモノレポ管理ツールの高度な機能（キャッシュ最適化、複雑な依存関係管理）は不要であり、ツールの学習・設定コストを避けることを優先する。
*   **サンプルとしての分かりやすさ**: 各パート（バックエンド、フロントエンド）の構成が独立して理解しやすい。
*   **OpenAPI連携の容易さ**: バックエンドで生成されたOpenAPI仕様ファイルを、フロントエンドから相対パスで容易に参照でき、クライアントコード生成等に活用しやすい。
*   **段階的導入の可能性**: 将来的にプロジェクトが複雑化した場合に、Turborepoなどを後から導入することは可能である。

**具体的な構成イメージ:**
```
/
├── backend/      # Rustバックエンド
│   ├── src/
│   ├── openapi.yaml  # utoipaで生成 (例)
│   └── Cargo.toml
├── frontend/     # Next.js フロントエンド
│   ├── app/
│   ├── components/
│   ├── src/api/ # openapi-typescript-codegenで生成 (例)
│   ├── .storybook/
│   ├── src/stories/
│   └── package.json
├── docs/
└── ... (その他設定ファイル)
```

## Consequences

### Positive:
*   プロジェクト構成が非常にシンプルで理解しやすい。
*   追加ツールの学習や設定が不要。
*   バックエンドとフロントエンド間のファイル参照（例: OpenAPI仕様）が容易。
*   各パートを独立して扱いやすい。

### Negative:
*   `backend` と `frontend` で共通化したいコード（例: 一部の型定義）が出てきた場合に、共有する仕組みを別途検討する必要がある（例: `common/` ディレクトリを作成するなど）。
*   複数のパッケージを横断するビルドやテストの実行には、カスタムスクリプトなどが必要になる可能性がある。

## References

*   ADR 0005: システム構成としてSPA + APIアーキテクチャを採用
*   ADR 0011: APIスキーマ定義・管理に OpenAPI (utoipa) を採用 