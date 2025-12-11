use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, DeleteResult};
use serde::Deserialize;

use crate::{AppState, models::pump};

#[derive(Deserialize)]
pub struct CreatePumpRequest {
    pub name: String,
    pub land_id: i32,
}

#[derive(Deserialize)]
pub struct UpdatePumpRequest {
    pub name: String,
}

#[post("/pumps")]
pub async fn create_pump(
    data: web::Data<AppState>,
    form: web::Json<CreatePumpRequest>,
) -> impl Responder {
    let new_pump = pump::ActiveModel {
        name: Set(form.name.clone()),
        land_id: Set(form.land_id),
        ..Default::default()
    };

    match new_pump.insert(&data.db).await {
        Ok(p) => HttpResponse::Ok().json(serde_json::json!({ "success": true, "data": p })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "success": false, "error": e.to_string() })),
    }
}

#[get("/lands/{land_id}/pumps")]
pub async fn get_pumps_by_land(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let land_id = path.into_inner();
    match pump::Entity::find().filter(pump::Column::LandId.eq(land_id)).all(&data.db).await {
        Ok(pumps) => HttpResponse::Ok().json(serde_json::json!({ "success": true, "data": pumps })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "success": false, "error": e.to_string() })),
    }
}

#[get("/pumps/{id}")]
pub async fn get_pump_by_id(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    match pump::Entity::find_by_id(id).one(&data.db).await {
        Ok(Some(p)) => HttpResponse::Ok().json(serde_json::json!({ "success": true, "data": p })),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({ "success": false, "error": "Pump not found" })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "success": false, "error": e.to_string() })),
    }
}

#[put("/pumps/{id}")]
pub async fn update_pump(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    form: web::Json<UpdatePumpRequest>,
) -> impl Responder {
    let id = path.into_inner();
    let existing = match pump::Entity::find_by_id(id).one(&data.db).await {
        Ok(Some(p)) => p,
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let mut active: pump::ActiveModel = existing.into();
    active.name = Set(form.name.clone());

    match active.update(&data.db).await {
        Ok(p) => HttpResponse::Ok().json(serde_json::json!({ "success": true, "data": p })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "success": false, "error": e.to_string() })),
    }
}

#[delete("/pumps/{id}")]
pub async fn delete_pump(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    match pump::Entity::delete_by_id(id).exec(&data.db).await {
        Ok(res) => {
            if res.rows_affected > 0 {
                HttpResponse::Ok().json(serde_json::json!({ "success": true }))
            } else {
                HttpResponse::NotFound().json(serde_json::json!({ "success": false, "error": "Not found" }))
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "success": false, "error": e.to_string() })),
    }
}