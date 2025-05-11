import type { Meta, StoryObj } from '@storybook/react';
import { Loader2 } from 'lucide-react';

import { Button } from './button';

// More on how to set up stories at: https://storybook.js.org/docs/writing-stories#default-export
const meta = {
  title: 'UI/Button', // Components/UI 下に配置されるように Title を変更
  component: Button,
  parameters: {
    // Optional parameter to center the component in the Canvas.
    layout: 'centered',
  },
  // This component will have an automatically generated Autodocs entry:
  // https://storybook.js.org/docs/writing-docs/autodocs
  tags: ['autodocs'],
  // More on argTypes: https://storybook.js.org/docs/api/argtypes
  argTypes: {
    variant: {
      control: 'select',
      options: [
        'default',
        'destructive',
        'outline',
        'secondary',
        'ghost',
        'link',
      ],
    },
    size: {
      control: 'select',
      options: ['default', 'sm', 'lg', 'icon'],
    },
    asChild: {
      control: 'boolean',
    },
    disabled: {
      control: 'boolean',
    },
  },
} satisfies Meta<typeof Button>;

export default meta;
type Story = StoryObj<typeof meta>;

// More on writing stories with args: https://storybook.js.org/docs/writing-stories/args
export const Default: Story = {
  args: {
    children: 'デフォルトボタン',
  },
};

export const PrimaryAction: Story = {
  args: {
    children: '予約を確定する',
    variant: 'default', // shadcn/ui のデフォルトがプライマリ相当
  },
};

export const PrimaryActionDisabled: Story = {
  args: {
    children: '予約を確定する',
    variant: 'default',
    disabled: true,
  },
};

export const PrimaryActionLoading: Story = {
  args: {
    children: (
      <>
        <Loader2 className="mr-2 h-4 w-4 animate-spin" />
        予約処理中...
      </>
    ),
    variant: 'default',
    disabled: true, // ローディング中は通常 disabled
  },
};

export const CancelAction: Story = {
  args: {
    children: 'キャンセル',
    variant: 'outline', // 仕様に合わせて outline を選択
  },
};

export const SecondaryAction: Story = {
  args: {
    children: '戻る',
    variant: 'secondary',
  },
};

export const DestructiveAction: Story = {
  args: {
    children: '削除する',
    variant: 'destructive',
  },
};

export const Outline: Story = {
  args: {
    children: 'Outline',
    variant: 'outline',
  },
};

export const Ghost: Story = {
  args: {
    children: 'Ghost',
    variant: 'ghost',
  },
};

export const Link: Story = {
  args: {
    children: 'Link',
    variant: 'link',
  },
};

export const Large: Story = {
  args: {
    children: 'Large Button',
    size: 'lg',
  },
};

export const Small: Story = {
  args: {
    children: 'Small Button',
    size: 'sm',
  },
};

export const Disabled: Story = {
  args: {
    children: 'Disabled',
    variant: 'default',
    disabled: true,
  },
};

export const Loading: Story = {
  args: {
    variant: 'default',
    disabled: true,
    children: (
      <>
        <Loader2 className="animate-spin" />
        Loading...
      </>
    ),
  },
};

// Example for icon button (requires an icon library like lucide-react)
// export const Icon: Story = {
//   args: {
//     variant: 'outline',
//     size: 'icon',
//     children: <svg>...</svg>, // Replace with your icon component
//   },
// };
