import type { Meta, StoryObj } from '@storybook/react';

// --- ↓ インポートを追加 ---
import { Button } from '@/components/ui/button';
import { DatePicker } from '@/components/ui/date-picker';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Textarea } from '@/components/ui/textarea';
// --- ↑ インポートを追加 ---

// 仮のレイアウトコンポーネント
const PresentReservationShippingProxyPageLayout = () => {
  // ここにレイアウトを実装していく
  return (
    <div className="container mx-auto max-w-2xl p-4">
      {' '}
      {/* 全体のコンテナ */}
      <h1 className="mb-6 text-2xl font-bold">プレゼント発送代行予約</h1>
      {/* --- 届け先選択セクション --- */}
      <div className="mb-6">
        <Label htmlFor="destination">届け先</Label>
        <Select>
          <SelectTrigger id="destination">
            <SelectValue placeholder="届け先を選択してください" />
          </SelectTrigger>
          <SelectContent>
            <SelectGroup>
              <SelectLabel>登録済みの届け先</SelectLabel>
              <SelectItem value="address-1">自宅 (山田 太郎)</SelectItem>
              <SelectItem value="address-2">勤務先 (田中 花子)</SelectItem>
              <SelectItem value="address-3">友人宅 (鈴木 一郎)</SelectItem>
            </SelectGroup>
          </SelectContent>
        </Select>
      </div>
      {/* --- 予約情報入力セクション --- */}
      <div className="mb-6 space-y-4">
        <div>
          <Label htmlFor="anniversary-date">記念日</Label>
          <DatePicker /> {/* DatePicker は基本的な表示 */}
        </div>
        <div>
          <Label htmlFor="message">メッセージカード内容</Label>
          <Textarea
            id="message"
            placeholder="感謝の気持ちをメッセージカードに添えましょう（最大200文字）"
          />
        </div>
        <div>
          <Label htmlFor="product-name">
            発送代行プレゼント情報 (品名/内容)
          </Label>
          <Input id="product-name" placeholder="例: 手編みのマフラー" />
          <p className="text-muted-foreground mt-1 text-sm">
            運営がプレゼントを識別できる内容を入力してください。
          </p>
        </div>
        <div>
          <Label htmlFor="remarks">特記事項</Label>
          <Textarea
            id="remarks"
            placeholder="サイズ、重さ、壊れ物等の特記事項を自由記述。（例: 「割れ物注意」「要冷蔵」）"
          />
        </div>
      </div>
      {/* --- 予約操作ボタンエリア --- */}
      <div className="flex justify-end space-x-2">
        <Button variant="outline">キャンセル</Button>
        <Button>予約を確定する</Button>
      </div>
    </div>
  );
};

const meta = {
  title: 'Pages/PresentReservationShippingProxyLayout', // Storybook上のパス
  component: PresentReservationShippingProxyPageLayout, // 実際のレイアウトコンポーネントに変更
  parameters: {
    layout: 'fullscreen', // ページ全体を表示するため fullscreen を推奨
  },
  tags: ['autodocs'],
} satisfies Meta<typeof PresentReservationShippingProxyPageLayout>;

export default meta;
type Story = StoryObj<typeof meta>;

// 基本的なレイアウト表示用のストーリー
export const Default: Story = {
  // render 関数を使ってここにレイアウトを実装していく
  render: () => <PresentReservationShippingProxyPageLayout />,
};
