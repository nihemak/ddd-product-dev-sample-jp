import globals from 'globals';
import pluginJs from '@eslint/js';
import tseslint from 'typescript-eslint';
import pluginReactConfig from 'eslint-plugin-react/configs/recommended.js';
import { fixupConfigRules } from '@eslint/compat';
import { FlatCompat } from '@eslint/eslintrc';
import path from 'path';
import { fileURLToPath } from 'url';
import storybookPlugin from 'eslint-plugin-storybook';
// import tailwindPlugin from "eslint-plugin-tailwindcss"; // この行を削除またはコメントアウト
import nextPlugin from '@next/eslint-plugin-next';
import prettierPlugin from 'eslint-plugin-prettier';
import prettierConfig from 'eslint-config-prettier';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const compat = new FlatCompat({
  baseDirectory: __dirname,
  recommendedConfig: pluginJs.configs.recommended,
});

export default [
  { files: ['**/*.{js,mjs,cjs,ts,jsx,tsx}'] },
  { languageOptions: { globals: { ...globals.browser, ...globals.node } } },
  pluginJs.configs.recommended,
  ...tseslint.config(...tseslint.configs.recommended),
  pluginReactConfig,
  {
    files: ['**/*.{ts,tsx,js,jsx}'],
    settings: {
      react: {
        version: 'detect',
      },
    },
    rules: {
      'react/react-in-jsx-scope': 'off',
      'react/prop-types': 'off',
    },
  },
  {
    files: ['**/*.{ts,tsx}'],
    languageOptions: {
      parserOptions: {
        project: ['./tsconfig.json'],
      },
    },
  },
  // 現在のNext.js設定ブロックをコメントアウトまたは削除
  // {
  //   // Next.js 用の設定
  //   files: ['src/**/*.{ts,tsx}'],
  //   plugins: {
  //     '@next/next': nextPlugin,
  //   },
  //   rules: {
  //     ...nextPlugin.configs.recommended.rules,
  //     ...nextPlugin.configs['core-web-vitals'].rules,
  //   },
  // },
  ...compat.extends('next/core-web-vitals'), // FlatCompat を使って next/core-web-vitals を適用
  {
    // Storybook 用の設定
    files: ['src/**/*.stories.{ts,tsx}'],
    ...storybookPlugin.configs['flat/recommended'], // 新しい推奨設定
  },
  // ...tailwindPlugin.configs["flat/recommended"], // この行を削除またはコメントアウト
  {
    // Prettier との競合を避けるための設定
    // ...fixupConfigRules(prettierConfig), // eslint-config-prettier
    plugins: {
      prettier: prettierPlugin, // eslint-plugin-prettier
    },
    rules: {
      ...prettierConfig.rules, // prettierConfig のルールをここに展開
      'prettier/prettier': 'warn', // Prettier のルール違反を警告として表示
    },
  },
  {
    ignores: [
      '**/node_modules/',
      '.next/',
      'out/',
      'storybook-static/',
      '**/dist/',
      '**/build/',
      '.eslintrc.js', // 以前の形式の ESLint 設定ファイルは無視
      'postcss.config.js', // 以前の形式の PostCSS 設定ファイルは無視
      'src/lib/api/generated/**', // 自動生成されるAPIクライアントコードは無視
    ],
  },
];
