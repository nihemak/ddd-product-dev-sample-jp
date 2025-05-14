# Frontend (Next.js + TypeScript)

このディレクトリは、記念日プレゼント予約・配送サービス のフロントエンドアプリケーションのコードを含みます。
Next.js, React, TypeScript を使用して構築されています。

## 技術スタック

- Framework: [Next.js](https://nextjs.org/) (with App Router)
- Language: [TypeScript](https://www.typescriptlang.org/)
- UI Library: [Shadcn/ui](https://ui.shadcn.com/) (ADR 0013 参照、導入済み)
- Styling: [Tailwind CSS](https://tailwindcss.com/)
- State Management: [React Query (TanStack Query)](https://tanstack.com/query/latest) (非同期状態管理)
- API Client Generation: [OpenAPI TypeScript Codegen](https://github.com/ferdikoomen/openapi-typescript-codegen) (バックエンドAPIスキーマからクライアントコードを生成)
- Linting: [ESLint](https://eslint.org/)
- Formatting: [Prettier](https://prettier.io/)
- Package Manager: [npm](https://www.npmjs.com/)

## セットアップ (ローカル環境)

**注記:** プロジェクト全体で Docker 開発環境 (ルートディレクトリの `docker-compose.yml`) の利用を推奨しています。ローカルに Node.js 環境を構築する場合の手順は以下の通りです。

1. **依存関係のインストール:**

   ```bash
   npm install
   ```

## 開発 (ローカル環境)

**注記:** プロジェクト全体で Docker 開発環境の利用を推奨しています。ローカル環境で開発サーバーを起動する手順は以下の通りです。

1. **開発サーバーの起動:**

   ```bash
   npm run dev
   ```

   開発サーバーが起動し、[http://localhost:3000](http://localhost:3000) でアプリケーションにアクセスできます。ファイルの変更は自動的に反映されます。

## Docker を使った開発

プロジェクトルートディレクトリにある `docker-compose.yml` を使用して、コンテナ化された開発環境を起動することを推奨します (ADR 0015)。

1. **開発コンテナの起動:**
   プロジェクトのルートディレクトリで以下のコマンドを実行します。これにより、フロントエンド (Next.js) と Storybook の開発サーバーがバックグラウンドで起動します。

   ```bash
   docker compose up -d frontend storybook
   ```

2. **アクセス:**
   - Next.js アプリケーション: [http://localhost:3000](http://localhost:3000)
   - Storybook: [http://localhost:6006](http://localhost:6006)
3. **ホットリロード:**
   ソースコード (`frontend/` ディレクトリ内) を変更すると、ブラウザは自動的にリロードされます。
4. **停止:**

   ```bash
   docker compose down
   ```

## Docker 設定の詳細

Docker を利用した開発環境の主な設定ポイントは以下の通りです。

- **Dockerfile (`frontend/Dockerfile`):**
  - **マルチステージビルド:** 最終的なイメージサイズを削減するため、依存関係のインストールステージ (`builder`) と実行ステージ (`final`) を分離しています。
  - **`npm ci`:** ビルドの再現性を高めるため、`package-lock.json` に基づいて依存関係をインストールします。
- **Docker Compose (`docker-compose.yml`):**
  - **ボリューム (`volumes`):**
    - `./frontend:/app`: ホストのソースコードをコンテナにマウントし、ホットリロードを可能にしています。
    - `frontend_node_modules:/app/node_modules`: `node_modules` 用に名前付きボリュームを使用しています。これにより、コンテナ起動時にホストの `node_modules` で上書きされるのを防ぎ、`frontend` サービスと `storybook` サービス間でビルド済みの依存関係を共有し、ディスクスペースを節約します。
    - `frontend_next:/app/.next`: Next.js のビルドキャッシュ用に名前付きボリュームを使用し、再ビルド時間を短縮します。
  - **環境変数 (`environment`):**
    - `WATCHPACK_POLLING=true`: Docker 環境で Next.js のホットリロードを安定させるために設定されています。
- **Docker Ignore (`frontend/.dockerignore`):**
  - `node_modules` や `.next` などを記載し、これらが Docker イメージのビルドコンテキストに含まれないようにしています。これにより、ビルド時のホストから Docker デーモンへのファイル転送が高速化されます。

## ビルド

アプリケーションを本番用にビルドするには、以下のコマンドを実行します。

```bash
npm run build
```

ビルド成果物は `.next` ディレクトリに出力されます。

## 利用可能なスクリプト

- `npm run dev`: 開発モードでアプリケーションを起動します (ホットリロード、エラーレポートなど)。
- `npm run build`: 本番用にアプリケーションをビルドします。
- `npm run start`: ビルドされた本番アプリケーションを起動します。
- `npm run lint`: ESLint を使用してコードのエラーや潜在的な問題をチェックします。
- `npm run lint:fix`: ESLint を使用して問題を自動修正します。
- `npm run format`: Prettier を使用してコードをフォーマットします。
- `npm run generate:api`: バックエンドのOpenAPIスキーマからAPIクライアントと型定義を生成します (`frontend/src/lib/api/generated/` に出力)。

## ディレクトリ構成

```
frontend/
├── public/              # 静的ファイル (画像など)
├── src/
│   ├── app/             # Next.js App Router (ルーティング、ページ、レイアウト)
│   │   └── (pages)/     # ルートグループ (例: health)
│   ├── components/      # 再利用可能なUIコンポーネント
│   │   ├── features/    # 特定機能に関連するコンポーネント群 (例: HealthCheckDisplay)
│   │   └── ui/          # Shadcn/ui コンポーネント (例: Button, Select, DatePicker等)
│   ├── hooks/           # カスタム Reactフック (例: useHealthCheck)
│   ├── lib/
│   │   ├── api/
│   │   │   └── generated/ # OpenAPIから自動生成されたAPIクライアントと型定義
│   │   └── utils/       # 汎用ユーティリティ関数 (もしあれば)
│   ├── providers/       # React Context Provider (例: QueryClientProviderComponent)
│   ├── services/        # (API呼び出し等のサービス層 - useHealthCheck のようなフックで代替または包含する方針)
│   ├── store/           # (状態管理ストア - React Query や Zustand 等の置き場、hooks で対応できる場合は不要)
│   ├── styles/          # グローバルスタイル、テーマなど (globals.css)
│   └── types/           # (グローバルな型定義 - APIクライアントや各コンポーネントで型定義する方針)
├── eslint.config.mjs    # ESLint 設定
├── .gitignore           # Git 無視リスト
├── .prettierignore      # Prettier 無視リスト
├── .prettierrc.json     # Prettier 設定
├── next.config.mjs      # Next.js 設定
├── package.json         # プロジェクト情報、依存関係、スクリプト
├── postcss.config.mjs   # PostCSS 設定 (Tailwind CSS 用)
├── tailwind.config.ts   # Tailwind CSS 設定
└── tsconfig.json        # TypeScript 設定
```

## その他

- バックエンドAPIは、ルートディレクトリの `backend` プロジェクトで開発されています。
- デプロイは Vercel を使用する予定です (ADR 0008)。

## Learn More

To learn more about Next.js, take a look at the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.

You can check out [the Next.js GitHub repository](https://github.com/vercel/next.js) - your feedback and contributions are welcome!

## Deploy on Vercel

The easiest way to deploy your Next.js app is to use the [Vercel Platform](https://vercel.com/new?utm_medium=default-template&filter=next.js&utm_source=create-next-app&utm_campaign=create-next-app-readme) from the creators of Next.js.

Check out our [Next.js deployment documentation](https://nextjs.org/docs/app/building-your-application/deploying) for more details.
