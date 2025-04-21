use sqlx::PgPool;
use std::env;

#[tokio::test]
async fn test_db_connection() {
    // Arrange: 環境変数からデータベース接続URLを取得
    // docker-compose.yml で設定した DATABASE_URL がコンテナ内で読み込まれる想定
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Act & Assert: データベースへの接続を試みる
    // 接続プールが正常に作成できれば成功とする
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres.");

    // (任意) プールから接続を取得して簡単なクエリを実行してみる
    // sqlx::query("SELECT 1")
    //     .execute(&pool)
    //     .await
    //     .expect("Failed to execute query.");

    // ここまで到達すれば接続成功
    assert!(true);
} 