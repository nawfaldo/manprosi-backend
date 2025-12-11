use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, DeleteResult};
use serde::Deserialize;

use crate::{AppState, models::sensor::{self, SensorType}};

#[derive(Deserialize)]
pub struct CreateSensorRequest {
    pub name: String,
    pub sensor_type: String,
    pub land_id: i32,
}

#[derive(Deserialize)]
pub struct UpdateSensorRequest {
    pub name: Option<String>,
    pub sensor_type: Option<String>,
}

fn parse_sensor_type(type_str: &str) -> Result<SensorType, String> {
    match type_str {
        "Temperature" => Ok(SensorType::Temperature),
        "Humidity" => Ok(SensorType::Humidity),
        "SoilMoisture" => Ok(SensorType::SoilMoisture),
        "PH" => Ok(SensorType::PH),
        "LightIntensity" => Ok(SensorType::LightIntensity),
        _ => Err(format!("Invalid sensor type: {}", type_str)),
    }
}

#[post("/sensors")]
pub async fn create_sensor(
    data: web::Data<AppState>,
    form: web::Json<CreateSensorRequest>,
) -> impl Responder {
    let type_enum = match parse_sensor_type(&form.sensor_type) {
        Ok(t) => t,
        Err(e) => return HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": e
        })),
    };

    let new_sensor = sensor::ActiveModel {
        name: Set(form.name.clone()),
        sensor_type: Set(type_enum),
        land_id: Set(form.land_id),
        ..Default::default()
    };

    match new_sensor.insert(&data.db).await {
        Ok(s) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "Sensor created successfully",
            "data": s
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}

#[get("/sensors")]
pub async fn get_sensors(data: web::Data<AppState>) -> impl Responder {
    match sensor::Entity::find().all(&data.db).await {
        Ok(sensors) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": sensors
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}

#[get("/lands/{land_id}/sensors")]
pub async fn get_land_sensors(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let land_id = path.into_inner();

    match sensor::Entity::find()
        .filter(sensor::Column::LandId.eq(land_id))
        .all(&data.db)
        .await
    {
        Ok(sensors) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": sensors
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}

#[get("/sensors/{id}")]
pub async fn get_sensor_by_id(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();

    match sensor::Entity::find_by_id(id).one(&data.db).await {
        Ok(Some(s)) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": s
        })),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "error": "Sensor not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}

#[put("/sensors/{id}")]
pub async fn update_sensor(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    form: web::Json<UpdateSensorRequest>,
) -> impl Responder {
    let id = path.into_inner();

    let existing_sensor = match sensor::Entity::find_by_id(id).one(&data.db).await {
        Ok(Some(s)) => s,
        Ok(None) => return HttpResponse::NotFound().json(serde_json::json!({
            "success": false, "error": "Sensor not found"
        })),
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false, "error": format!("Database error: {:?}", e)
        })),
    };

    let mut active_model: sensor::ActiveModel = existing_sensor.into();

    if let Some(name) = &form.name {
        active_model.name = Set(name.clone());
    }

    if let Some(type_str) = &form.sensor_type {
        match parse_sensor_type(type_str) {
            Ok(t) => {
                active_model.sensor_type = Set(t);
            },
            Err(e) => return HttpResponse::BadRequest().json(serde_json::json!({
                "success": false, "error": e
            })),
        }
    }

    match active_model.update(&data.db).await {
        Ok(s) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "Sensor updated successfully",
            "data": s
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}

#[delete("/sensors/{id}")]
pub async fn delete_sensor(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();

    match sensor::Entity::find_by_id(id).one(&data.db).await {
        Ok(Some(s)) => {
            let active_sensor: sensor::ActiveModel = s.into();
            match active_sensor.delete(&data.db).await {
                Ok(DeleteResult { rows_affected }) if rows_affected > 0 => {
                    HttpResponse::Ok().json(serde_json::json!({
                        "success": true,
                        "message": "Sensor deleted successfully"
                    }))
                }
                Ok(_) => HttpResponse::NotFound().json(serde_json::json!({
                    "success": false, "error": "Sensor not found or already deleted"
                })),
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "success": false, "error": format!("Database error: {:?}", e)
                })),
            }
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "success": false, "error": "Sensor not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false, "error": format!("Database error: {:?}", e)
        })),
    }
}