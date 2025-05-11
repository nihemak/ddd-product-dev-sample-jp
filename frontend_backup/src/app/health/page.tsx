import { HealthCheckDisplay } from '@/components/features/HealthCheck/HealthCheckDisplay';

export default function HealthPage() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div className="z-10 w-full max-w-5xl items-center justify-between font-mono text-sm lg:flex">
        <h1 className="mb-4 text-2xl font-bold">APIヘルスチェック</h1>
        <HealthCheckDisplay />
      </div>
    </main>
  );
}
