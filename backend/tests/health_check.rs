// クレート名を指定して spawn_app をインポート
use ddd_sample_jp::spawn_app;

// `tokio::test` マクロを使って非同期テストを実行可能にする
#[tokio::test]
async fn health_check_works() {
    // Arrange: アプリケーションをバックグラウンドで起動する
    let address = spawn_app().await;
    // HTTPクライアントを作成
    let client = reqwest::Client::new();

    // Act: /health エンドポイントにGETリクエストを送信
    let response = client
        .get(&format!("{}/health", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert: ステータスコードが 200 OK であることを確認
    assert!(response.status().is_success());
    // (任意) レスポンスボディが空であることを確認
    assert_eq!(Some(0), response.content_length());
} 