use actix_session::Session;
use actix_web::{HttpResponse, Responder, get, post, web};
use bcrypt::verify;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::{
    AppState,
    models::{user, user_role},
};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(
    data: web::Data<AppState>,
    session: Session,
    form: web::Json<LoginRequest>,
) -> impl Responder {
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(&form.username))
        .one(&data.db)
        .await
        .unwrap();

    if let Some(user) = user {
        if verify(&form.password, &user.password).unwrap_or(false) {
            let role = user_role::Entity::find_by_id(user.user_role_id)
                .one(&data.db)
                .await
                .unwrap()
                .map(|r| r.name)
                .unwrap_or_else(|| "user".to_string());

            session.insert("user_id", user.id).unwrap();
            return HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": "Login successful",
                "data": {
                    "id": user.id,
                    "username": user.username,
                    "role": role
                }
            }));
        }
    }

    HttpResponse::Unauthorized().json(serde_json::json!({
        "success": false,
        "error": "Invalid username or password"
    }))
}

#[get("/me")]
pub async fn me(session: Session, data: web::Data<AppState>) -> impl Responder {
    if let Some(user_id) = session.get::<i32>("user_id").unwrap() {
        if let Some((u, role)) = user::Entity::find_by_id(user_id)
            .find_also_related(user_role::Entity)
            .one(&data.db)
            .await
            .unwrap()
        {
            return HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "data": {
                    "id": u.id,
                    "username": u.username,
                    "role": role.map(|r| r.name).unwrap_or_else(|| "user".to_string()),
                }
            }));
        }
    }

    HttpResponse::Unauthorized().json(serde_json::json!({
        "success": false,
        "error": "Not authenticated"
    }))
}

#[post("/logout")]
pub async fn logout(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Logged out"
    }))
}
