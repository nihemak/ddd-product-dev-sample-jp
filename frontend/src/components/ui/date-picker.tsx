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

// Props 型定義を元に戻す
export interface DatePickerProps {
  value?: Date;
  onValueChange?: (date: Date | undefined) => void;
  disabled?: (date: Date) => boolean;
  isError?: boolean;
  placeholder?: string; // プレースホルダーの Props を追加
}

export function DatePicker({
  // Props を再度受け取るように変更
  value,
  onValueChange,
  disabled,
  isError,
  placeholder = '日付を選択', // デフォルトプレースホルダーを日本語に戻す
}: DatePickerProps) {
  // 内部状態は持たない形に戻す
  // const [date, setDate] = React.useState<Date | undefined>(undefined);

  return (
    <Popover>
      <PopoverTrigger asChild>
        {/* isError に応じたスタイルを元に戻す */}
        <Button
          variant={'outline'}
          className={cn(
            'w-[280px] justify-start text-left font-normal',
            !value && 'text-muted-foreground',
            // isError が true ならエラー時のスタイルを適用 (ring を利用)
            isError && 'ring-2 ring-destructive ring-offset-2',
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
      {/* PopoverContent の className を元の状態に戻す */}
      <PopoverContent className="w-auto border bg-popover">
        <Calendar
          mode="single"
          locale={ja} // Calendar コンポーネントに locale を指定
          selected={value} // value Prop を使用
          onSelect={onValueChange} // onValueChange Prop を使用
          disabled={disabled} // disabled Prop を使用
          autoFocus // autoFocus に戻す
        />
      </PopoverContent>
    </Popover>
  );
}
