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
- [ ] chore(dev-env): DevContainer 及び Docker-out-of-Docker (DooD) 環境の安定性とパフォーマンスを向上させる (権限問題、cargo clean 問題含む) #dev-env #docker #ux #tech-debt #performance
  - **現象**:
    - Dev Container 内で `cargo check`, `rust-analyzer`, または `npm install` 等を実行すると、共有ボリューム (`/workspace/backend/target` や `/workspace/frontend/node_modules`) への書き込み権限エラーが発生する場合がある。
    - Dev Container 内で `cargo clean` を実行すると "Device or resource busy" エラーで失敗する。
    - Docker Compose でのサービス起動・停止やログ表示が遅い、または不安定になることがある。

## 開発プロセス・ドキュメント (Process & Docs)

- [x] docs(process): 画面仕様の定義・管理方法を決定する (Storybook+MD) #process #documentation
- [x] docs(frontend): UI コンポーネントライブラリを選定する #frontend #decision
- [ ] docs(ui): 主要画面の仕様（画面定義・遷移）をドキュメント化する (Storybook+MD) #documentation #ui
- [x] docs(process): 開発サイクルの計画・管理方法を定義する (イテレーション計画ドキュメント?) #process
- [x] docs(process): テーブル設計（データモデリング）の記録方法を決定する #process
- [x] docs(testing): フロントエンドのユニットテスト戦略・ツールを決定する (Vitest/RTL + Storybook Interaction Tests) #testing #frontend #process
- [x] docs(testing): フロントエンドの E2E テスト戦略・ツールを選定する (Playwright) #testing #frontend #process
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
- [ ] chore(frontend): HealthCheckDisplay.test.tsx の Linter エラー (@typescript-eslint/no-unused-vars, @typescript-eslint/no-explicit-any) を修正する #tech-debt #frontend #lint #testing
- [ ] chore(frontend): playwright.config.ts の型定義エラーを解消する #tech-debt #quality #frontend #testing
- [ ] chore(frontend): Vitest環境におけるパスエイリアス (@/) の型解決エラーを解消する #tech-debt #quality #frontend #testing
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
- [ ] fix(frontend): Storybook/アプリで shadcn/ui コンポーネントのスタイルが正しく適用されない問題を調査・修正する #tech-debt #frontend #ui-styling
  - **現象**:
    - その他、コンポーネントの枠線や背景色が意図通りに表示されない場合がある。(DatePickerの件は別途記載あり)
    - `DatePicker` の年月表示が中央揃えにならない、月変更ボタンが左に寄る、曜日ヘッダーとカレンダー本体の配置が崩れる問題も発生していた (2025-W20 イテレーションで詳細調査、暫定対応済み)。
  - **原因(推測)**:
    - Tailwind CSS の設定またはビルドプロセスの不備。
    - グローバルなCSSの競合、詳細度の問題。
    - Next.js または Storybook 環境における特有のスタイル適用の問題。
    - 依存パッケージ間のバージョン不整合。
    - `DatePicker` に関しては、`react-day-picker` が生成するHTML構造と、`classNames` prop経由で適用しようとしたTailwind CSSクラス（shadcn/uiデフォルトやカスタムスタイル）との間に不整合があった。特に、ナビゲーションボタンとキャプション（年月表示）のコンテナ構造が想定と異なっていたことが判明 (2025-W20 イテレーションでの調査結果)。
  - **対応方針(検討)**:
    - ブラウザ開発者ツールでの詳細なスタイル調査。
    - Tailwind CSS, PostCSS, Next.js, Storybook の設定ファイルの再確認と修正。
    - 必要であれば、フロントエンド環境のクリーンな状態からの再構築（Next.js, Tailwind, shadcn/ui, Storybook の再セットアップ）。
    - `DatePicker` に関する2025-W20イテレーションでの暫定対応: `frontend/src/components/ui/calendar.tsx` 内の `DayPicker` コンポーネントの `classNames` propを空オブジェクト `{}` に設定。これにより、`react-day-picker/dist/style.css` によるデフォルトの基本的なスタイリングが適用され、デザインの洗練度は低いものの、主要コンポーネントの配置（年月とボタンが上部、曜日と日付の整合性）は確保された。
    - `DatePicker` の将来的な改善点:
      - `shadcn/ui` の `Calendar` コンポーネントとして期待される、より洗練されたデザインとTailwind CSSによるスタイリングに戻すための再調査と修正。
      - `react-day-picker` のバージョンや `shadcn/ui` の `Calendar` コンポーネントの更新状況を注視し、公式の推奨するHTML構造や `classNames` の使い方に追従する。
- [x] chore(frontend): APIクライアント生成ツール (openapi-typescript-codegen) を導入・設定する #frontend #api #dev-env
- [x] chore(frontend): React Query を導入・設定し、非同期状態管理の基本を整備する #frontend #state-management #dev-env
- [ ] feat(testing): フロントエンドテストツール (Vitest, RTL, Storybook Interaction Tests, Playwright) の導入と初期テスト作成 #testing #frontend #dev-env #e2e
- [ ] feat(frontend): Storybook Interaction Tests をセットアップし、UIコンポーネントのインタラクションテストを作成する #testing #frontend #storybook #dev-env
- [ ] feat(frontend): Playwright をセットアップし、基本的なE2Eテスト（例: ヘルスチェックページ表示）を作成する #testing #frontend #e2e #dev-env
  - **サブタスク:**
    - [x] `@playwright/test` の依存関係を `package.json` に追加する。
    - [x] DevContainer内で `npx playwright install --with-deps` を実行し、ブラウザバイナリとOS依存関係をインストールする。
    - [x] `playwright.config.ts` を作成・設定し、基本的なプロジェクト設定を行う。
    - [x] ヘルスチェックページ (`/health`) を表示し、特定のテキストが存在することを確認する簡単なE2Eテスト (`*.spec.ts`) を作成する。
    - [ ] テスト実行用のnpmスクリプト (例: `test:e2e`, `test:e2e:ui`) を `package.json` に追加する。
    - [ ] ローカルでテストが実行でき、Playwright UI Mode (`--ui`) やTrace Viewerが利用できることを確認する。
  - **現状 (2025-W20 イテレーション終了時):** 上記サブタスクの途中まで完了。npmスクリプトの追加とローカルでの詳細な実行確認（UI Mode, Trace Viewer）が未了。テスト実行時に `Error [ERR_MODULE_NOT_FOUND]: Cannot find package '@playwright/test' imported from /workspace/frontend/playwright.config.ts` が発生する問題が残っている。
  - **背景・試行錯誤 (既存の記述を維持、または必要に応じて更新):**
    - `frontend/playwright.config.ts` およびテストファイル (`frontend/tests/e2e/health.spec.ts`) で `@playwright/test` の型定義が見つからないLinterエラーが発生し、`// @ts-nocheck` で一時的に抑制。
    - `playwright.config.ts` をCommonJS形式 (`require`) に変更したが `ReferenceError: require is not defined in ES module scope` が発生。
    - 設定ファイルを `playwright.config.mjs` にリネーム（中身はESM）しても `ERR_MODULE_NOT_FOUND` は解消せず。
  - **考えられる原因・調査ポイント (既存の記述を維持、または必要に応じて更新):**
    - Node.jsのモジュール解決 (ESM vs CommonJS、`frontend/package.json` の `"type": "module"` との関連)。
    - `tsconfig.json` の `compilerOptions` (`module`, `moduleResolution` など) の設定。
    - Playwrightが内部的に設定ファイルやテストファイルをどのようにトランスパイル/ロードしているか。
    - Monorepo構成特有の問題の可能性。
  - **参考情報 (既存の記述を維持、または必要に応じて更新):**
    - [Cucumber+Playwright+Bunでフロントエンドのe2eテストを行ったメモ (Qiita)](https://qiita.com/hibohiboo/items/3b83c70abdc7e8d46d0e) - Bun + Vite + Monorepo環境でのESM/TSConfig設定の試行錯誤例。
- [ ] chore(frontend): playwright.config.ts の型定義エラーを解消する (`feat(frontend): Playwright をセットアップし...` タスクに詳細統合) #tech-debt #quality #frontend #testing
- [ ] chore(frontend): Vitest と Storybook Interaction Test (`@storybook/experimental-addon-test/vitest-plugin`) の連携問題を解消する #tech-debt #testing #frontend #storybook #vitest
  - **現象 (2025-W20 イテレーション時):** `vitest run` 実行時に `@storybook/experimental-nextjs-vite` 内部で `Could not resolve "sb-original/image-context"` エラーが発生する。
  - **試したこと:**
    - `vitest.config.ts` の `test.deps.external` に `sb-original/image-context` を追加。
    - `vitest.config.ts` の `test.deps.optimizer.web.exclude` に `sb-original/image-context` を追加。
  - **一時的な対処:** `vitest.config.ts` で Storybook テスト用の `workspace` 設定をコメントアウトし、Vitest による Storybook テストの実行を無効化。
  - **調査の方向性:**
    - `@storybook/experimental-nextjs-vite` および関連する `@storybook/*` パッケージのバージョン確認・更新。
    - Storybook の設定ファイル (`.storybook/main.ts` 等) と `@storybook/experimental-nextjs-vite` の推奨設定との整合性確認。
    - Storybook / Vitest の GitHub Issue 等での類似問題の検索。

## いつかやる (優先度 低)

- [ ] chore: リポジトリ全体の lint ルールを最新化する #tech-debt
- [ ] docs: ADR テンプレートを導入する #documentation
- [ ] chore(infra): 基本的なサーバー監視を設定する (Render/Vercel 標準機能) #monitoring #infra
- [ ] refactor(db): イミュータブルデータモデリング（履歴テーブル等）の導入を検討・実施する (Ref: ADR 0018) #tech-debt #architecture
- [ ] chore(frontend): Shadcn/ui (Tailwind CSS, Radix UI) をセットアップする (React 18 を使用中) #frontend #dev-env #ui
- [ ] chore(frontend): Shadcn/ui が React 19 に対応したら React を 19 にアップグレードする #frontend #deps #tech-debt
- [ ] chore(dev-env): Markdown/Rust の lint/format 環境を整備する (Husky 等の git hooks 導入検討) #dev-env #linting #quality
- [ ] chore(frontend): フロントエンド開発環境の全体的なクリーンアップを実施する (依存関係整理、不要な設定削除など) #frontend #dev-env #tech-debt
