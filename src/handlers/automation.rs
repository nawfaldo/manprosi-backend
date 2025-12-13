use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::Deserialize;
use crate::{AppState, models::automation::{self, AutomationType}}; // Import Enum

#[derive(Deserialize)]
pub struct CreateAutoRequest {
    pub name: String,
    pub automation_type: AutomationType, // Tambahkan ini
    pub sensor_id: i32,
    pub sensor_value: f64,
    pub pump_id: i32,
    pub valve_id: i32,
    pub land_id: i32,
    pub dispense_amount: f64,
}

#[derive(Deserialize)]
pub struct UpdateAutoRequest {
    pub name: String,
    pub automation_type: AutomationType, // Tambahkan ini
    pub sensor_id: i32,
    pub sensor_value: f64,
    pub pump_id: i32,
    pub valve_id: i32,
    pub dispense_amount: f64,
}

#[post("/automations")]
pub async fn create_automation(data: web::Data<AppState>, form: web::Json<CreateAutoRequest>) -> impl Responder {
    let new_auto = automation::ActiveModel {
        name: Set(form.name.clone()),
        automation_type: Set(form.automation_type.clone()), // Set Type
        sensor_id: Set(form.sensor_id),
        sensor_value: Set(form.sensor_value),
        pump_id: Set(form.pump_id),
        valve_id: Set(form.valve_id),
        land_id: Set(form.land_id),
        dispense_amount: Set(form.dispense_amount),
        ..Default::default()
    };
    match new_auto.insert(&data.db).await {
        Ok(a) => HttpResponse::Ok().json(serde_json::json!({ "success": true, "data": a })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "success": false, "error": e.to_string() })),
    }
}

// ... Get functions sama ...
#[get("/lands/{land_id}/automations")]
pub async fn get_automations_by_land(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    match automation::Entity::find().filter(automation::Column::LandId.eq(path.into_inner())).all(&data.db).await {
        Ok(res) => HttpResponse::Ok().json(serde_json::json!({ "success": true, "data": res })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "success": false, "error": e.to_string() })),
    }
}

#[get("/automations/{id}")]
pub async fn get_automation_by_id(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    match automation::Entity::find_by_id(path.into_inner()).one(&data.db).await {
        Ok(Some(a)) => HttpResponse::Ok().json(serde_json::json!({ "success": true, "data": a })),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "success": false, "error": e.to_string() })),
    }
}

#[put("/automations/{id}")]
pub async fn update_automation(data: web::Data<AppState>, path: web::Path<i32>, form: web::Json<UpdateAutoRequest>) -> impl Responder {
    let id = path.into_inner();
    let existing = match automation::Entity::find_by_id(id).one(&data.db).await {
        Ok(Some(a)) => a,
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let mut active: automation::ActiveModel = existing.into();
    active.name = Set(form.name.clone());
    active.automation_type = Set(form.automation_type.clone()); // Update Type
    active.sensor_id = Set(form.sensor_id);
    active.sensor_value = Set(form.sensor_value);
    active.pump_id = Set(form.pump_id);
    active.valve_id = Set(form.valve_id);
    active.dispense_amount = Set(form.dispense_amount);

    match active.update(&data.db).await {
        Ok(a) => HttpResponse::Ok().json(serde_json::json!({ "success": true, "data": a })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "success": false, "error": e.to_string() })),
    }
}

#[delete("/automations/{id}")]
pub async fn delete_automation(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    match automation::Entity::delete_by_id(path.into_inner()).exec(&data.db).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "success": true })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "success": false, "error": e.to_string() })),
    }
}