import type { Meta, StoryObj } from '@storybook/react';

import { Dummy } from './Dummy';

const meta: Meta<typeof Dummy> = {
  title: 'Example/Dummy', // カテゴリ名を Example にします
  component: Dummy,
  parameters: {
    // Optional parameter to center the component in the Canvas.
    layout: 'centered',
  },
  // This component will have an automatically generated Autodocs entry:
  // https://storybook.js.org/docs/writing-docs/autodocs
  tags: ['autodocs'],
};

export default meta;
type Story = StoryObj<typeof Dummy>;

// More on writing stories with args: https://storybook.js.org/docs/writing-stories/args
export const Default: Story = {
  args: {},
}; 