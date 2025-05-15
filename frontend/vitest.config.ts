import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { defineConfig } from 'vitest/config';
import react from '@vitejs/plugin-react';

import { storybookTest } from '@storybook/experimental-addon-test/vitest-plugin';

const dirname =
  typeof __dirname !== 'undefined'
    ? __dirname
    : path.dirname(fileURLToPath(import.meta.url));

// More info at: https://storybook.js.org/docs/writing-tests/test-addon
export default defineConfig({
  plugins: [react()],
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: './vitest.setup.ts', // グローバルセットアップ用のファイルパス
    css: true, // CSSファイルのインポートを有効にする (例: Tailwind CSS)
    deps: {
      optimizer: {
        web: {
          exclude: ['sb-original/image-context'],
        },
      },
    },
    exclude: ['**/node_modules/**', '**/dist/**', 'tests/e2e/**'],
    // workspace: [
    //   {
    //     extends: true,
    //     plugins: [
    //       storybookTest({ configDir: path.join(dirname, '.storybook') }),
    //     ],
    //     test: {
    //       name: 'storybook',
    //       browser: {
    //         enabled: true,
    //         headless: true,
    //         name: 'chromium',
    //         provider: 'playwright',
    //       },
    //       setupFiles: ['.storybook/vitest.setup.ts'],
    //     },
    //   },
    // ],
  },
  resolve: {
    alias: [
      { find: '@/', replacement: new URL('./src/', import.meta.url).pathname },
    ],
  },
});
