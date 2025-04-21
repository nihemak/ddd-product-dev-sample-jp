use actix_web::{HttpResponse, Responder};

// utoipa の path マクロを追加
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health", // タグでグループ化 (任意)
    responses(
        (status = 200, description = "Service is healthy")
    )
)]
// GET /health リクエストに対するハンドラ
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish() // 200 OK ステータスと空のボディを返す
} 