mod db;
mod handlers;
mod models;
mod seeder;

use actix_cors::Cors;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;
use actix_web::{App, HttpServer, web};
use db::{init_db, setup_tables};
use dotenvy::dotenv;
use sea_orm::DatabaseConnection;
use seeder::seed_db;
use std::env;

struct AppState {
    db: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = init_db(&db_url)
        .await
        .expect("Failed to initialize database");

    setup_tables(&db).await.expect("Failed to create tables");

    let should_seed = env::var("SEED")
        .unwrap_or_else(|_| "false".to_string())
        .to_lowercase() == "true";

    if should_seed {
        match seed_db(&db).await {
            Ok(_) => println!("Database seeding complete."),
            Err(e) => println!("Database seeding failed: {:?}", e),
        }
    } else {
        println!("Seeding disabled by environment variable (SEED=false)");
    }

    let secret_key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE])
                    .supports_credentials(),
            )
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .app_data(web::Data::new(AppState { db: db.clone() }))
            // auth
            .service(handlers::auth::login)
            .service(handlers::auth::me)
            .service(handlers::auth::logout)
            // user
            .service(handlers::user::create_user)
            .service(handlers::user::get_users)
            .service(handlers::user::update_user)
            .service(handlers::user::get_user_by_id)
            .service(handlers::user::delete_user)
            // land
            .service(handlers::land::create_land)
            .service(handlers::land::get_user_lands)
            .service(handlers::land::get_land_by_id)
            .service(handlers::land::update_land)
            .service(handlers::land::delete_land)
            // sensor
            .service(handlers::sensor::create_sensor)
            .service(handlers::sensor::get_land_sensors)
            .service(handlers::sensor::get_sensors)
            .service(handlers::sensor::get_sensor_by_id)
            .service(handlers::sensor::update_sensor)
            .service(handlers::sensor::delete_sensor)
            // sensor history
            .service(handlers::sensor_history::get_history_by_sensor)
            .service(handlers::sensor_history::get_latest_history_by_sensor)
            // plant
            .service(handlers::plant::create_plant)
            .service(handlers::plant::get_plants_by_land)
            .service(handlers::plant::get_plant_by_id)
            .service(handlers::plant::update_plant)
            .service(handlers::plant::delete_plant)
            // valve
            .service(handlers::valve::create_valve)
            .service(handlers::valve::get_valves_by_land)
            .service(handlers::valve::get_valve_by_id)
            .service(handlers::valve::update_valve)
            .service(handlers::valve::delete_valve)
            // valve
            .service(handlers::pump::create_pump)
            .service(handlers::pump::get_pumps_by_land)
            .service(handlers::pump::get_pump_by_id)
            .service(handlers::pump::update_pump)
            .service(handlers::pump::delete_pump)
            // automation
            .service(handlers::automation::create_automation)
            .service(handlers::automation::get_automations_by_land)
            .service(handlers::automation::get_automation_by_id)
            .service(handlers::automation::update_automation)
            .service(handlers::automation::delete_automation)
            // sensor history
            .service(handlers::automation_history::get_history_by_automation)
            .service(handlers::automation_history::get_latest_history_by_automation)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
