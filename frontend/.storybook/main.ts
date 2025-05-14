import type { StorybookConfig } from '@storybook/nextjs';
// import postcss from 'postcss'; // postcss の型が必要な場合、インポートする

const config: StorybookConfig = {
  stories: ['../src/**/*.mdx', '../src/**/*.stories.@(js|jsx|mjs|ts|tsx)'],
  addons: [
    '@storybook/addon-links',
    '@storybook/addon-essentials',
    '@storybook/addon-onboarding',
    '@storybook/addon-interactions',
    '@storybook/addon-themes',
    {
      name: '@storybook/addon-styling-webpack',
      options: {
        rules: [
          // Replaces existing CSS rules to support PostCSS
          {
            test: /\.css$/,
            use: [
              'style-loader',
              {
                loader: 'css-loader',
                options: { importLoaders: 1 },
              },
              {
                // プロジェクトルートの postcss.config.js を参照する
                loader: 'postcss-loader',
                options: { implementation: require.resolve('postcss') },
              },
            ],
          },
        ],
      },
    },
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
  /* typescript: {
    check: true, // ストーリーファイルの型チェックを有効にする
    // reactDocgen: 'react-docgen-typescript', // より詳細なPropsテーブルを生成する場合
    compilerOptions: {
      // ECMAScript のターゲットバージョンを指定
      target: 'ES2020', // または 'ESNext' など
      // 必要に応じて他のコンパイラオプションもここに追加可能
    },
  }, */
};
export default config;
