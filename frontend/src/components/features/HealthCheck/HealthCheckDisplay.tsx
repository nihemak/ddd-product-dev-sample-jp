// @ts-nocheck
import React from 'react';
('use client'); // Next.js App Router でクライアントコンポーネントとしてマーク

import { useHealthCheck } from '@/hooks/useHealthCheck';

export const HealthCheckDisplay = () => {
  const { data, error, isLoading, isFetching } = useHealthCheck();

  console.log('HealthCheckDisplay data:', data);

  if (isLoading) {
    return <p>ヘルスチェックAPIを呼び出し中...</p>;
  }

  if (error) {
    // error オブジェクトの構造や型に応じて、より詳細なエラーメッセージを表示できます
    // 例: if (error instanceof Error) return <p>エラー: {error.message}</p>;
    return <p>ヘルスチェックAPIの呼び出しに失敗しました。</p>;
  }

  return (
    <div>
      <h2>ヘルスチェック結果:</h2>
      {isFetching && <p>データを再取得中...</p>}
      {/* APIレスポンスがany型なので、実際の構造に合わせて表示します */}
      {/* バックエンドは通常 {"status": "OK"} のようなJSONを返すと想定 */}
      <pre data-testid="health-check-json">{JSON.stringify(data, null, 2)}</pre>
      {/* もし data.status のように特定のプロパティを表示したい場合 */}
      {/* {data && <p>ステータス: {data.status || '不明'}</p>} */}
    </div>
  );
};
