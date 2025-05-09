use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json;
use std::sync::Arc;

use crate::application::プレゼント予約サービス;
// use crate::domain::core::プレゼント予約Repository; // 不要になったので削除

// utoipa の path マクロを追加
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health", // タグでグループ化 (任意)
    responses(
        (status = 200, description = "Service is healthy"),
        (status = 503, description = "Service is unavailable (e.g., DB connection failed)") // 503 レスポンスを追加
    )
)]
// GET /health リクエストに対するハンドラ (Axum 版)
pub async fn health_check(
    State(reservation_service): State<Arc<プレゼント予約サービス>>, // State を受け取るように変更
) -> impl IntoResponse {
    tracing::debug!("Checking health..."); // デバッグログ追加

    // サービス経由でヘルスチェックを実行
    match reservation_service.check_health().await {
        Ok(_) => {
            tracing::debug!("Health check successful.");
            (StatusCode::OK, Json(serde_json::json!({"status": "OK"})))
        }
        Err(e) => {
            tracing::error!("Health check failed: {:?}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
        }
    }
}
