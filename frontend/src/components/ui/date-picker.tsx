'use client'; // クライアントコンポーネントとしてマーク

import * as React from 'react';
import { format } from 'date-fns';
import { ja } from 'date-fns/locale'; // 日本語ロケールをインポート
import { Calendar as CalendarIcon } from 'lucide-react'; // アイコンをインポート

import { cn } from '@/lib/utils';
import { Button } from '@/components/ui/button'; // buttonVariants は DatePicker では不要なので削除
import { Calendar } from '@/components/ui/calendar';
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover';

// Props 型定義を更新
export interface DatePickerProps {
  value?: Date;
  onValueChange?: (date: Date | undefined) => void;
  disabled?: (date: Date) => boolean;
  isError?: boolean;
  placeholder?: string; // プレースホルダーの Props を追加
}

export function DatePicker({
  // Props を受け取るように変更
  value,
  onValueChange,
  disabled,
  isError,
  placeholder = '日付を選択', // デフォルトプレースホルダーを日本語に変更
}: DatePickerProps) {
  // 内部状態は持たず、Props で制御する
  // const [date, setDate] = React.useState<Date>();

  return (
    <Popover>
      <PopoverTrigger asChild>
        {/* isError に応じてスタイルを変更 */}
        <Button
          variant={'outline'}
          className={cn(
            'w-[280px] justify-start text-left font-normal',
            !value && 'text-muted-foreground',
            // isError が true ならエラー時のスタイルを適用 (ring を利用)
            isError && 'ring-destructive ring-2 ring-offset-2',
          )}
          aria-invalid={isError} // アクセシビリティのために追加
        >
          <CalendarIcon className="mr-2 h-4 w-4" />
          {/* format 関数に locale を指定 */}
          {value ? (
            format(value, 'PPP', { locale: ja })
          ) : (
            <span>{placeholder}</span>
          )}
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-auto p-0">
        <Calendar
          mode="single"
          locale={ja} // Calendar コンポーネントに locale を指定
          selected={value} // value Prop を使用
          onSelect={onValueChange} // onValueChange Prop を使用
          disabled={disabled} // disabled Prop を使用
          initialFocus
        />
      </PopoverContent>
    </Popover>
  );
}
