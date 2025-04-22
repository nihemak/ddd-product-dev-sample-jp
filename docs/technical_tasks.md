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

## 環境構築・CI/CD (Setup & Infra)

*   [ ] docs(devops): CI/CDパイプラインを構築する #ci-cd
*   [ ] chore(devops): フロントエンド開発環境を構築する #frontend #dev-env
*   [ ] chore(frontend): Shadcn/ui (Tailwind CSS, Radix UI) をセットアップする #frontend #dev-env #ui
*   [ ] chore(storybook): Storybook をセットアップする #frontend #dev-env #documentation
*   [x] chore(devops): バックエンド開発環境を Docker Compose で構築する (Rust + PostgreSQL) #backend #dev-env #docker
*   [ ] chore(infra): クラウド環境 (Render, Vercel) を構築・設定する #infra
*   [ ] chore(security): Auth0の初期設定を行う #security

## 開発プロセス・ドキュメント (Process & Docs)

*   [x] docs(process): 画面仕様の定義・管理方法を決定する (Storybook+MD) #process #documentation
*   [x] docs(frontend): UIコンポーネントライブラリを選定する #frontend #decision
*   [ ] docs(ui): 主要画面の仕様（画面定義・遷移）をドキュメント化する (Storybook+MD) #documentation #ui
*   [x] docs(process): 開発サイクルの計画・管理方法を定義する (イテレーション計画ドキュメント?) #process
*   [ ] docs(process): テーブル設計（データモデリング）の記録方法を決定する #process
*   [ ] docs(testing): フロントエンドのユニットテスト戦略・ツールを決定する (Jest/RTL?) #testing #frontend #process
*   [ ] docs(testing): フロントエンドのE2Eテスト戦略・ツールを選定する (Playwright/Cypress?) #testing #frontend #process
*   [ ] docs(testing): バックエンドの結合テスト戦略を決定する #testing #backend #process
*   [ ] docs(adr): ADR 0001 (オニオンアーキテクチャ) の内容を現状に合わせて見直す #documentation
*   [ ] docs(adr): ADR 0002 (mockall採用) の内容を現状に合わせて見直す #documentation

## 実装・リファクタリング (Implementation)

*   [x] refactor(domain): 状態を型で表現するアプローチを採用し、プレゼント予約ドメインに適用する #refactoring #architecture
*   [ ] refactor(domain): 状態を型で表現するアプローチを採用し、支払いドメインに適用する #refactoring #architecture
*   [ ] refactor(sample): サンプル実装コードをプロダクト定義に合わせて修正・削除する #implementation #refactoring

## いつかやる (優先度 低)

*   [ ] chore: リポジトリ全体のlintルールを最新化する #tech-debt
*   [ ] docs: ADRテンプレートを導入する #documentation
*   [ ] chore(infra): 基本的なサーバー監視を設定する (Render/Vercel標準機能) #monitoring #infra
*   [ ] docs: プロジェクトのセットアップ手順をREADMEに記述する #documentation 