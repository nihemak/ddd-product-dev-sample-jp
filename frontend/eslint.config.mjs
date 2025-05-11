import { dirname } from "path";
import { fileURLToPath } from "url";
import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';
import nextPlugin from '@next/eslint-plugin-next';
import storybookPlugin from 'eslint-plugin-storybook';
import tailwindcssPlugin from 'eslint-plugin-tailwindcss';
import prettierConfig from 'eslint-config-prettier';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

export default tseslint.config(
  eslint.configs.recommended,
  ...tseslint.configs.recommendedTypeChecked,
  {
    plugins: {
      '@next/next': nextPlugin
    },
    rules: {
      // Next.js 固有のルールをここに追加
      // 例: '@next/next/no-html-link-for-pages': 'error',
    }
  },
  {
    files: ['**/*.stories.@(ts|tsx|js|jsx|mjs|cjs)'],
    plugins: {
      storybook: storybookPlugin,
    },
    rules: {
      ...storybookPlugin.configs.recommended.rules,
      // Storybook 固有のルールで上書きや追加があればここに
    },
  },
  {
    plugins: {
      tailwindcss: tailwindcssPlugin,
    },
    rules: {
      ...tailwindcssPlugin.configs.recommended.rules,
      // Tailwind CSS 固有のルールで上書きや追加があればここに
    },
  },
  prettierConfig,
  {
    languageOptions: {
      parserOptions: {
        project: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
    linterOptions: {
      reportUnusedDisableDirectives: 'warn',
    },
    rules: {
      // プロジェクト固有のルールや上書き
      // 例: 'no-console': 'warn',
      // tseslint/explicit-function-return-type などは好みに応じて
    },
  },
  {
    ignores: ['.next/', 'node_modules/', 'eslint.config.mjs'],
  }
);
