import { useQuery } from '@tanstack/react-query';
import { HealthService } from '@/lib/api/generated/services/HealthService';
// 必要に応じて、APIレスポンスの型を定義します。
// OpenAPI Generator が具体的な型を生成していない場合や、より厳密に型付けしたい場合。
// interface HealthStatus {
//   status: string;
//   // 他のフィールドがあればここに追加
// }

// API呼び出し関数
const fetchHealth = async () => {
  // HealthService.healthCheck() は CancelablePromise<any> を返す
  // 実際のレスポンスの型に合わせてキャストするか、any のまま扱う
  const response = await HealthService.healthCheck();
  // response は any 型なので、期待する構造に合わせてアクセス
  // 例: return response as HealthStatus;
  return response; // ここでは any のまま返す
};

export const useHealthCheck = () => {
  return useQuery({
    // クエリキーはReact Queryがキャッシュ管理や再取得のために使用します
    queryKey: ['healthCheck'],
    // クエリ関数はデータを取得するPromiseを返す非同期関数です
    queryFn: fetchHealth,
    // オプション:
    // staleTime: 5 * 60 * 1000, // 5分間はstaleとみなさない (再取得を抑制)
    // cacheTime: 10 * 60 * 1000, // 10分間はキャッシュを保持
  });
};
