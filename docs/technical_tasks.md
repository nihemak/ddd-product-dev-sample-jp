# 技術タスクバックログ

このファイルは、ユーザーストーリーに直接紐付かない技術的な改善タスク（環境整備、リファクタリング、依存関係更新、技術的負債返済など）を管理するためのバックログです。

## アーキテクチャ・技術選定 (Decision & Design)

*   [x] docs(architecture): サービス提供形態を決定する (Web, Mobile, etc.) #decision
*   [x] docs(architecture): システム構成を決定する (SPA, SSR, etc.) #decision
*   [x] docs(architecture): データストアを選定する (RDB, NoSQL, etc.) #decision
*   [x] docs(architecture): Webフロントエンド技術を選定する (React, Rust(SSR/WASM), etc.) #decision #frontend
*   [x] docs(architecture): プロジェクト構成を決定する (モノレポ or Not) #decision
*   [x] docs(architecture): デプロイ戦略・プラットフォームを選定する (GCP, AWS, Cloud Run, Lambda, etc.) #decision #infra
*   [ ] docs(architecture): ブランチ戦略を定義する #decision #process
*   [ ] docs(architecture): 環境定義 (本番, Staging, Test) #decision #infra
*   [x] docs(architecture): API設計方針を決定する (REST, GraphQL, etc.) #decision
*   [x] docs(architecture): 認証・認可方式を決定する #decision #security
*   [ ] docs(architecture): ログ・監視戦略を決定する #decision #infra
*   [ ] docs(architecture): Webアナリティクス導入を検討・決定する #decision #monitoring
*   [x] docs(architecture): APIスキーマ定義・管理方法を決定する (OpenAPI?) #decision #api
*   [ ] refactor(server): Axum の State 管理方法を改善する (AppState 構造体の導入検討) #architecture #backend
*   [ ] feat(openapi): /api/health 以外のエンドポイントのスキーマ定義を OpenAPI 仕様に追加する #api #backend

## 環境構築・CI/CD (Setup & Infra)

*   [ ] docs(devops): CI/CDパイプラインを構築する #ci-cd
*   [x] chore(devops): フロントエンド開発環境を構築する #frontend #dev-env
*   [x] chore(devops): バックエンド開発環境を Docker Compose で構築する (Rust + PostgreSQL) #backend #dev-env #docker
*   [ ] chore(infra): クラウド環境 (Render, Vercel) を構築・設定する #infra
*   [ ] chore(security): Auth0の初期設定を行う #security
*   [x] chore(docker): Dockerfile のビルドキャッシュ効率とイメージサイズを最適化する #docker #performance
*   [ ] chore(devops): sqlx-cli 実行時の DATABASE_URL 指定方法を改善する (ラッパースクリプト等) #dev-env #db
*   [x] chore(ci): GitHub Actions のキャッシュを設定してビルド時間を短縮する #ci-cd #performance
*   [x] chore(ci): backend ディレクトリ配下の変更時のみ CI を実行するようトリガーを最適化する #ci-cd #monorepo
*   [ ] chore(devops): コンテナ起動時にDBマイグレーションを自動実行する仕組みを検討・導入する #dev-env #db #docker

## 開発プロセス・ドキュメント (Process & Docs)

*   [x] docs(process): 画面仕様の定義・管理方法を決定する (Storybook+MD) #process #documentation
*   [x] docs(frontend): UIコンポーネントライブラリを選定する #frontend #decision
*   [ ] docs(ui): 主要画面の仕様（画面定義・遷移）をドキュメント化する (Storybook+MD) #documentation #ui
*   [x] docs(process): 開発サイクルの計画・管理方法を定義する (イテレーション計画ドキュメント?) #process
*   [x] docs(process): テーブル設計（データモデリング）の記録方法を決定する #process
*   [ ] docs(testing): フロントエンドのユニットテスト戦略・ツールを決定する (Jest/RTL?) #testing #frontend #process
*   [ ] docs(testing): フロントエンドのE2Eテスト戦略・ツールを選定する (Playwright/Cypress?) #testing #frontend #process
*   [ ] docs(testing): バックエンドの結合テスト戦略を決定する #testing #backend #process
*   [x] docs(adr): ADR 0001 (オニオンアーキテクチャ) の内容を現状に合わせて見直す #documentation
*   [x] docs(adr): ADR 0002 (mockall採用) の内容を現状に合わせて見直す #documentation
*   [ ] chore(process): プロダクト戦略レビューを実施する (次回目安: MVPリリース後) #process #strategy
*   [x] docs: プロジェクトのセットアップ手順をREADMEに記述する #documentation
*   [ ] docs(process): CONTRIBUTING.md を作成し、コミット規約や開発フローの詳細を記述する #documentation #process

## 実装・リファクタリング (Implementation)

*   [x] refactor(domain): 状態を型で表現するアプローチを採用し、プレゼント予約ドメインに適用する #refactoring #architecture
*   [ ] refactor(domain): 状態を型で表現するアプローチを採用し、支払いドメインに適用する #refactoring #architecture
*   [x] refactor(sample): サンプル実装コードをプロダクト定義に合わせて修正・削除する #implementation #refactoring
*   [x] refactor(infra): InMemoryRepository を PgRepository に置き換える #implementation #db #backend
*   [ ] refactor(error): expect() の使用箇所を見直し、適切なエラーハンドリングに改善する #refactoring #backend #quality
*   [ ] chore(lint): プロジェクト全体の警告（未使用import等）を修正する (`cargo fix`, `cargo clippy`) #quality #backend
*   [x] refactor(application): ok_or_else を ok_or に修正する (clippy::unnecessary_lazy_evaluations) #tech-debt #quality
*   [ ] refactor(domain): 各ID型に Default トレイトを実装する (clippy::new_without_default) #tech-debt #quality
*   [x] refactor(main): #![allow(clippy::single_component_path_imports)] を削除し use 文を整理 #tech-debt #quality
*   [ ] refactor(functions): 引数が多い関数をリファクタリングする (コマンドオブジェクト等, clippy::too_many_arguments) #tech-debt #architecture
*   [ ] refactor(infra): PgRepository の DB エラーマッピングを改善する (sqlx::Error -> DomainError/InfrastructureError) #tech-debt #quality #backend
*   [ ] test(infra): PgRepository のテストケースを拡充する (異常系、境界値など) #testing #quality #backend
*   [ ] chore(deps): バックエンドの依存クレート(Rust)のバージョンを定期的に確認・更新する #tech-debt #quality #backend
*   [ ] chore(deps): フロントエンドの依存パッケージ(npm)のバージョンを定期的に確認・更新する #tech-debt #quality #frontend
*   [ ] chore(deps): 依存クレートのバージョンを定期的に確認・更新する #tech-debt #quality
*   [ ] chore(ci): "potentially unused queries" 警告の原因を調査し修正する #tech-debt #quality #ci

## いつかやる (優先度 低)

*   [ ] chore: リポジトリ全体のlintルールを最新化する #tech-debt
*   [ ] docs: ADRテンプレートを導入する #documentation
*   [ ] chore(infra): 基本的なサーバー監視を設定する (Render/Vercel標準機能) #monitoring #infra
*   [ ] refactor(db): イミュータブルデータモデリング（履歴テーブル等）の導入を検討・実施する (Ref: ADR 0018) #tech-debt #architecture
*   [ ] chore(frontend): Shadcn/ui (Tailwind CSS, Radix UI) をセットアップする (React 18を使用中) #frontend #dev-env #ui
*   [ ] chore(frontend): Shadcn/ui が React 19 に対応したら React を 19 にアップグレードする #frontend #deps #tech-debt 