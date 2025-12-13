use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, EntityTrait, Set, DeleteResult};
use serde::Deserialize;

use crate::{AppState, models::seed};

#[derive(Deserialize)]
pub struct CreateSeedRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateSeedRequest {
    pub name: Option<String>,
}

#[post("/seeds")]
pub async fn create_seed(
    data: web::Data<AppState>,
    form: web::Json<CreateSeedRequest>,
) -> impl Responder {
    let new_seed = seed::ActiveModel {
        name: Set(form.name.clone()),
        ..Default::default()
    };

    match new_seed.insert(&data.db).await {
        Ok(s) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "Seed created successfully",
            "data": s
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}

#[get("/seeds")]
pub async fn get_seeds(
    data: web::Data<AppState>,
) -> impl Responder {
    match seed::Entity::find().all(&data.db).await {
        Ok(seeds) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": seeds
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}

#[get("/seeds/{id}")]
pub async fn get_seed_by_id(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();

    match seed::Entity::find_by_id(id).one(&data.db).await {
        Ok(Some(s)) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": s
        })),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "error": "Seed not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}

#[put("/seeds/{id}")]
pub async fn update_seed(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    form: web::Json<UpdateSeedRequest>,
) -> impl Responder {
    let id = path.into_inner();

    let existing_seed = match seed::Entity::find_by_id(id).one(&data.db).await {
        Ok(Some(s)) => s,
        Ok(None) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "success": false,
                "error": "Seed not found"
            }))
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": format!("Database error: {:?}", e)
            }))
        }
    };

    let mut active_model: seed::ActiveModel = existing_seed.into();

    if let Some(name) = &form.name {
        active_model.name = Set(name.clone());
    }

    match active_model.update(&data.db).await {
        Ok(s) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "Seed updated successfully",
            "data": s
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}

#[delete("/seeds/{id}")]
pub async fn delete_seed(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();

    match seed::Entity::find_by_id(id).one(&data.db).await {
        Ok(Some(s)) => {
            let active_seed: seed::ActiveModel = s.into();

            match active_seed.delete(&data.db).await {
                Ok(DeleteResult { rows_affected }) if rows_affected > 0 => {
                    HttpResponse::Ok().json(serde_json::json!({
                        "success": true,
                        "message": "Seed deleted successfully"
                    }))
                }
                Ok(_) => HttpResponse::NotFound().json(serde_json::json!({
                    "success": false,
                    "error": "Seed not found or already deleted"
                })),
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "success": false,
                    "error": format!("Database error: {:?}", e)
                })),
            }
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "error": "Seed not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}