#!/bin/bash

# エラーが発生したらすぐにスクリプトを終了する
set -e

# スクリプトがプロジェクトルートから実行されているか確認
# (必要に応じてパス調整)
if [ ! -d "backend" ]; then
  echo "エラー: このスクリプトはプロジェクトのルートディレクトリから実行してください。"
  exit 1
fi

# DevContainer (develop service) から db サービスに接続する際の URL
DATABASE_URL="postgres://app_user:password123@db:5432/app_db"
export DATABASE_URL # 環境変数として設定

echo "DB サービスが起動していることを確認してください (例: docker compose ps db)"
# docker compose ps db || (echo "DBが起動していません。'docker compose up -d db'を実行してください。" && exit 1)

echo "backend ディレクトリに移動します..."
cd backend

# --- マイグレーション部分は削除 ---

echo "使用する DATABASE_URL: ${DATABASE_URL}" # 確認用に出力
# echo "cargo sqlx prepare --workspace を実行します..."
echo "cargo sqlx prepare --workspace -- --all-targets --tests を実行します..."
# cargo sqlx prepare を実行 (エクスポートされた DATABASE_URL を利用)
# cargo sqlx prepare --workspace
cargo sqlx prepare --workspace -- --all-targets --tests # フラグを追加

echo "cargo sqlx prepare が正常に完了しました。"

# スクリプト終了時に環境変数を解除 (任意)
unset DATABASE_URL 