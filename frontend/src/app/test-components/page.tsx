'use client';

import { Calendar } from '@/components/ui/calendar';
import React from 'react';

export default function TestComponentsPage() {
  const [date, setDate] = React.useState<Date | undefined>(new Date());

  return (
    <div className="p-4">
      <h1 className="mb-4 text-2xl font-bold">Component Test Page</h1>
      <p className="mb-2">DatePicker (Calendar) Test:</p>
      <div className="flex justify-center">
        {' '}
        {/* 中央寄せのためのコンテナ */}
        <Calendar
          mode="single"
          selected={date}
          onSelect={setDate}
          className="rounded-md border"
        />
      </div>
    </div>
  );
}
