# ADR 0019: フロントエンドアーキテクチャ方針の決定

*   **Status**: Accepted
*   **Date**: 2025-05-04
*   **Deciders**: nihemak, AI Assistant

## Context and Problem Statement

React (Next.js) + TypeScript をフロントエンド技術として採用 (ADR 0006) し、SPA + API アーキテクチャ (ADR 0005) で構築することを決定した。
フロントエンドアプリケーション (`frontend/`) 内部の具体的なアーキテクチャ方針を定める必要がある。
目指すのは、シンプルさ、テスト容易性、保守性、およびバックエンドのオニオンアーキテクチャ (ADR 0001) の原則（関心の分離など）との整合性である。
ディレクトリ構造、コンポーネント設計、状態管理、APIクライアント連携に関する方針を明確にする必要がある。

## Decision Drivers

*   **シンプルさと保守性**: 理解しやすく、長期的にメンテナンス可能な構造。
*   **テスト容易性**: 各コンポーネントやロジックを独立してテストしやすいこと。
*   **開発者体験**: モダンなツールやプラクティスを活用し、効率的に開発できること。
*   **モダンなプラクティス**: 2025年現在において一般的で堅牢な React/Next.js 開発手法を採用すること。
*   **全体整合性**: バックエンドアーキテクチャとの思想的な一貫性（関心の分離）。
*   **既存選択肢の活用**: `shadcn/ui` (ADR 0013) や OpenAPI (ADR 0011) を効果的に利用すること。
*   **グローバル状態の必要性**: ユーザー認証情報（`roadmap.md`）など、アプリケーション全体で共有する状態を管理する必要がある。

## Considered Options

*   **ディレクトリ構造**: 機能ベース (`features/`)、レイヤーベース (`hooks/`, `services/` など)、あるいはそれらの組み合わせ。
*   **コンポーネント設計**: Atomic Design、Container/Presentational パターン、カスタムフックによるロジック分離。
*   **状態管理**: React Context API、Redux、Recoil、Jotai、Zustand、React Query (TanStack Query)、SWR。
*   **APIクライアント**: 標準 `fetch`、`axios`、OpenAPI スキーマからのコード生成。

## Decision Outcome

以下のフロントエンドアーキテクチャ方針を採用する。

1.  **ディレクトリ構造:** `frontend/src/` 以下を以下のように構成する。
    ```
    frontend/src/
    ├── app/                   # Next.js App Router (Pages & Layouts)
    ├── components/
    │   ├── ui/                # shadcn/ui components (generated)
    │   └── features/          # Feature-specific components (e.g., PresentForm)
    ├── hooks/                 # Custom Hooks (state logic, API logic wrappers, etc.)
    ├── lib/                   # Utility functions, constants
    │   └── api/               # API client related (generated types, client instance)
    ├── providers/             # React Context Providers (e.g., React Query Provider, Auth Provider)
    ├── stores/                # Zustand store definitions
    ├── styles/                # Global styles
    └── types/                 # Shared TypeScript types (if API types not covering all)
    ```
    *   *補足: グローバル状態管理として Zustand を採用するため、`stores/` ディレクトリを追加。関連する Provider は `providers/` に配置。*

2.  **コンポーネント設計:**
    *   基本的なUI部品は `shadcn/ui` を活用し、`components/ui/` に配置（または参照）。
    *   特定の機能やページに関連するコンポーネントは `components/features/` に作成する。
    *   状態管理ロジックや副作用（API呼び出し等）はカスタムフック (`hooks/`) にカプセル化し、コンポーネントの関心を分離する。

3.  **状態管理:**
    *   **ローカル状態:** 各コンポーネント固有の一時的な状態は `useState` を使用する。
    *   **サーバーキャッシュ状態:** APIから取得したデータとその状態管理（キャッシュ、再検証、ローディング/エラー状態）には **React Query (TanStack Query)** を使用する。
    *   **グローバル状態:** アプリケーション全体で共有される状態（例: ユーザー認証情報）には **Zustand** を使用する。

4.  **APIクライアント:**
    *   バックエンドの OpenAPI スキーマ (`utoipa` で生成) から、**型安全な API クライアントと関連型定義を自動生成する** (`openapi-typescript-codegen` 等のツール利用を推奨)。
    *   生成されたクライアントや関連設定は `lib/api/` ディレクトリで管理する。

## Consequences

### Positive:
*   明確なディレクトリ構造により、コードの見通しが良くなる。
*   関心の分離が促進され、コンポーネントとロジックのテスト容易性が向上する。
*   React Query と Zustand の採用により、状態管理の記述が効率化され、ボイラープレートが削減される。
*   APIクライアントの自動生成により、バックエンドとの連携における型安全性が向上し、開発効率が上がる。
*   モダンで一般的なツールセットを採用することで、開発者体験が向上し、コミュニティの知見を活用しやすくなる。

### Negative:
*   React Query, Zustand, APIクライアント生成ツールの学習・設定コストが初期に発生する。
*   導入するライブラリへの依存が増える。

## References

*   ADR 0001: Use Onion Architecture
*   ADR 0005: Adopt SPA + API Architecture
*   ADR 0006: Adopt React (Next.js) + TypeScript for Frontend
*   ADR 0011: Adopt OpenAPI with utoipa
*   ADR 0013: Adopt shadcn/ui for Component Library
*   `docs/product/roadmap.md` (Requirement for User Authentication) 