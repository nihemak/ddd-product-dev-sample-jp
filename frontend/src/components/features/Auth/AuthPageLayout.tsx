import React from 'react';

interface AuthPageLayoutProps {
  title: string;
  children: React.ReactNode;
}

export const AuthPageLayout: React.FC<AuthPageLayoutProps> = ({
  title,
  children,
}) => {
  return (
    <div className="flex min-h-screen flex-col items-center justify-center bg-gray-100">
      <header className="py-6">
        {/* 仮のロゴ */}
        <h1 className="text-center text-3xl font-bold text-blue-600">ロゴ</h1>
      </header>
      <main className="w-full max-w-md space-y-6 rounded-lg bg-white p-8 shadow-md">
        <h2 className="text-center text-2xl font-bold text-gray-900">
          {title}
        </h2>
        {children}
      </main>
      <footer className="mt-8 py-6 text-center text-gray-500">
        <p>
          &copy; {new Date().getFullYear()} 記念日プレゼント予約・配送サービス.
          All rights reserved.
        </p>
      </footer>
    </div>
  );
};
