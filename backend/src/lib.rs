// src/lib.rs - ライブラリのエントリーポイント

use std::net::TcpListener;
use actix_web::{dev::Server, web, App, HttpServer};

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
pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
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

    // サーバーを別スレッド (tokioタスク) で起動
    let server = run(listener).await.expect("Failed to bind address");
    let _ = tokio::spawn(server);

    // サーバーのアドレスを返す
    address
} 