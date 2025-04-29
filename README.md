# DDDプロダクト開発 サンプル (Rustバックエンド & Next.jsフロントエンド, 日本語UL)

これは、ドメイン駆動設計 (DDD) の原則と軽量なプロダクト開発プロセスを組み合わせたサンプルアプリケーションです。
**具体的には、「記念日プレゼント予約・配送サービス」を題材としています。**
バックエンド (`backend/`) はRustで実装され、**フロントエンド (`frontend/`) は Next.js (React + TypeScript) で実装されています。**
開発環境は Docker Compose で構築されています。

**バックエンドの特徴:**
*   **Webフレームワーク**: `axum` を使用。
*   **非同期ランタイム**: `tokio` を使用。
*   **データベースアクセス**: `sqlx` を使用 (PostgreSQL)。
*   **APIドキュメント**: `utoipa` を使用して OpenAPI 仕様を生成し、Swagger UI で表示。
*   **関数型スタイル**: ドメインロジックは副作用を極力排した関数として実装。
*   **Railway Oriented Programming (ROP)**: `Result` 型を活用。
*   **オニオンアーキテクチャ**: Domain, Application, Infrastructure の層分離。
*   **日本語ユビキタス言語**: ドメイン層の命名に日本語を採用。
*   **依存性の注入 (DI)**: `main` 関数でリポジトリ等を注入。

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
*   [開発プロセス (Process)](docs/process/) - 主要ファイル: [イテレーション計画](docs/process/iteration_planning.md)

## 環境設定 ✨

ローカルでの開発やテスト実行のために、環境変数の設定が必要です。

1.  **`.env` ファイルの作成:**
    プロジェクトルートに `.env` という名前のファイルを作成します。
    このファイルは `.gitignore` に記載されているため、Gitリポジトリにはコミットされません。

2.  **`DATABASE_URL` の設定:**
    `.env` ファイル内に、PostgreSQL データベースへの接続文字列を `DATABASE_URL` として設定します。
    `docker-compose.yml` を使用してデータベースを起動する場合、ホストマシンからアクセスするための URL は通常以下のようになります。

    ```env
    # .env ファイルの内容例
    DATABASE_URL=postgres://app_user:password123@localhost:5432/app_db
    ```

    ユーザー名 (`app_user`)、パスワード (`password123`)、ホスト名 (`localhost`)、ポート (`5432`)、データベース名 (`app_db`) は、`docker-compose.yml` の `db` サービスの設定に合わせてください。

3.  **`sqlx` のオフライン準備 (任意ですが推奨):**
    `sqlx` はコンパイル時にデータベースに接続してクエリを検証します。コンパイル時に毎回データベース接続を行わないように、以下のコマンドを実行してクエリ情報をファイル (`sqlx-data.json`) に保存できます。

    ```bash
    # backend ディレクトリに移動
    cd backend
    # DATABASE_URL を環境変数として設定して実行
    DATABASE_URL="postgres://app_user:password123@localhost:5432/app_db" cargo sqlx prepare --merged
    # 元のディレクトリに戻る
    cd ..
    ```
    `sqlx-data.json` が生成されると、コンパイル時に `DATABASE_URL` 環境変数がなくても `sqlx::query!` マクロのエラーが発生しなくなります。

## 実行方法 (Docker Compose)

1.  Docker および Docker Compose がインストールされていることを確認してください。
2.  プロジェクトのルートディレクトリで環境変数ファイルを作成します。
    ```bash
    cp .env.sample .env
    # .env ファイル内の DATABASE_URL などを必要に応じて編集します。
    ```
3.  以下のコマンドでコンテナをビルドしてバックグラウンドで起動します。
    ```bash
    docker compose up -d --build
    ```
4.  各サービスは以下のURLで利用可能になります:
    *   **フロントエンド:** `http://localhost:3000`
    *   **バックエンドAPIドキュメント (Swagger UI):** `http://localhost:8080/swagger-ui/`

*   **ログの確認:**
    *   フロントエンド: `docker compose logs -f frontend`
    *   バックエンド: `docker compose logs -f backend`
    *   データベース: `docker compose logs -f db`
*   **コンテナの停止:** `docker compose down`
*   **開発時のホットリロード:**
    *   バックエンド: `backend/src` 以下のコードを変更すると自動で再ビルド・再起動されます。
    *   フロントエンド: `frontend/src` 以下のコードを変更すると自動で反映されます (HMR)。

## テスト

### 単体・結合テスト (バックエンド)

`cargo test` を使用して、ユニットテスト（主にドメイン層）と結合テスト（アプリケーション層・インフラ層）を実行します。

```bash
# プロジェクトルートから実行
docker compose exec backend cargo test
# または、バックエンドディレクトリ内で直接実行
# cd backend
# cargo test
```

*   **Domain層**: ドメインロジックの純粋性を検証。
*   **Application層**: `mockall` を使用して依存性をモック化し、ユースケースを検証。
*   **Infrastructure層**: 実際のDBコンテナに接続してリポジトリ実装を検証するテストも含まれる場合があります。

### API エンドポイントテスト

`backend/tests/` ディレクトリ (今後作成予定) にて、`reqwest` クレートなどを用いて実際のAPIエンドポイントを叩くテストを実装します。これらのテストも `cargo test` で実行されます。

## 構成

```
.
├── .env.sample         # 環境変数サンプル
├── .gitignore          # Gitで無視するファイル
├── docker-compose.yml  # Docker Compose 設定
├── backend/            # バックエンド Rust プロジェクト
│   ├── Cargo.toml      # プロジェクト定義と依存関係
│   ├── Cargo.lock      # 依存関係のロックファイル
│   ├── Dockerfile      # バックエンド用 Dockerfile
│   └── src/
│       ├── lib.rs      # ライブラリクレートのエントリ、モジュール宣言
│       ├── main.rs     # アプリケーションのエントリ、DIコンテナ、サーバー起動、OpenAPI定義
│       ├── domain.rs   # Domain層
│       ├── application.rs # Application層
│       ├── infrastructure.rs # Infrastructure層
│       └── routes/     # (Axum 用に再構成予定) APIルートハンドラ
├── frontend/           # フロントエンド Next.js プロジェクト
│   ├── Dockerfile      # フロントエンド用 Dockerfile
│   ├── package.json    # 依存関係、スクリプト等
│   └── src/            # ソースコード (App Router ベース)
├── docs/               # ドキュメントルート
│   ├── product/        # プロダクト定義
│   ├── requirements/   # 要求/仕様
│   ├── domain/         # ドメイン関連
│   ├── architecture/   # アーキテクチャ関連 (ADR含む)
│   └── process/        # 開発プロセス関連
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