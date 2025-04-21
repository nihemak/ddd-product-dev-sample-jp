// src/lib.rs - ライブラリのエントリーポイント

use std::net::TcpListener;
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;

// モジュール宣言 (ファイルから読み込む)
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod routes;

// domain モジュール内の要素をトップレベルに re-export してみる (問題解決のため)
// pub use domain::*;
// application や infrastructure も必要に応じて re-export 可能
// pub use application::*;
// pub use infrastructure::*;

// テスト時やmain関数からアプリケーションサーバーを起動するための関数
pub async fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // 接続プールを Arc でラップして web::Data に変換 (複数スレッドで安全に共有するため)
    let db_pool_data = web::Data::new(db_pool);
    let server = HttpServer::new(move || { // move クロージャで db_pool_data の所有権を移動
        App::new()
            // アプリケーションデータを登録
            .app_data(db_pool_data.clone()) // 各ワーカースレッド用にプールをクローン
            // /health ルートを追加し、health_checkハンドラに紐付ける
            .route("/health", web::get().to(routes::health_check::health_check))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

// テスト内でアプリケーションを起動するためのヘルパー関数
pub async fn spawn_app() -> String {
    // OSに空いているポートを割り当ててもらう
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    // 割り当てられたポート番号を取得
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    // テスト用のデータベース接続プールを作成
    // 注意: テストごとに独立したDBを使うのが理想
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create test Postgres connection pool.");

    // サーバーを別スレッド (tokioタスク) で起動
    let server = run(listener, db_pool).await.expect("Failed to bind address"); // db_pool を渡す
    let _ = tokio::spawn(server);

    // サーバーのアドレスを返す
    address
} 