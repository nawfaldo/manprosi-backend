use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use bcrypt::hash;
use sea_orm::{ActiveModelTrait, EntityTrait, Set, DeleteResult};
use serde::Deserialize;

use crate::{AppState, models::user};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub user_role_id: i32,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub password: Option<String>,
    pub user_role_id: Option<i32>,
}

#[post("/users")]
pub async fn create_user(
    data: web::Data<AppState>,
    form: web::Json<CreateUserRequest>,
) -> impl Responder {
    let hashed_password = match hash(&form.password, 12) {
        Ok(h) => h,
        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": "Failed to hash password"
            }))
        }
    };

    let new_user = user::ActiveModel {
        username: Set(form.username.clone()),
        password: Set(hashed_password),
        user_role_id: Set(form.user_role_id),
        ..Default::default()
    };

    match new_user.insert(&data.db).await {
        Ok(u) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "User created successfully",
            "data": {
                "id": u.id,
                "username": u.username,
                "user_role_id": u.user_role_id
            }
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}

#[get("/users")]
pub async fn get_users(data: web::Data<AppState>) -> impl Responder {
    match user::Entity::find().all(&data.db).await {
        Ok(users) => {
            let result: Vec<_> = users
                .into_iter()
                .map(|u| serde_json::json!({
                    "id": u.id,
                    "username": u.username,
                    "user_role_id": u.user_role_id
                }))
                .collect();

            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "data": result
            }))
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}

#[get("/users/{id}")]
pub async fn get_user_by_id(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();

    match user::Entity::find_by_id(id).one(&data.db).await {
        Ok(Some(u)) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": {
                "id": u.id,
                "username": u.username,
                "user_role_id": u.user_role_id
            }
        })),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "error": "User not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}

#[put("/users/{id}")]
pub async fn update_user(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    form: web::Json<UpdateUserRequest>,
) -> impl Responder {
    let id = path.into_inner();

    let existing_user = match user::Entity::find_by_id(id).one(&data.db).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "success": false,
                "error": "User not found"
            }))
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": format!("Database error: {:?}", e)
            }))
        }
    };

    let mut active_model: user::ActiveModel = existing_user.into();

    if let Some(username) = &form.username {
        active_model.username = Set(username.clone());
    }

    if let Some(password) = &form.password {
        match hash(password, 12) {
            Ok(h) => active_model.password = Set(h),
            Err(_) => {
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "success": false,
                    "error": "Failed to hash password"
                }))
            }
        }
    }

    if let Some(role_id) = form.user_role_id {
        active_model.user_role_id = Set(role_id);
    }

    match active_model.update(&data.db).await {
        Ok(u) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "User updated successfully",
            "data": {
                "id": u.id,
                "username": u.username,
                "user_role_id": u.user_role_id
            }
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}

#[delete("/users/{id}")]
pub async fn delete_user(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();

    match user::Entity::find_by_id(id).one(&data.db).await {
        Ok(Some(u)) => {
            // Konversi ke ActiveModel agar bisa dihapus
            let active_user: user::ActiveModel = u.into();

            match active_user.delete(&data.db).await {
                Ok(DeleteResult { rows_affected }) if rows_affected > 0 => {
                    HttpResponse::Ok().json(serde_json::json!({
                        "success": true,
                        "message": "User deleted successfully"
                    }))
                }
                Ok(_) => HttpResponse::NotFound().json(serde_json::json!({
                    "success": false,
                    "error": "User not found or already deleted"
                })),
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "success": false,
                    "error": format!("Database error: {:?}", e)
                })),
            }
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "error": "User not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Database error: {:?}", e)
        })),
    }
}
