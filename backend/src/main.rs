// use anyhow::Result; // anyhowは必須ではなくなるかも
// use std::sync::Arc;
// use std::net::TcpListener; // tokio を使うため不要
use anyhow::Result;
use axum::{routing::get, Router};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::{env, net::SocketAddr, sync::Arc};
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// クレートから必要なモジュールや型をインポート (修正)
use ddd_sample_jp::{
    application::プレゼント予約サービス, infrastructure::PgRepository,
    routes::health_check::health_check,
};

// --- OpenAPI ドキュメント定義 ---
#[derive(OpenApi)]
#[openapi(
    paths(
        ddd_sample_jp::routes::health_check::health_check
    ),
    components(
        schemas(
            // TODO: スキーマを追加
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoint")
    ),
    servers(
        (url = "http://localhost:8080/api", description = "Local development server")
    ),
)]
struct ApiDoc;

// --- Main / Presentation Layer ---
#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // --- Tracing の設定 ---
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "ddd_sample_jp=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // --- DB接続 (有効化) ---
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create Postgres connection pool.");

    // --- 依存関係の構築 (DI) --- (PgRepository を使用)
    let repository = Arc::new(PgRepository::new(pool.clone()));
    let reservation_service = Arc::new(プレゼント予約サービス::new(repository));

    // --- OpenAPI ドキュメント生成 ---
    let openapi = ApiDoc::openapi();

    // --- ルーターの設定 --- (State を修正)
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi.clone()))
        .route("/api/health", get(health_check))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .layer(CorsLayer::very_permissive())
        .with_state(reservation_service);

    // --- サーバーの起動 ---
    let addr_str = env::var("LISTEN_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    let addr: SocketAddr = addr_str
        .parse()
        .expect("Invalid address format in LISTEN_ADDR");

    tracing::info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
