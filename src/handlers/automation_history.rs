use actix_web::{get, post, web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};
use serde::Deserialize;
use chrono::Local;

use crate::{AppState, models::automation_history};

// Struct request jika Anda ingin trigger manual via API
#[derive(Deserialize)]
pub struct CreateAutoHistoryRequest {
    pub automation_id: i32,
}

// Handler untuk mencatat history (Triggered)
#[post("/automation-history")]
pub async fn create_automation_history(
    data: web::Data<AppState>,
    form: web::Json<CreateAutoHistoryRequest>,
) -> impl Responder {
    let new_history = automation_history::ActiveModel {
        automation_id: Set(form.automation_id),
        triggered_at: Set(Local::now().naive_local()), // Otomatis set waktu sekarang
        ..Default::default()
    };

    match new_history.insert(&data.db).await {
        Ok(h) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": h
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}

// Handler untuk mengambil list history berdasarkan ID automation
#[get("/automations/{automation_id}/history")]
pub async fn get_history_by_automation(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let automation_id = path.into_inner();

    match automation_history::Entity::find()
        .filter(automation_history::Column::AutomationId.eq(automation_id))
        .order_by_desc(automation_history::Column::TriggeredAt) // Urutkan dari yang terbaru
        .all(&data.db)
        .await
    {
        Ok(histories) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": histories
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}

// Handler untuk mengambil kejadian TERAKHIR saja
#[get("/automations/{automation_id}/latest")]
pub async fn get_latest_history_by_automation(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let automation_id = path.into_inner();

    match automation_history::Entity::find()
        .filter(automation_history::Column::AutomationId.eq(automation_id))
        .order_by_desc(automation_history::Column::TriggeredAt)
        .one(&data.db)
        .await
    {
        Ok(Some(history)) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": history
        })),
        Ok(None) => HttpResponse::Ok().json(serde_json::json!({ // Return OK tapi null data
            "success": true,
            "data": null
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}