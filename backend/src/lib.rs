// src/lib.rs - ライブラリのエントリーポイント

use std::net::TcpListener;
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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

// OpenAPI ドキュメントの定義
#[derive(OpenApi)]
#[openapi(
    paths(
        // ここに公開するAPIエンドポイントのハンドラ関数を追加していく
        routes::health_check::health_check,
    ),
    components(
        schemas(
            // ここにAPIで使うデータ構造 (レスポンス/リクエストボディ等) を追加していく
            // 例: crate::domain::注文, crate::domain::エラー応答
        )
    ),
    tags(
        (name = "ddd-sample-jp", description = "DDD Sample API")
    ),
    servers(
        (url = "/api/v1", description = "Local server") // APIのベースパス (任意)
    ),
)]
struct ApiDoc;

// テスト時やmain関数からアプリケーションサーバーを起動するための関数
pub async fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool_data = web::Data::new(db_pool);
    // OpenAPIドキュメントを生成
    let openapi = ApiDoc::openapi();

    let server = HttpServer::new(move || {
        App::new()
            // Swagger UI を /swagger-ui/* で提供
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()),
            )
            .app_data(db_pool_data.clone())
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