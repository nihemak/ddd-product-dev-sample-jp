import type { StorybookConfig } from '@storybook/nextjs';

const config: StorybookConfig = {
  stories: ['../src/**/*.mdx', '../src/**/*.stories.@(js|jsx|mjs|ts|tsx)'],
  addons: [
    '@storybook/addon-links',
    '@storybook/addon-essentials',
    '@storybook/addon-onboarding',
    '@storybook/addon-interactions',
    // 必要に応じて他のアドオンを追加 (例:デザインツール連携、アクセシビリティチェックなど)
  ],
  framework: {
    name: '@storybook/nextjs',
    options: {},
  },
  core: {
    // builder: '@storybook/builder-webpack5', // デフォルトはWebpack5のはず。Viteを試す場合は別途設定
  },
  docs: {
    autodocs: 'tag',
  },
  staticDirs: ['../public'], // publicディレクトリを静的ファイルとして配信する場合
  typescript: {
    check: true, // ストーリーファイルの型チェックを有効にする
    // reactDocgen: 'react-docgen-typescript', // より詳細なPropsテーブルを生成する場合
  },
};
export default config;