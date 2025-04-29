# Frontend (Next.js + TypeScript)

このディレクトリは、記念日プレゼント予約・配送サービス のフロントエンドアプリケーションのコードを含みます。
Next.js, React, TypeScript を使用して構築されています。

## 技術スタック

*   Framework: [Next.js](https://nextjs.org/) (with App Router)
*   Language: [TypeScript](https://www.typescriptlang.org/)
*   UI Library: (後ほど Shadcn/ui を導入予定)
*   Styling: [Tailwind CSS](https://tailwindcss.com/)
*   Linting: [ESLint](https://eslint.org/)
*   Formatting: [Prettier](https://prettier.io/)
*   Package Manager: [npm](https://www.npmjs.com/)

## セットアップ

1.  **依存関係のインストール:**
    ```bash
    npm install
    ```

## 開発

1.  **開発サーバーの起動:**
    ```bash
    npm run dev
    ```
    開発サーバーが起動し、[http://localhost:3000](http://localhost:3000) でアプリケーションにアクセスできます。ファイルの変更は自動的に反映されます。

## ビルド

アプリケーションを本番用にビルドするには、以下のコマンドを実行します。

```bash
npm run build
```
ビルド成果物は `.next` ディレクトリに出力されます。

## 利用可能なスクリプト

*   `npm run dev`: 開発モードでアプリケーションを起動します (ホットリロード、エラーレポートなど)。
*   `npm run build`: 本番用にアプリケーションをビルドします。
*   `npm run start`: ビルドされた本番アプリケーションを起動します。
*   `npm run lint`: ESLint を使用してコードのエラーや潜在的な問題をチェックします。
*   `npm run lint:fix`: ESLint を使用して問題を自動修正します。
*   `npm run format`: Prettier を使用してコードをフォーマットします。

## ディレクトリ構成 (予定)

```
frontend/
├── public/              # 静的ファイル (画像など)
├── src/
│   ├── app/             # Next.js App Router (ルーティング、ページ、レイアウト)
│   ├── components/      # 再利用可能なUIコンポーネント
│   │   ├── ui/          # Shadcn/ui コンポーネント (予定)
│   │   └── ...
│   ├── lib/             # ユーティリティ関数、型定義など
│   ├── styles/          # グローバルスタイル、テーマなど (globals.css)
│   └── ...
├── .eslintrc.json       # ESLint 設定
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

*   バックエンドAPIは、ルートディレクトリの `backend` プロジェクトで開発されています。
*   デプロイは Vercel を使用する予定です (ADR 0008)。

## Learn More

To learn more about Next.js, take a look at the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.

You can check out [the Next.js GitHub repository](https://github.com/vercel/next.js) - your feedback and contributions are welcome!

## Deploy on Vercel

The easiest way to deploy your Next.js app is to use the [Vercel Platform](https://vercel.com/new?utm_medium=default-template&filter=next.js&utm_source=create-next-app&utm_campaign=create-next-app-readme) from the creators of Next.js.

Check out our [Next.js deployment documentation](https://nextjs.org/docs/app/building-your-application/deploying) for more details.
