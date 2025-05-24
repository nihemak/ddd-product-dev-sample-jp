import type { Meta, StoryObj } from '@storybook/react';
import HomePageUnauthenticated from './page.tsx'; // 同階層のpage.tsxをインポート

// Next.jsのLinkコンポーネントのモック
// プロジェクトのStorybook設定でグローバルにモックされているか、
// @storybook/nextjsアドオンが適切に設定されていれば不要な場合もあります。
// ここでは念のため記述しますが、状況に応じて削除または調整してください。
jest.mock('next/link', () => {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const MockedLink = ({ children, href, ...props }: any) => (
    <a href={href} {...props}>
      {children}
    </a>
  );
  MockedLink.displayName = 'MockedNextLink'; // displayName を設定
  return MockedLink;
});

const meta = {
  title: 'Pages/Unauthenticated/HomePage', // Storybook上での表示名
  component: HomePageUnauthenticated,
  parameters: {
    layout: 'fullscreen', // ページ全体のレイアウトを見るため
    // nextjs: { // App Routerを使用している場合、Storybookのnextjsアドオン設定が必要な場合があります
    //   appDirectory: true,
    //   router: {
    //     pathname: '/', // このページがどのパスに対応するか (任意)
    //   },
    // },
  },
  tags: ['autodocs'],
} satisfies Meta<typeof HomePageUnauthenticated>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {}, // このページコンポーネントはpropsを受け取らないので空
};
