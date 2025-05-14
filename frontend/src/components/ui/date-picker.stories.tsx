import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test'; // fn をインポート (以前の actions 代替)
import { addDays, subDays } from 'date-fns'; // 日付操作用関数をインポート

import { DatePicker } from './date-picker';

const meta: Meta<typeof DatePicker> = {
  title: 'UI/DatePicker',
  component: DatePicker,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  // argTypes で新しい Props を制御可能にする
  argTypes: {
    value: {
      control: 'date',
    },
    onValueChange: {
      action: 'onValueChange', // Storybook Actions Addon でログ出力
    },
    disabled: {
      // disabled は関数なので直接制御は難しい
      // ストーリー側で具体的な関数を渡す
      control: false, // Controls パネルからは非表示
    },
    isError: {
      control: 'boolean',
    },
    placeholder: {
      control: 'text',
    },
  },
  args: {
    // デフォルトの args (モック関数など)
    onValueChange: fn(),
    isError: false,
    placeholder: '日付を選択', // デフォルトプレースホルダーを日本語に合わせる
  },
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    // value は指定しない (未選択状態)
  },
};

export const DateSelected: Story = {
  args: {
    value: new Date(), // 今日の日付を選択済みとする
  },
};

export const CustomPlaceholder: Story = {
  args: {
    placeholder: '記念日を入力してください', // 日本語に修正
  },
};

export const WithDisabledDates: Story = {
  args: {
    // 例: 今日より前の日付と、10日後より先の日付を無効化
    disabled: (date: Date) =>
      date < subDays(new Date(), 1) || date > addDays(new Date(), 10),
    placeholder: '本日～10日後の日付を選択', // 日本語に修正
  },
};

export const WithError: Story = {
  args: {
    value: new Date(), // エラー状態でも選択値は表示される想定
    isError: true,
  },
};

// 例: DatePicker コンポーネントが disabledDates prop を持つと仮定した場合
// export const WithDisabledDates: Story = {
//   args: {
//     disabledDates: (date) => date.getDay() === 0 || date.getDay() === 6, // 週末無効
//   },
// };

// 例: DatePicker コンポーネントが error prop (boolean) を持つと仮定した場合
// export const WithError: Story = {
//   args: {
//     error: true,
//   },
// };
