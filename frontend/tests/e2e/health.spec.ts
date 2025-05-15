// @ts-nocheck
import { test, expect } from '@playwright/test';

test.describe('Health Check Page', () => {
  test('ヘルスチェックページが正しく表示され、特定のテキストが含まれること', async ({
    page,
  }) => {
    // ヘルスチェックページに移動
    await page.goto('/health');

    // ページタイトルが期待通りか確認 (任意)
    await expect(page).toHaveTitle(/ヘルスチェック/);

    // 特定のテキストが存在することを確認
    // h2 タグのテキストを想定
    const heading = page.locator('h2');
    await expect(heading).toContainText('ヘルスチェック結果:');

    // または、ページ全体からテキストを検索 (より広範)
    // await expect(page.getByText('ヘルスチェック結果:')).toBeVisible();

    // API呼び出し中やエラーメッセージが表示されていないことも確認できるとより良い
    // 例: await expect(page.getByText('ヘルスチェックAPIを呼び出し中...')).not.toBeVisible();
    // 例: await expect(page.getByText('ヘルスチェックAPIの呼び出しに失敗しました。')).not.toBeVisible();
  });
});
