# 技術タスクバックログ

このファイルは、ユーザーストーリーに直接紐付かない技術的な改善タスク（環境整備、リファクタリング、依存関係更新、技術的負債返済など）を管理するためのバックログです。

## v1.1 対応目標 (例)

- [ ] chore(ci): ビルドキャッシュを導入してCI時間を短縮する #tech-debt
- [ ] refactor(domain): 注文集約内の複雑な条件分岐をポリシークラスに切り出す #refactoring
- [ ] deps: ロギングライブラリを最新バージョンに更新し、非推奨APIの呼び出し箇所を修正する #maintenance
- [x] refactor(domain): 状態を型で表現するアプローチを採用し、プレゼント予約ドメインに適用する #refactoring #architecture
- [ ] refactor(domain): 状態を型で表現するアプローチを採用し、支払いドメインに適用する #refactoring #architecture

## いつかやる (優先度 中 - 例)

- [ ] perf(infra): DB接続プールの設定を見直す #performance
- [ ] chore: リポジトリ全体のlintルールを最新化する #tech-debt

## いつかやる (優先度 低 - 例)

- [ ] docs: ADRテンプレートを導入する #documentation 