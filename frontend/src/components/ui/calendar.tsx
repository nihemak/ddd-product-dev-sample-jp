'use client';

import 'react-day-picker/dist/style.css'; // コメントアウトを解除
import * as React from 'react';
import { ChevronLeft, ChevronRight } from 'lucide-react';
import { DayPicker } from 'react-day-picker';

import { cn } from '@/lib/utils';
// import { buttonVariants } from '@/components/ui/button'; // 削除

function Calendar({
  className,
  // classNames, // この引数はDayPickerに渡さず、下記の固定値を使用 <- 削除
  showOutsideDays = true,
  ...props
}: React.ComponentProps<typeof DayPicker>) {
  return (
    <DayPicker
      showOutsideDays={showOutsideDays}
      className={cn('p-3', className)}
      classNames={{}} // ここは意図的に空オブジェクト
      components={{
        Chevron: ({ orientation, ...restChevronProps }) => {
          const Icon = orientation === 'left' ? ChevronLeft : ChevronRight;
          return (
            <Icon
              className={cn('h-4 w-4', restChevronProps.className)} // as any を削除
              {...restChevronProps}
            />
          );
        },
      }}
      {...props}
    />
  );
}
Calendar.displayName = 'Calendar'; // 追加: shadcn/uiの慣習に従う

export { Calendar };
