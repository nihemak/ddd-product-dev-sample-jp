use actix_web::{HttpResponse, Responder};

// GET /health リクエストに対するハンドラ
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish() // 200 OK ステータスと空のボディを返す
} 