use axum::http::StatusCode;
use axum::response::IntoResponse;

// utoipa の path マクロを追加
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health", // タグでグループ化 (任意)
    responses(
        (status = 200, description = "Service is healthy")
    )
)]
// GET /health リクエストに対するハンドラ (Axum 版)
pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK // Axum では StatusCode を直接返せる
} 