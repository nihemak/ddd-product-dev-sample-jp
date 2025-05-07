# 技術タスクバックログ

このファイルは、ユーザーストーリーに直接紐付かない技術的な改善タスク（環境整備、リファクタリング、依存関係更新、技術的負債返済など）を管理するためのバックログです。

## アーキテクチャ・技術選定 (Decision & Design)

- [x] docs(architecture): サービス提供形態を決定する (Web, Mobile, etc.) #decision
- [x] docs(architecture): システム構成を決定する (SPA, SSR, etc.) #decision
- [x] docs(architecture): データストアを選定する (RDB, NoSQL, etc.) #decision
- [x] docs(architecture): Web フロントエンド技術を選定する (React, Rust(SSR/WASM), etc.) #decision #frontend
- [x] docs(architecture): プロジェクト構成を決定する (モノレポ or Not) #decision
- [x] docs(architecture): デプロイ戦略・プラットフォームを選定する (GCP, AWS, Cloud Run, Lambda, etc.) #decision #infra
- [ ] docs(architecture): ブランチ戦略を定義する #decision #process
- [ ] docs(architecture): 環境定義 (本番, Staging, Test) #decision #infra
- [x] docs(architecture): API 設計方針を決定する (REST, GraphQL, etc.) #decision
- [x] docs(architecture): 認証・認可方式を決定する #decision #security
- [ ] docs(architecture): ログ・監視戦略を決定する #decision #infra
- [ ] docs(architecture): Web アナリティクス導入を検討・決定する #decision #monitoring
- [x] docs(architecture): API スキーマ定義・管理方法を決定する (OpenAPI?) #decision #api
- [ ] refactor(server): Axum の State 管理方法を改善する (AppState 構造体の導入検討) #architecture #backend
- [ ] feat(openapi): /api/health 以外のエンドポイントのスキーマ定義を OpenAPI 仕様に追加する #api #backend

## 環境構築・CI/CD (Setup & Infra)

- [ ] docs(devops): CI/CD パイプラインを構築する #ci-cd
- [x] chore(devops): フロントエンド開発環境を構築する #frontend #dev-env
- [x] chore(devops): バックエンド開発環境を Docker Compose で構築する (Rust + PostgreSQL) #backend #dev-env #docker
- [x] chore(dev-env): 開発環境を Dev Container に移行する #dev-env #docker
- [ ] chore(infra): クラウド環境 (Render, Vercel) を構築・設定する #infra
- [ ] chore(security): Auth0 の初期設定を行う #security
- [x] chore(docker): Dockerfile のビルドキャッシュ効率とイメージサイズを最適化する #docker #performance
- [ ] chore(devops): sqlx-cli 実行時の DATABASE_URL 指定方法を改善する (ラッパースクリプト等) #dev-env #db
- [x] chore(ci): GitHub Actions のキャッシュを設定してビルド時間を短縮する #ci-cd #performance
- [x] chore(ci): backend ディレクトリ配下の変更時のみ CI を実行するようトリガーを最適化する #ci-cd #monorepo
- [ ] chore(devops): コンテナ起動時に DB マイグレーションを自動実行する仕組みを検討・導入する #dev-env #db #docker
- [x] chore(devops): Dev Container 内から Docker Compose を操作可能にする (Docker Socket マウント検討) #dev-env #docker #ux
- [ ] chore(dev-env): Dev Container 内共有ボリューム (`backend_target`, `develop_node_modules`) の権限問題を調査し恒久対策を行う #dev-env #docker #ux #tech-debt
  - **現象**: Dev Container 内で `cargo check`, `rust-analyzer`, または `npm install` 等を実行すると、共有ボリューム (`/workspace/backend/target` や `/workspace/frontend/node_modules`) への書き込み権限エラーが発生する場合がある。
  - **原因(推測)**: ホスト OS (macOS 等) とコンテナ (Linux) 間でのユーザー ID/グループ ID の不一致、または Docker のボリュームマウント時の権限設定。
  - **一時的ワークアラウンド**: エラー発生時に Dev Container 内のターミナルで `sudo chown -R vscode:vscode /workspace/backend/target` や `sudo chown -R vscode:vscode /workspace/frontend/node_modules` を実行する。
  - **恒久対策(検討)**: Dockerfile 内でのユーザー/グループ作成と権限設定、`docker-compose.yml` の `user` や `group_add` ディレクティブ、ボリュームマウントオプション (`:delegated` など) の見直し。
- [ ] chore(dev-env): Dev Container 内での `cargo clean` 失敗問題を調査する (許容中) #dev-env #docker #ux #tech-debt
  - **現象**: Dev Container 内で `cargo clean` を実行すると "Device or resource busy" エラーで失敗する。
  - **原因(推測)**: `backend/target` が共有ボリュームのマウントポイントであるため、`cargo clean` がマウントポイント自体を削除しようとして失敗する。
  - **現状の判断**: `cargo build`/`check` は問題なく実行できるため、当面は `cargo clean` の失敗を許容する。根本解決が難しい場合は、特定のサブディレクトリのみ削除するスクリプト等で代替することも検討。

## 開発プロセス・ドキュメント (Process & Docs)

- [x] docs(process): 画面仕様の定義・管理方法を決定する (Storybook+MD) #process #documentation
- [x] docs(frontend): UI コンポーネントライブラリを選定する #frontend #decision
- [ ] docs(ui): 主要画面の仕様（画面定義・遷移）をドキュメント化する (Storybook+MD) #documentation #ui
- [x] docs(process): 開発サイクルの計画・管理方法を定義する (イテレーション計画ドキュメント?) #process
- [x] docs(process): テーブル設計（データモデリング）の記録方法を決定する #process
- [ ] docs(testing): フロントエンドのユニットテスト戦略・ツールを決定する (Jest/RTL?) #testing #frontend #process
- [ ] docs(testing): フロントエンドの E2E テスト戦略・ツールを選定する (Playwright/Cypress?) #testing #frontend #process
- [ ] docs(testing): バックエンドの結合テスト戦略を決定する #testing #backend #process
- [x] docs(adr): ADR 0001 (オニオンアーキテクチャ) の内容を現状に合わせて見直す #documentation
- [x] docs(adr): ADR 0002 (mockall 採用) の内容を現状に合わせて見直す #documentation
- [ ] chore(process): プロダクト戦略レビューを実施する (次回目安: MVP リリース後) #process #strategy
- [x] docs: プロジェクトのセットアップ手順を README に記述する #documentation
- [ ] docs(process): CONTRIBUTING.md を作成し、コミット規約や開発フローの詳細を記述する #documentation #process

## 実装・リファクタリング (Implementation)

- [x] refactor(domain): 状態を型で表現するアプローチを採用し、プレゼント予約ドメインに適用する #refactoring #architecture
- [ ] refactor(domain): 状態を型で表現するアプローチを採用し、支払いドメインに適用する #refactoring #architecture
- [x] refactor(sample): サンプル実装コードをプロダクト定義に合わせて修正・削除する #implementation #refactoring
- [x] refactor(infra): InMemoryRepository を PgRepository に置き換える #implementation #db #backend
- [ ] refactor(error): expect() の使用箇所を見直し、適切なエラーハンドリングに改善する #refactoring #backend #quality
- [ ] chore(lint): プロジェクト全体の警告（未使用 import 等）を修正する (`cargo fix`, `cargo clippy`) #quality #backend
- [x] refactor(application): ok_or_else を ok_or に修正する (clippy::unnecessary_lazy_evaluations) #tech-debt #quality
- [ ] refactor(domain): 各 ID 型に Default トレイトを実装する (clippy::new_without_default) #tech-debt #quality
- [x] refactor(main): #![allow(clippy::single_component_path_imports)] を削除し use 文を整理 #tech-debt #quality
- [ ] refactor(functions): 引数が多い関数をリファクタリングする (コマンドオブジェクト等, clippy::too_many_arguments) #tech-debt #architecture
- [ ] refactor(infra): PgRepository の DB エラーマッピングを改善する (sqlx::Error -> DomainError/InfrastructureError) #tech-debt #quality #backend
- [ ] test(infra): PgRepository のテストケースを拡充する (異常系、境界値など) #testing #quality #backend
- [ ] chore(deps): バックエンドの依存クレート(Rust)のバージョンを定期的に確認・更新する #tech-debt #quality #backend
- [ ] chore(deps): フロントエンドの依存パッケージ(npm)のバージョンを定期的に確認・更新する #tech-debt #quality #frontend
- [ ] chore(deps): 依存クレートのバージョンを定期的に確認・更新する #tech-debt #quality
- [ ] chore(ci): "potentially unused queries" 警告の原因を調査し修正する #tech-debt #quality #ci
- [x] chore(frontend): Next.js 14 / Tailwind CSS v3 へダウングレードする #tech-debt #frontend #compatibility
  - **目的**: StorybookでのDatePicker等のスタイリング問題を解消するため、より安定し `shadcn/ui` との互換性情報が多いバージョン構成に戻す。
  - **背景**: イテレーション 2025-W19 の Task 5 実施中に Next.js 15 + Tailwind v4 環境でスタイリングが崩れる問題が発生。Canary版対応は存在するが、安定性を優先。
  - **作業内容**: `package.json` 編集、`node_modules` 再生成、Tailwind設定ファイル調整など。
- [ ] chore(frontend): eslint-config-next の扱いを再検討する #tech-debt #frontend #lint
  - **背景**: Next.js/Tailwindダウングレード時に `unrs-resolver` (間接的依存) が原因で `npm install` が失敗したため、一時的に `eslint-config-next` を無効化して回避した。
  - **目的**: Next.jsプロジェクト向けの適切なESLint設定を再度有効にする。
  - **対応案**:
    - `eslint-config-next` の `unrs-resolver` に依存しないバージョンを探す/待つ。
    - `unrs-resolver` がDevContainer(Linux)環境で正しくインストールされる方法を調査・対応する。
    - 代替のESLint設定 (例: `eslint-plugin-react` 等の手動設定) を導入する。

## いつかやる (優先度 低)

- [ ] chore: リポジトリ全体の lint ルールを最新化する #tech-debt
- [ ] docs: ADR テンプレートを導入する #documentation
- [ ] chore(infra): 基本的なサーバー監視を設定する (Render/Vercel 標準機能) #monitoring #infra
- [ ] refactor(db): イミュータブルデータモデリング（履歴テーブル等）の導入を検討・実施する (Ref: ADR 0018) #tech-debt #architecture
- [ ] chore(frontend): Shadcn/ui (Tailwind CSS, Radix UI) をセットアップする (React 18 を使用中) #frontend #dev-env #ui
- [ ] chore(frontend): Shadcn/ui が React 19 に対応したら React を 19 にアップグレードする #frontend #deps #tech-debt
