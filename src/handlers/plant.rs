use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, DeleteResult};
use serde::Deserialize;
// Pastikan DateTime sesuai dengan definisi di model (biasanya NaiveDateTime untuk SeaORM)
use chrono::NaiveDateTime; 

use crate::{AppState, models::plant};

// Struct untuk request Create
#[derive(Deserialize)]
pub struct CreatePlantRequest {
    pub name: String,
    pub quantity: i32,
    pub land_id: i32,
    pub planted_at: NaiveDateTime, // Format JSON default: "YYYY-MM-DDTHH:MM:SS"
}

// Struct untuk request Update (semua field optional)
#[derive(Deserialize)]
pub struct UpdatePlantRequest {
    pub name: Option<String>,
    pub quantity: Option<i32>,
    pub land_id: Option<i32>,
    pub planted_at: Option<NaiveDateTime>,
}

#[post("/plants")]
pub async fn create_plant(
    data: web::Data<AppState>,
    form: web::Json<CreatePlantRequest>,
) -> impl Responder {
    let new_plant = plant::ActiveModel {
        name: Set(form.name.clone()),
        quantity: Set(form.quantity),
        land_id: Set(form.land_id),
        planted_at: Set(form.planted_at),
        ..Default::default()
    };

    match new_plant.insert(&data.db).await {
        Ok(p) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "Plant created successfully",
            "data": p
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}

#[get("/lands/{land_id}/plants")]
pub async fn get_plants_by_land(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let land_id = path.into_inner();

    match plant::Entity::find()
        .filter(plant::Column::LandId.eq(land_id))
        .all(&data.db)
        .await
    {
        Ok(plants) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": plants
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}

#[get("/plants/{id}")]
pub async fn get_plant_by_id(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();

    match plant::Entity::find_by_id(id).one(&data.db).await {
        Ok(Some(p)) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": p
        })),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "error": "Plant not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}

#[put("/plants/{id}")]
pub async fn update_plant(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    form: web::Json<UpdatePlantRequest>,
) -> impl Responder {
    let id = path.into_inner();

    // 1. Cari data lama
    let existing_plant = match plant::Entity::find_by_id(id).one(&data.db).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "success": false,
                "error": "Plant not found"
            }))
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": format!("Database error: {:?}", e)
            }))
        }
    };

    // 2. Convert ke ActiveModel
    let mut active_model: plant::ActiveModel = existing_plant.into();

    // 3. Update field jika ada di request
    if let Some(name) = &form.name {
        active_model.name = Set(name.clone());
    }

    if let Some(quantity) = form.quantity {
        active_model.quantity = Set(quantity);
    }

    if let Some(land_id) = form.land_id {
        active_model.land_id = Set(land_id);
    }

    if let Some(planted_at) = form.planted_at {
        active_model.planted_at = Set(planted_at);
    }

    // 4. Simpan perubahan
    match active_model.update(&data.db).await {
        Ok(p) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "Plant updated successfully",
            "data": p
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}

#[delete("/plants/{id}")]
pub async fn delete_plant(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();

    match plant::Entity::find_by_id(id).one(&data.db).await {
        Ok(Some(p)) => {
            let active_plant: plant::ActiveModel = p.into();

            match active_plant.delete(&data.db).await {
                Ok(DeleteResult { rows_affected }) if rows_affected > 0 => {
                    HttpResponse::Ok().json(serde_json::json!({
                        "success": true,
                        "message": "Plant deleted successfully"
                    }))
                }
                Ok(_) => HttpResponse::NotFound().json(serde_json::json!({
                    "success": false,
                    "error": "Plant not found or already deleted"
                })),
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "success": false,
                    "error": format!("Database error: {:?}", e)
                })),
            }
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "error": "Plant not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}