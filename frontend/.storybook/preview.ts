import type { Preview } from '@storybook/react';
import '../src/app/globals.css'; // Next.js App Router のグローバルCSSをインポート

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
  // globalTypes: { // グローバルなテーマ切り替えなどを設定する場合
  //   theme: {
  //     name: 'Theme',
  //     description: 'Global theme for components',
  //     defaultValue: 'light',
  //     toolbar: {
  //       icon: 'circlehollow',
  //       items: ['light', 'dark'],
  //       showName: true,
  //     },
  //   },
  // },
  // decorators: [ // グローバルなデコレーター
  //   (Story) => (
  //     <ThemeProvider theme={yourTheme}> {/* テーマプロバイダーなどでラップする場合 */}
  //       <Story />
  //     </ThemeProvider>
  //   ),
  // ],
};

export default preview;
