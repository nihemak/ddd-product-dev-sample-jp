import type { Preview } from '@storybook/react';
import { withThemeByClassName } from '@storybook/addon-themes'; // インポート
import '../src/app/globals.css'; // Next.js App Router のグローバルCSSをインポート
import React from 'react'; // Reactをインポート

const preview: Preview = {
  parameters: {
    actions: { argTypesRegex: '^on[A-Z].*' },
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/i,
      },
    },
    // backgrounds: { // 必要に応じて背景色を設定
    //   default: 'light',
    //   values: [
    //     { name: 'light', value: '#ffffff' },
    //     { name: 'dark', value: '#000000' },
    //   ],
    // },
    // viewport: { // 必要に応じてビューポートを設定
    //   viewports: INITIAL_VIEWPORTS,
    // },
    // layout: 'centered', // 'padded', 'fullscreen' など
  },
  decorators: [
    withThemeByClassName({
      themes: {
        light: 'light', // 'light' クラスを適用
        dark: 'dark', // 'dark' クラスを適用 (必要であれば)
      },
      defaultTheme: 'light',
      // parentSelector: 'body', // クラスを適用するセレクター (デフォルトは :root)
    }),
  ],
  // 必要に応じて、ツールバーからテーマを切り替えるための globalTypes を設定
  // globalTypes: {
  //   theme: {
  //     name: 'Theme',
  //     description: 'Global theme for components',
  //     defaultValue: 'light',
  //     toolbar: {
  //       icon: 'mirror', // 'circlehollow' や他のアイコンも可
  //       items: [
  //         { value: 'light', title: 'Light' },
  //         { value: 'dark', title: 'Dark' },
  //       ],
  //       showName: true,
  //     },
  //   },
  // },
};

export default preview;
