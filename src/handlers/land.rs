use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, DeleteResult};
use serde::Deserialize;

use crate::{AppState, models::land};

#[derive(Deserialize)]
pub struct CreateLandRequest {
    pub location_name: String,
    pub size: f64,
    pub user_id: i32,
}

#[derive(Deserialize)]
pub struct UpdateLandRequest {
    pub location_name: Option<String>,
    pub size: Option<f64>,
    pub user_id: Option<i32>,
}

#[post("/lands")]
pub async fn create_land(
    data: web::Data<AppState>,
    form: web::Json<CreateLandRequest>,
) -> impl Responder {
    let new_land = land::ActiveModel {
        location_name: Set(form.location_name.clone()),
        size: Set(form.size),
        user_id: Set(form.user_id),
        ..Default::default()
    };

    match new_land.insert(&data.db).await {
        Ok(l) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "Land created successfully",
            "data": l
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}

#[get("/users/{user_id}/lands")]
pub async fn get_user_lands(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let user_id = path.into_inner();

    match land::Entity::find()
        .filter(land::Column::UserId.eq(user_id)) 
        .all(&data.db)
        .await 
    {
        Ok(lands) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": lands
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}

#[get("/lands/{id}")]
pub async fn get_land_by_id(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();

    match land::Entity::find_by_id(id).one(&data.db).await {
        Ok(Some(l)) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": l
        })),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "error": "Land not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}

#[put("/lands/{id}")]
pub async fn update_land(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    form: web::Json<UpdateLandRequest>,
) -> impl Responder {
    let id = path.into_inner();

    let existing_land = match land::Entity::find_by_id(id).one(&data.db).await {
        Ok(Some(l)) => l,
        Ok(None) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "success": false,
                "error": "Land not found"
            }))
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": format!("Database error: {:?}", e)
            }))
        }
    };

    let mut active_model: land::ActiveModel = existing_land.into();

    if let Some(location_name) = &form.location_name {
        active_model.location_name = Set(location_name.clone());
    }

    if let Some(size) = form.size {
        active_model.size = Set(size);
    }

    if let Some(user_id) = form.user_id {
        active_model.user_id = Set(user_id);
    }

    match active_model.update(&data.db).await {
        Ok(l) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "Land updated successfully",
            "data": l
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}

#[delete("/lands/{id}")]
pub async fn delete_land(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();

    match land::Entity::find_by_id(id).one(&data.db).await {
        Ok(Some(l)) => {
            let active_land: land::ActiveModel = l.into();

            match active_land.delete(&data.db).await {
                Ok(DeleteResult { rows_affected }) if rows_affected > 0 => {
                    HttpResponse::Ok().json(serde_json::json!({
                        "success": true,
                        "message": "Land deleted successfully"
                    }))
                }
                Ok(_) => HttpResponse::NotFound().json(serde_json::json!({
                    "success": false,
                    "error": "Land not found or already deleted"
                })),
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "success": false,
                    "error": format!("Database error: {:?}", e)
                })),
            }
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "error": "Land not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}