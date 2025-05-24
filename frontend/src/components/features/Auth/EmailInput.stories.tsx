import type { Meta, StoryObj } from '@storybook/react';
import { EmailInput } from './EmailInput.tsx'; // .tsx 拡張子を追加

const meta = {
  title: 'Features/Auth/EmailInput',
  component: EmailInput,
  tags: ['autodocs'],
  argTypes: {
    label: { control: 'text' },
    name: { control: 'text' },
    placeholder: { control: 'text' },
    error: { control: 'text' },
    disabled: { control: 'boolean' },
    // value: { control: 'text' }, // コントロールするときの挙動が少し複雑になるので、一旦argsで直接指定
  },
} satisfies Meta<typeof EmailInput>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    label: 'メールアドレス',
    name: 'email',
    placeholder: 'your@example.com',
  },
};

export const WithValue: Story = {
  args: {
    label: 'メールアドレス',
    name: 'email',
    placeholder: 'your@example.com',
    defaultValue: 'test@example.com', // defaultValueを使用
  },
};

export const WithError: Story = {
  args: {
    label: 'メールアドレス',
    name: 'email',
    placeholder: 'your@example.com',
    defaultValue: 'invalid-email',
    error: '有効なメールアドレスを入力してください。',
  },
};

export const Disabled: Story = {
  args: {
    label: 'メールアドレス',
    name: 'email',
    placeholder: 'your@example.com',
    defaultValue: 'disabled@example.com',
    disabled: true,
  },
};
