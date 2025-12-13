use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, DeleteResult};
use serde::Deserialize;

use crate::{AppState, models::pest_control::{self, PestControlStatus}};

#[derive(Deserialize)]
pub struct CreatePestRequest {
    pub name: String,
    pub status: PestControlStatus,
    pub land_id: i32, // Wajib ada land_id
}

#[derive(Deserialize)]
pub struct UpdatePestRequest {
    pub name: Option<String>,
    pub status: Option<PestControlStatus>,
}

#[post("/pest-controls")]
pub async fn create_pest_control(
    data: web::Data<AppState>,
    form: web::Json<CreatePestRequest>,
) -> impl Responder {
    let new_pest = pest_control::ActiveModel {
        name: Set(form.name.clone()),
        status: Set(form.status.clone()),
        land_id: Set(form.land_id), // Set Land ID
        ..Default::default()
    };

    match new_pest.insert(&data.db).await {
        Ok(res) => HttpResponse::Ok().json(serde_json::json!({ "success": true, "data": res })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "success": false, "error": e.to_string() })),
    }
}

// Ganti get_pest_controls biasa dengan get_pest_controls_by_land
#[get("/lands/{land_id}/pest-controls")]
pub async fn get_pest_controls_by_land(
    data: web::Data<AppState>, 
    path: web::Path<i32>
) -> impl Responder {
    let land_id = path.into_inner();
    
    // Filter berdasarkan Land ID
    match pest_control::Entity::find()
        .filter(pest_control::Column::LandId.eq(land_id))
        .all(&data.db)
        .await 
    {
        Ok(res) => HttpResponse::Ok().json(serde_json::json!({ "success": true, "data": res })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "success": false, "error": e.to_string() })),
    }
}

#[get("/pest-controls/{id}")]
pub async fn get_pest_control_by_id(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    match pest_control::Entity::find_by_id(path.into_inner()).one(&data.db).await {
        Ok(Some(res)) => HttpResponse::Ok().json(serde_json::json!({ "success": true, "data": res })),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({ "success": false, "error": "Not found" })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "success": false, "error": e.to_string() })),
    }
}

#[put("/pest-controls/{id}")]
pub async fn update_pest_control(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    form: web::Json<UpdatePestRequest>,
) -> impl Responder {
    let id = path.into_inner();
    let existing = match pest_control::Entity::find_by_id(id).one(&data.db).await {
        Ok(Some(res)) => res,
        Ok(None) => return HttpResponse::NotFound().json(serde_json::json!({ "success": false, "error": "Not found" })),
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({ "success": false, "error": e.to_string() })),
    };

    let mut active: pest_control::ActiveModel = existing.into();
    if let Some(name) = &form.name { active.name = Set(name.clone()); }
    if let Some(status) = &form.status { active.status = Set(status.clone()); }

    match active.update(&data.db).await {
        Ok(res) => HttpResponse::Ok().json(serde_json::json!({ "success": true, "data": res })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "success": false, "error": e.to_string() })),
    }
}

#[delete("/pest-controls/{id}")]
pub async fn delete_pest_control(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    match pest_control::Entity::delete_by_id(path.into_inner()).exec(&data.db).await {
        Ok(DeleteResult { rows_affected }) if rows_affected > 0 => {
            HttpResponse::Ok().json(serde_json::json!({ "success": true, "message": "Deleted" }))
        }
        Ok(_) => HttpResponse::NotFound().json(serde_json::json!({ "success": false, "error": "Not found" })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "success": false, "error": e.to_string() })),
    }
}