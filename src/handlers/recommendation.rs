use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, EntityTrait, Set, DeleteResult};
use serde::Deserialize;

use crate::{AppState, models::recommendation::{self, RecommendationType}};

// Struct Create: seed_id wajib i32
#[derive(Deserialize)]
pub struct CreateRecRequest {
    pub name: String,
    pub description: String,
    pub rec_type: RecommendationType,
    pub seed_id: i32, // Ubah jadi i32
}

// Struct Update
#[derive(Deserialize)]
pub struct UpdateRecRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub rec_type: Option<RecommendationType>,
    pub seed_id: Option<i32>, // Ubah jadi i32
}

#[post("/recommendations")]
pub async fn create_recommendation(
    data: web::Data<AppState>,
    form: web::Json<CreateRecRequest>,
) -> impl Responder {
    
    let new_rec = recommendation::ActiveModel {
        name: Set(form.name.clone()),
        description: Set(form.description.clone()),
        rec_type: Set(form.rec_type.clone()),
        seed_id: Set(form.seed_id), // Set ID
        ..Default::default()
    };

    match new_rec.insert(&data.db).await {
        Ok(rec) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "Recommendation created successfully",
            "data": rec
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}

#[get("/recommendations")]
pub async fn get_recommendations(data: web::Data<AppState>) -> impl Responder {
    // Optional: Bisa tambah .find().find_with_related(seed::Entity) jika ingin return data seed juga
    match recommendation::Entity::find().all(&data.db).await {
        Ok(recs) => HttpResponse::Ok().json(serde_json::json!({ "success": true, "data": recs })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "success": false, "error": format!("{:?}", e) })),
    }
}

#[get("/recommendations/{id}")]
pub async fn get_recommendation_by_id(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    match recommendation::Entity::find_by_id(path.into_inner()).one(&data.db).await {
        Ok(Some(rec)) => HttpResponse::Ok().json(serde_json::json!({ "success": true, "data": rec })),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({ "success": false, "error": "Not found" })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "success": false, "error": format!("{:?}", e) })),
    }
}

#[put("/recommendations/{id}")]
pub async fn update_recommendation(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    form: web::Json<UpdateRecRequest>,
) -> impl Responder {
    let id = path.into_inner();

    let existing_rec = match recommendation::Entity::find_by_id(id).one(&data.db).await {
        Ok(Some(rec)) => rec,
        Ok(None) => return HttpResponse::NotFound().json(serde_json::json!({"success": false, "error": "Not found"})),
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({"success": false, "error": e.to_string()})),
    };

    let mut active_model: recommendation::ActiveModel = existing_rec.into();

    if let Some(name) = &form.name { active_model.name = Set(name.clone()); }
    if let Some(desc) = &form.description { active_model.description = Set(desc.clone()); }
    if let Some(rtype) = &form.rec_type { active_model.rec_type = Set(rtype.clone()); }
    if let Some(sid) = form.seed_id { active_model.seed_id = Set(sid); } // Update ID

    match active_model.update(&data.db).await {
        Ok(rec) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "Recommendation updated successfully",
            "data": rec
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}

#[delete("/recommendations/{id}")]
pub async fn delete_recommendation(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    match recommendation::Entity::delete_by_id(path.into_inner()).exec(&data.db).await {
        Ok(DeleteResult { rows_affected }) if rows_affected > 0 => {
            HttpResponse::Ok().json(serde_json::json!({ "success": true, "message": "Deleted" }))
        }
        Ok(_) => HttpResponse::NotFound().json(serde_json::json!({ "success": false, "error": "Not found" })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "success": false, "error": format!("{:?}", e) })),
    }
}