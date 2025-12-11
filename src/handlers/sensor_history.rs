use actix_web::{get, post, web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};
use serde::Deserialize;
use chrono::Local;

use crate::{AppState, models::sensor_history};

#[derive(Deserialize)]
pub struct CreateHistoryRequest {
    pub sensor_id: i32,
    pub value: f64,
}

#[get("/sensors/{sensor_id}/history")]
pub async fn get_history_by_sensor(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let sensor_id = path.into_inner();

    match sensor_history::Entity::find()
        .filter(sensor_history::Column::SensorId.eq(sensor_id))
        .order_by_desc(sensor_history::Column::RecordedAt)
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

#[get("/sensors/{sensor_id}/latest")]
pub async fn get_latest_history_by_sensor(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let sensor_id = path.into_inner();

    match sensor_history::Entity::find()
        .filter(sensor_history::Column::SensorId.eq(sensor_id))
        .order_by_desc(sensor_history::Column::RecordedAt)
        .one(&data.db)
        .await
    {
        Ok(Some(history)) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": history
        })),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "error": "No data recorded yet"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}