import type { Meta, StoryObj } from '@storybook/react';

import { Textarea } from './textarea';
import { Label } from './label'; // Labelも使うかもしれないのでインポート

const meta = {
  title: 'UI/Textarea',
  component: Textarea,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  args: {
    disabled: false,
    placeholder: 'テキストを入力してください',
  },
  argTypes: {
    placeholder: { control: 'text' },
    disabled: { control: 'boolean' },
  },
} satisfies Meta<typeof Textarea>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    placeholder: 'デフォルトのプレースホルダー',
  },
};

export const MessageInput: Story = {
  args: {
    placeholder: '感謝の気持ちをメッセージカードに添えましょう（最大200文字）',
    // maxLength: 200, // Storybookのcontrolでは直接扱えないが、仕様としては存在
  },
  render: (args) => (
    <div className="grid w-full max-w-sm items-center gap-1.5">
      <Label htmlFor="message">メッセージ</Label>
      <Textarea {...args} id="message" />
      {/* <p className="text-sm text-muted-foreground">残り文字数: ...</p> */}
    </div>
  ),
};

export const RemarksInput: Story = {
  args: {
    placeholder:
      'サイズ、重さ、壊れ物等の特記事項を自由記述。（例: 「割れ物注意」「要冷蔵」）',
  },
  render: (args) => (
    <div className="grid w-full max-w-sm items-center gap-1.5">
      <Label htmlFor="remarks">特記事項</Label>
      <Textarea {...args} id="remarks" />
    </div>
  ),
};

export const Disabled: Story = {
  args: {
    disabled: true,
    placeholder: '入力できません',
  },
};
