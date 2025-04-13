# DDDプロダクト開発 サンプル (Rustバックエンド, 日本語UL)

これは、ドメイン駆動設計 (DDD) の原則と軽量なプロダクト開発プロセスを組み合わせたサンプルアプリケーションです。
バックエンドはRustで実装され、以下の特徴を持っています：

*   **関数型スタイル**: ドメインロジックは副作用を極力排した関数として実装されています。
*   **Railway Oriented Programming (ROP)**: `Result` 型と `and_then`, `map_err` などを活用し、エラー処理を含むワークフローを直線的に記述しています。
*   **オニオンアーキテクチャ**: 関心事を Domain, Application, Infrastructure の層に分離しています。
    *   `src/main.rs` 内に `mod domain`, `mod application`, `mod infrastructure` として定義されています。
*   **日本語ユビキタス言語**: ドメイン層の型名や関数名には、日本語（またはそのローマ字表記）を積極的に採用しています（例: `注文`, `商品ID`, `mark_as_preparing`）。
*   **依存性の注入 (DI)**: Application層はRepositoryのインターフェース（トレイト）に依存し、具体的な実装は `main` 関数で注入されます（ここではインメモリ実装を使用）。

## 主な概念

*   **ドメイン (Domain)**: ビジネスルールとロジックの中核。値オブジェクト (`商品ID`, `注文ID`)、エンティティ (`注文`, `商品`)、ドメインエラー (`DomainError`)、ドメインロジック関数、リポジトリインターフェース（トレイト）が含まれます。
*   **アプリケーション (Application)**: ユースケース（ワークフロー）を実装。ドメイン層のオブジェクトや関数をオーケストレーションし、リポジトリを通じて永続化などを扱います。アプリケーション固有のエラー (`ApplicationError`) も定義します。
*   **インフラストラクチャ (Infrastructure)**: データベースアクセス、外部API連携など、技術的な詳細を実装。ここではインメモリのダミーリポジトリ (`InMemory注文Repository`, `InMemory商品Repository`) を提供します。

## アーキテクチャ

このサンプルではオニオンアーキテクチャを採用しています。
詳細については、以下のドキュメントを参照してください。

*   **アーキテクチャ概要**: [`docs/architecture/overview.md`](docs/architecture/overview.md)
*   **ドメインモデル**: [`docs/domain/domain-model.md`](docs/domain/domain-model.md) (Mermaid形式)
*   **ユビキタス言語**: [`docs/domain/ubiquitous-language.md`](docs/domain/ubiquitous-language.md)

### 主要ドキュメント

*   [プロダクト開発ガイドライン (Product Development Guide)](docs/PRODUCT_DEVELOPMENT_GUIDE.md) ✨ **まずはこちらをお読みください**
*   [プロダクト定義 (Product Definition)](docs/product/) - 主要ファイル: [ビジョン＆戦略](docs/product/vision_strategy.md), [ロードマップ](docs/product/roadmap.md)
*   [要求/仕様 (Requirements)](docs/requirements/) - 具体例: [ユーザーストーリーマップ](docs/requirements/user_story_mapping.md)
*   [ドメイン関連 (Domain)](docs/domain/) - 主要ファイル: [ユビキタス言語](docs/domain/ubiquitous-language.md), [モデル図](docs/domain/domain-model.md)
*   [アーキテクチャ関連 (Architecture)](docs/architecture/) - 主要ファイル: [概要](docs/architecture/overview.md), [ADR](docs/architecture/adr/)

## 実行方法

1.  Rustのツールチェインがインストールされていることを確認してください。
2.  プロジェクトのルートディレクトリで以下のコマンドを実行します。

    ```bash
    cargo run
    ```

これにより、依存関係がダウンロード・コンパイルされ、`src/main.rs` 内のサンプルコードが実行されます。
コンソールに注文の作成、状態遷移、エラーハンドリングの過程が出力されます。

## テスト

このプロジェクトにはユニットテストが含まれています。各モジュールファイル (`src/domain.rs`, `src/application.rs`) 内にテストコード (`#[cfg(test)] mod tests { ... }`) が記述されています。

テストの実行は以下のコマンドで行います。

```bash
cargo test
```

*   **Domain層のテスト**: ドメインロジック関数の純粋性を検証します。
*   **Application層のテスト**: `mockall` クレートを使用してリポジトリインターフェース（トレイト）をモック化し、ユースケース（`注文サービス`）が期待通りにドメインロジックやリポジトリメソッドを呼び出すかを検証します。

## 構成

```
.
├── Cargo.toml          # プロジェクト定義と依存関係
├── Cargo.lock          # 依存関係のロックファイル
├── .gitignore          # Gitで無視するファイル
├── docs/               # ドキュメントルート
│   ├── product/        # プロダクト定義 (ビジョン, 戦略, ロードマップ, ペルソナ)
│   ├── requirements/   # 要求/仕様
│   ├── domain/         # ドメインモデリング関連 (ユビキタス言語, モデル図)
│   └── architecture/   # アーキテクチャ設計 (概要, ADR)
├── src/
│   ├── main.rs         # アプリケーションのエントリーポイント (バイナリクレート)
│   ├── lib.rs          # ライブラリクレートのエントリーポイント、モジュール宣言
│   ├── domain.rs       # Domain層のコードとテスト
│   ├── application.rs  # Application層のコードとテスト
│   └── infrastructure.rs # Infrastructure層のコード
└── target/             # (Git無視) ビルド成果物
```

## 目的

このサンプルプロジェクトは、以下の複数の目的を持っています。

1.  **Rustによる実践的なDDDの実装例:**
    *   Rustを用いてドメイン駆動設計 (DDD) の原則（特に日本語ユビキタス言語、オニオンアーキテクチャ）を適用する具体的な方法を示すこと。
    *   関数型プログラミングのスタイル（純粋関数中心のロジック、Railway Oriented Programmingによるエラー処理）をRustで実践する例を示すこと。

2.  **軽量なプロダクト開発プロセスの提示:**
    *   プロダクトの「Why」（ビジョン、戦略）から「What」（要求定義）、そして「How」（設計、実装）までを、ドキュメント（Markdown, Mermaid等）を活用して一貫して繋げる、軽量な開発プロセスの一例を示すこと。
    *   DDDのプラクティス（ユビキタス言語、ドメインモデリング）を、技術実装だけでなく要求定義やプロダクト定義の段階から活用するアプローチを示すこと。

これらの目的を通じて、Rustでのアプリケーション開発やDDDの実践、あるいは軽量なプロダクト開発プロセスの導入を検討する際の、具体的な出発点や学習リソースとなることを目指しています。 