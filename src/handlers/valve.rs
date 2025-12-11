use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, DeleteResult};
use serde::Deserialize;

use crate::{AppState, models::valve};

#[derive(Deserialize)]
pub struct CreateValveRequest {
    pub name: String,
    pub land_id: i32,
}

#[derive(Deserialize)]
pub struct UpdateValveRequest {
    pub name: String,
}

#[post("/valves")]
pub async fn create_valve(
    data: web::Data<AppState>,
    form: web::Json<CreateValveRequest>,
) -> impl Responder {
    let new_valve = valve::ActiveModel {
        name: Set(form.name.clone()),
        land_id: Set(form.land_id),
        ..Default::default()
    };

    match new_valve.insert(&data.db).await {
        Ok(v) => HttpResponse::Ok().json(serde_json::json!({ "success": true, "data": v })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "success": false, "error": e.to_string() })),
    }
}

#[get("/lands/{land_id}/valves")]
pub async fn get_valves_by_land(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let land_id = path.into_inner();
    match valve::Entity::find().filter(valve::Column::LandId.eq(land_id)).all(&data.db).await {
        Ok(valves) => HttpResponse::Ok().json(serde_json::json!({ "success": true, "data": valves })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "success": false, "error": e.to_string() })),
    }
}

#[get("/valves/{id}")]
pub async fn get_valve_by_id(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    match valve::Entity::find_by_id(id).one(&data.db).await {
        Ok(Some(v)) => HttpResponse::Ok().json(serde_json::json!({ "success": true, "data": v })),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({ "success": false, "error": "Valve not found" })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "success": false, "error": e.to_string() })),
    }
}

#[put("/valves/{id}")]
pub async fn update_valve(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    form: web::Json<UpdateValveRequest>,
) -> impl Responder {
    let id = path.into_inner();
    let existing = match valve::Entity::find_by_id(id).one(&data.db).await {
        Ok(Some(v)) => v,
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let mut active: valve::ActiveModel = existing.into();
    active.name = Set(form.name.clone());

    match active.update(&data.db).await {
        Ok(v) => HttpResponse::Ok().json(serde_json::json!({ "success": true, "data": v })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "success": false, "error": e.to_string() })),
    }
}

#[delete("/valves/{id}")]
pub async fn delete_valve(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    match valve::Entity::delete_by_id(id).exec(&data.db).await {
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