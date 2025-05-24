import { AuthPageLayout } from '@/components/features/Auth/index.tsx';
import Link from 'next/link';

export default function HomePageUnauthenticated() {
  return (
    <AuthPageLayout title="記念日プレゼント予約・配送サービスへようこそ">
      <div className="space-y-6 text-center">
        <p className="text-gray-700">
          大切な人への記念日プレゼント、心を込めてお届けします。
        </p>
        <div className="mt-8 flex flex-col space-y-4">
          <Link
            href="/signup" // 実際のサインアップページのパス
            className="flex w-full justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-3 text-base font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
          >
            新規登録して始める
          </Link>
          <Link
            href="/login" // 実際のログインページのパス
            className="flex w-full justify-center rounded-md border border-indigo-600 bg-white px-4 py-3 text-base font-medium text-indigo-700 shadow-sm hover:bg-indigo-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
          >
            ログイン
          </Link>
        </div>
      </div>
    </AuthPageLayout>
  );
}
