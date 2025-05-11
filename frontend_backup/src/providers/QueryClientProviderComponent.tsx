'use client';

import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import React, { useEffect } from 'react';
import { OpenAPI } from '@/lib/api/generated/core/OpenAPI';

// QueryClientのインスタンスを作成
// アプリケーション全体で共有されるため、コンポーネント外で一度だけ作成するのが一般的
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      // staleTime: 1000 * 60 * 5, // 5 minutes
      // refetchOnWindowFocus: false, // ウィンドウフォーカス時の自動再取得を無効化する場合
    },
  },
});

export const QueryClientProviderComponent = ({
  children,
}: {
  children: React.ReactNode;
}) => {
  useEffect(() => {
    // 環境変数などから取得することも検討できます
    //OpenAPI.BASE = 'http://backend:3000/api';
    // もしバックエンドAPIが /api プレフィックスを持つなら
    // OpenAPI.BASE = 'http://backend:3000/api'; のように設定
    console.log('OpenAPI BASE URL set to:', OpenAPI.BASE);
  }, []);

  return (
    <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  );
};
