// @ts-nocheck
import { render, screen } from '@testing-library/react';
import { HealthCheckDisplay } from './HealthCheckDisplay.tsx';
import { useHealthCheck } from '@/hooks/useHealthCheck';
// import { vi } from 'vitest'; // vi はグローバルにあるはずなのでコメントアウト

// useHealthCheck フックをモック
vi.mock('@/hooks/useHealthCheck');

// useHealthCheck の型をモック用に定義 (必要に応じて調整)
type MockUseHealthCheck = typeof useHealthCheck;

describe('HealthCheckDisplay', () => {
  it('ローディング中に正しいメッセージが表示されること', () => {
    (useHealthCheck as any).mockReturnValue({
      data: null,
      error: null,
      isLoading: true,
      isFetching: false,
    });
    render(<HealthCheckDisplay />);
    expect(
      screen.getByText('ヘルスチェックAPIを呼び出し中...'),
    ).toBeInTheDocument();
  });

  it('エラー時に正しいメッセージが表示されること', () => {
    (useHealthCheck as any).mockReturnValue({
      data: null,
      error: new Error('API Error'),
      isLoading: false,
      isFetching: false,
    });
    render(<HealthCheckDisplay />);
    expect(
      screen.getByText('ヘルスチェックAPIの呼び出しに失敗しました。'),
    ).toBeInTheDocument();
  });

  it('データ取得成功時に結果が表示されること', () => {
    const mockData = { status: 'OK' };
    (useHealthCheck as any).mockReturnValue({
      data: mockData,
      error: null,
      isLoading: false,
      isFetching: false,
    });
    render(<HealthCheckDisplay />);
    expect(screen.getByText('ヘルスチェック結果:')).toBeInTheDocument();
    // JSON.stringifyされた結果を確認
    const preElement = screen.getByTestId('health-check-json');
    expect(preElement).toHaveTextContent(/\{\s*"status":\s*"OK"\s*\}/);
  });

  it('データ再取得中に正しいメッセージが表示されること', () => {
    const mockData = { status: 'OK' };
    (useHealthCheck as any).mockReturnValue({
      data: mockData,
      error: null,
      isLoading: false,
      isFetching: true,
    });
    render(<HealthCheckDisplay />);
    expect(screen.getByText('ヘルスチェック結果:')).toBeInTheDocument();
    expect(screen.getByText('データを再取得中...')).toBeInTheDocument();
    // expect(
    //   screen.getByText(JSON.stringify(mockData, null, 2)),
    // ).toBeInTheDocument();
    const preElementFetching = screen.getByTestId('health-check-json');
    expect(preElementFetching).toHaveTextContent(/\{\s*"status":\s*"OK"\s*\}/);
  });
});
