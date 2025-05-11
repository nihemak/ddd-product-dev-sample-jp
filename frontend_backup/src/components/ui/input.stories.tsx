import type { Meta, StoryObj } from '@storybook/react';

import { Input } from './input';
import { Label } from './label';

const meta = {
  title: 'UI/Input',
  component: Input,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  args: {
    type: 'text',
    placeholder: 'テキストを入力',
    disabled: false,
  },
  argTypes: {
    type: {
      control: 'select',
      options: ['text', 'password', 'email', 'number', 'search', 'tel', 'url'],
    },
    placeholder: { control: 'text' },
    disabled: { control: 'boolean' },
  },
} satisfies Meta<typeof Input>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    placeholder: 'デフォルトのプレースホルダー',
  },
};

export const ProductNameInput: Story = {
  args: {
    placeholder: '例: 手編みのマフラー',
    type: 'text',
  },
  render: (args) => (
    <div className="grid w-full max-w-sm items-center gap-1.5">
      <Label htmlFor="product-name">プレゼント情報 (品名/内容)</Label>
      <Input {...args} id="product-name" />
      <p className="text-muted-foreground text-sm">
        運営がプレゼントを識別できる内容を入力してください。
      </p>
    </div>
  ),
};

export const EmailInput: Story = {
  args: {
    placeholder: 'email@example.com',
    type: 'email',
  },
};

export const PasswordInput: Story = {
  args: {
    placeholder: 'パスワード',
    type: 'password',
  },
};

export const Disabled: Story = {
  args: {
    disabled: true,
    placeholder: '入力できません',
  },
};
