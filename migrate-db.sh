#!/bin/bash
set -e

if [ ! -d "backend" ]; then
  echo "エラー: このスクリプトはプロジェクトのルートディレクトリから実行してください。"
  exit 1
fi

# DATABASE_URL をスクリプト内で直接定義
DATABASE_URL="postgres://app_user:password123@db:5432/app_db"
export DATABASE_URL

echo "DB サービスが起動していることを確認してください (例: docker compose ps db)"
# Optional DB check can remain here

echo "backend ディレクトリに移動します..."
cd backend

echo "データベースを作成または確認し、マイグレーションを実行します..."
(sqlx database create || echo "データベースは既に存在する可能性があります。") && sqlx migrate run
echo "マイグレーション完了。"

unset DATABASE_URL 