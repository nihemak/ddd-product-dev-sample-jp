# ADR 0013: UIコンポーネントライブラリとして Shadcn/ui を採用

*   **Status**: Accepted
*   **Date**: 2025-04-20
*   **Deciders**: (議論参加者)

## Context and Problem Statement

フロントエンドSPA (React/Next.js) のUIを構築するために、再利用可能なUIコンポーネントを提供するライブラリ/ツールセットを選定する必要がある (ADR 0006)。選定にあたっては、開発効率、カスタマイズ性、Next.jsとの親和性、およびGitHubリポジトリ内で完結させたいという意向 (ADR 0012) を考慮する。

## Decision Drivers

*   **カスタマイズ性**: デザインの自由度が高いこと。
*   **開発者体験 (DX)**: APIが使いやすく、ドキュメントが整備されていること。
*   **Next.js App Router / RSC 互換性**: 最新のNext.js機能と問題なく連携できること。
*   **脱ライブラリ依存**: 外部ライブラリへの依存を極力減らし、コンポーネントをプロジェクト内で管理したい。
*   **パフォーマンス**: バンドルサイズへの影響が小さいこと。
*   **アクセシビリティ (a11y)**: アクセシビリティ標準に準拠していること。
*   **モダンさ・コミュニティ**: 現代的であり、活発なコミュニティが存在すること。

## Considered Options

1.  **Shadcn/ui**: Radix UI (ヘッドレス) と Tailwind CSS ベースのコンポーネント集。コンポーネントコードをプロジェクトにコピーして使用する。
    *   Pros: 完全なカスタマイズ性 (Tailwind)、コードを直接管理できる (GitHub完結)、Next.js App Router/RSCとの親和性◎、アクセシビリティ◎ (Radix依存)、コミュニティ活発◎。
    *   Cons: Tailwind CSSの学習/利用が必須、コンポーネントのアップデートは手動（CLI支援あり）、従来のライブラリとは異なる概念。
2.  **Material UI (MUI)**: Material Design準拠の包括的なライブラリ。
    *   Pros: 豊富なコンポーネント、巨大なコミュニティ。
    *   Cons: デザインの制約、バンドルサイズが大きくなる傾向、GitHub完結ではない。
3.  **Chakra UI**: スタイルプロップによる高いカスタマイズ性を持つライブラリ。
    *   Pros: 優れたDX、アクセシビリティ。
    *   Cons: Next.js App Router/RSC利用時に追加設定が必要、GitHub完結ではない。
4.  **Mantine**: Chakra UIに似た特徴を持つが、RSCサポートが進んでいるライブラリ。
    *   Pros: 豊富なコンポーネント、カスタマイズ性、RSC対応。
    *   Cons: GitHub完結ではない。

## Decision Outcome

**選択肢1「Shadcn/ui」を採用する。**

理由:
*   **GitHub完結との整合性**: コンポーネントのコードを直接プロジェクトリポジトリ内にコピーして管理する方式は、「外部依存を減らしリポジトリ内で完結させたい」という方針に最も合致する。
*   **高いカスタマイズ性**: Tailwind CSSをベースとしているため、デザインの自由度が非常に高い。
*   **モダンな技術スタック**: Radix UI (アクセシビリティ) と Tailwind CSS (スタイリング) というモダンで評価の高い技術を基盤としている。
*   **Next.jsとの親和性**: Next.js App Router や React Server Components との連携がスムーズに行えるように設計されている。
*   **開発者体験**: CLIによるコンポーネント追加や、整ったドキュメントにより、開発体験が良い。
*   **学習価値**: Tailwind CSSを学ぶ良い機会となる。

## Consequences

### Positive:
*   UIコンポーネントを完全に自身のコードベースで管理できる。
*   Tailwind CSSによる柔軟で効率的なスタイリングが可能になる。
*   バンドルサイズへの影響を最小限に抑えられる。
*   アクセシブルなUIを構築しやすい (Radix UIベース)。
*   モダンなフロントエンド開発のプラクティスを採用できる。

### Negative:
*   Tailwind CSSの知識と利用が前提となる。
*   コンポーネントのアップデートは、提供元が更新された場合に手動（またはCLI経由）で行う必要がある。
*   従来のUIライブラリとは異なるセットアップと利用方法に慣れる必要がある。

## References

*   [Shadcn/ui](https://ui.shadcn.com/)
*   [Radix Primitives](https://www.radix-ui.com/primitives)
*   [Tailwind CSS](https://tailwindcss.com/)
*   ADR 0006: フロントエンド技術として React (TypeScript) + Next.js を採用
*   ADR 0012: UIコンポーネント仕様管理に Storybook を採用 