use actix_web::{get, web, HttpResponse, Responder};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{AppState, models::notification};

// GET /users/{user_id}/notifications
#[get("/users/{user_id}/notifications")]
pub async fn get_notifications_by_user(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let user_id = path.into_inner();

    // Cari notifikasi milik user tertentu
    match notification::Entity::find()
        .filter(notification::Column::UserId.eq(user_id))
        .all(&data.db)
        .await
    {
        Ok(notes) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": notes
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": e.to_string()
        })),
    }
}

// Opsional: GET All Notifications (Untuk Admin/Debug)
#[get("/notifications")]
pub async fn get_all_notifications(data: web::Data<AppState>) -> impl Responder {
    match notification::Entity::find().all(&data.db).await {
        Ok(notes) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": notes
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": e.to_string()
        })),
    }
}