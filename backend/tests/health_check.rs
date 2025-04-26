use axum::{serve, Router};
use ddd_sample_jp::application::プレゼント予約サービス;
use ddd_sample_jp::infrastructure::InMemoryプレゼント予約Repository; // テストでは InMemory を使う
use dotenv::dotenv;
use reqwest;
 // DB接続も必要に応じて準備
use std::sync::Arc;
use tokio;

// テスト用のアプリケーションを起動し、アドレスとポートを返すヘルパー関数
async fn spawn_test_app() -> String {
    // 最初から tokio::net::TcpListener を使う
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0") // 0番ポートでランダムなポートを確保
        .await
        .expect("Failed to bind random port");
    let addr = listener.local_addr().unwrap(); // ポート番号を取得
    let address = format!("http://{}", addr);

    // .env 読み込み (テスト用 DB URL など)
    dotenv().ok();

    // テスト用のデータベース接続プールを作成 (health check では不要だが、他のテストで必要になる可能性)
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");
    // let db_pool = PgPool::connect(&database_url)
    //     .await
    //     .expect("Failed to create test Postgres connection pool.");

    // テスト用の依存関係 (InMemory リポジトリを使用)
    let repository = Arc::new(InMemoryプレゼント予約Repository::new());
    let reservation_service = Arc::new(プレゼント予約サービス::new(repository.clone()));

    // テスト用の Axum ルーター (main.rs と同様に設定)
    let app = Router::new()
        .route(
            "/health",
            axum::routing::get(|| async { axum::http::StatusCode::OK }),
        ) // health エンドポイントを定義
        // .route("/", axum::routing::get(|| async { "Hello, Test!" })) // 必要なら他のルートも
        .with_state(reservation_service); // サービスを State として渡す

    // Tokio の TcpListener はそのまま使う
    tokio::spawn(async move {
        serve(listener, app.into_make_service()) // listener を直接渡す
            .await
            .unwrap();
    });

    address
}

#[tokio::test]
async fn health_check_works() {
    // Arrange: テストアプリケーションを起動
    let address = spawn_test_app().await;
    let client = reqwest::Client::new();

    // Act: /health エンドポイントにリクエスト送信
    let response = client
        .get(&format!("{}/health", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert: ステータスコード 200 OK を確認
    assert!(response.status().is_success());
    // ボディが空であることも確認 (axum::http::StatusCode::OK は空のボディを返す)
    assert_eq!(Some(0), response.content_length());
}
