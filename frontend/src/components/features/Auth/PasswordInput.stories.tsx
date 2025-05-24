import type { Meta, StoryObj } from '@storybook/react';
import { PasswordInput } from './PasswordInput.tsx'; // .tsx 拡張子を追加
import { userEvent, within, expect } from '@storybook/test'; //testing-library と jest の代わりに @storybook/test を使用

const meta = {
  title: 'Features/Auth/PasswordInput',
  component: PasswordInput,
  tags: ['autodocs'],
  argTypes: {
    label: { control: 'text' },
    name: { control: 'text' },
    placeholder: { control: 'text' },
    error: { control: 'text' },
    disabled: { control: 'boolean' },
  },
} satisfies Meta<typeof PasswordInput>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    label: 'パスワード',
    name: 'password',
    placeholder: '••••••••',
  },
};

export const WithValue: Story = {
  args: {
    label: 'パスワード',
    name: 'password',
    placeholder: '••••••••',
    defaultValue: 'secret123',
  },
};

export const WithError: Story = {
  args: {
    label: 'パスワード',
    name: 'password',
    placeholder: '••••••••',
    defaultValue: 'short',
    error: '8文字以上で入力してください。',
  },
};

export const Disabled: Story = {
  args: {
    label: 'パスワード',
    name: 'password',
    placeholder: '••••••••',
    defaultValue: 'disabled_password',
    disabled: true,
  },
};

export const ToggleVisibility: Story = {
  args: {
    label: 'パスワード (表示切替)',
    name: 'password-toggle',
    placeholder: '••••••••',
    defaultValue: 'P@$$wOrd',
  },
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const passwordInput = canvas.getByLabelText('パスワード (表示切替)');
    const toggleButton = canvas.getByRole('button', {
      name: 'パスワードを表示',
    });

    // 初期状態はパスワード非表示 (type="password")
    expect(passwordInput).toHaveAttribute('type', 'password');

    // 表示ボタンをクリック
    await userEvent.click(toggleButton);

    // パスワード表示 (type="text"), ボタンのaria-label変更
    expect(passwordInput).toHaveAttribute('type', 'text');
    expect(
      canvas.getByRole('button', { name: 'パスワードを隠す' }),
    ).toBeInTheDocument();

    // 再度クリックして非表示に戻す
    await userEvent.click(toggleButton);
    expect(passwordInput).toHaveAttribute('type', 'password');
    expect(
      canvas.getByRole('button', { name: 'パスワードを表示' }),
    ).toBeInTheDocument();
  },
};
