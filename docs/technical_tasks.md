# 技術タスクバックログ

このファイルは、ユーザーストーリーに直接紐付かない技術的な改善タスク（環境整備、リファクタリング、依存関係更新、技術的負債返済など）を管理するためのバックログです。

## アーキテクチャ・技術選定 (Decision & Design)

*   [x] docs(architecture): サービス提供形態を決定する (Web, Mobile, etc.) #decision
*   [x] docs(architecture): システム構成を決定する (SPA, SSR, etc.) #decision
*   [ ] docs(architecture): データストアを選定する (RDB, NoSQL, etc.) #decision
*   [ ] docs(architecture): Webフロントエンド技術を選定する (React, Rust(SSR/WASM), etc.) #decision #frontend
*   [ ] docs(architecture): プロジェクト構成を決定する (モノレポ or Not) #decision
*   [ ] docs(architecture): デプロイ戦略・プラットフォームを選定する (GCP, AWS, Cloud Run, Lambda, etc.) #decision #infra
*   [ ] docs(architecture): ブランチ戦略を定義する #decision #process
*   [ ] docs(architecture): 環境定義 (本番, Staging, Test) #decision #infra
*   [ ] docs(architecture): API設計方針を決定する (REST, GraphQL, etc.) #decision
*   [ ] docs(architecture): 認証・認可方式を決定する #decision #security
*   [ ] docs(architecture): ログ・監視戦略を決定する #decision #infra
*   [ ] docs(architecture): Webアナリティクス導入を検討・決定する #decision #monitoring
*   [ ] docs(architecture): APIスキーマ定義・管理方法を決定する (OpenAPI?) #decision #api

## 環境構築・CI/CD (Setup & Infra)

*   [ ] chore(dev-env): ローカル開発環境を整備する (Docker?) #setup
*   [ ] chore(ci-cd): CI/CDパイプラインを構築する (GitHub Actions?) #setup
*   [ ] chore(iac): Infrastructure as Code を導入する (Terraform?) #setup #infra

## 開発プロセス・ドキュメント (Process & Docs)

*   [ ] docs(process): 画面仕様の定義・管理方法を決定する (デザインシステム?) #process
*   [ ] docs(process): 開発サイクルの計画・管理方法を定義する (イテレーション計画ドキュメント?) #process
*   [ ] docs(process): テーブル設計（データモデリング）の記録方法を決定する #process
*   [ ] docs(adr): ADR 001 (オニオンアーキテクチャ) の内容を現状に合わせて見直す #documentation
*   [ ] docs(adr): ADR 002 (mockall採用) の内容を現状に合わせて見直す #documentation

## 実装・リファクタリング (Implementation)

*   [x] refactor(domain): 状態を型で表現するアプローチを採用し、プレゼント予約ドメインに適用する #refactoring #architecture
*   [ ] refactor(domain): 状態を型で表現するアプローチを採用し、支払いドメインに適用する #refactoring #architecture
*   [ ] refactor(sample): サンプル実装コードをプロダクト定義に合わせて修正・削除する #implementation #refactoring

## いつかやる (優先度 低)

*   [ ] chore: リポジトリ全体のlintルールを最新化する #tech-debt
*   [ ] docs: ADRテンプレートを導入する #documentation 