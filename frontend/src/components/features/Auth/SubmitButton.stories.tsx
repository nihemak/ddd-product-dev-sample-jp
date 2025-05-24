import type { Meta, StoryObj } from '@storybook/react';
import { SubmitButton } from './SubmitButton.tsx'; // .tsx 拡張子を追加
// ローディング状態の確認のため、インタラクションは不要ですが、
// クリックイベントをテストしたい場合は userEvent や expect をインポートします。
// import { userEvent, within, expect } from '@storybook/test';

const meta = {
  title: 'Features/Auth/SubmitButton',
  component: SubmitButton,
  tags: ['autodocs'],
  argTypes: {
    children: { control: 'text' },
    isLoading: { control: 'boolean' },
    disabled: { control: 'boolean' },
    onClick: { action: 'clicked' }, // onClickアクションをStorybookで確認
  },
} satisfies Meta<typeof SubmitButton>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    children: '送信する',
  },
};

export const Loading: Story = {
  args: {
    children: '送信する',
    isLoading: true,
  },
};

export const Disabled: Story = {
  args: {
    children: '送信する',
    disabled: true,
  },
};

export const CustomText: Story = {
  args: {
    children: '登録する',
  },
};

export const LoadingWithCustomText: Story = {
  args: {
    children: '更新中...', // ローディング時のテキストはコンポーネント側で固定しているので、childrenは表示されない
    isLoading: true,
  },
  // play: async ({ canvasElement }) => { // ローディング中のchildren表示確認（コンポーネント仕様による）
  //   const canvas = within(canvasElement);
  //   // expect(canvas.getByText('処理中...')).toBeInTheDocument(); // コンポーネント内のローディングテキストを確認
  // }
};
