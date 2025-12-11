mod db;
mod handlers;
mod models;
mod seeder;

use actix_cors::Cors;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::web::{self, ServiceConfig};
use db::setup_tables; // Pastikan fungsi setup_tables Anda kompatibel dengan Postgres
use sea_orm::{Database, DatabaseConnection};
use seeder::seed_db;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;

struct AppState {
    db: DatabaseConnection,
}

#[shuttle_runtime::main]
async fn main(
    // 1. Shuttle otomatis membuatkan database Postgres & memberikan connection string
    #[shuttle_shared_db::Postgres] conn_str: String,
    // 2. Shuttle mengelola environment variables via Secrets
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    
    // --- DATABASE CONNECTION ---
    // Kita langsung connect ke Postgres menggunakan string dari Shuttle
    // Tidak perlu lagi memanggil init_db yang buat file sqlite
    let db: DatabaseConnection = Database::connect(&conn_str)
        .await
        .expect("Failed to connect to Shuttle DB");

    // --- MIGRATION / TABLE SETUP ---
    // Pastikan fungsi ini menggunakan syntax SeaORM generic, bukan raw SQL SQLite
    setup_tables(&db).await.expect("Failed to create tables");

    // --- SEEDING ---
    let should_seed = secrets.get("SEED")
        .unwrap_or_else(|| "false".to_string())
        .to_lowercase() == "true";

    if should_seed {
        match seed_db(&db).await {
            Ok(_) => println!("Database seeding complete."),
            Err(e) => println!("Database seeding failed: {:?}", e),
        }
    }

    // --- SESSION KEY ---
    // Ambil key dari Secrets.toml, atau generate random jika tidak ada (untuk dev)
    let secret_key_str = secrets.get("SESSION_KEY").unwrap_or_else(|| "0".repeat(64));
    let secret_key = Key::from(secret_key_str.as_bytes());

    let state = web::Data::new(AppState { db: db.clone() });

    // --- SERVICE CONFIG ---
    // Shuttle menggunakan factory function untuk config, bukan HttpServer::new().bind()
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("")
                .wrap(
                    Cors::default()
                        .allowed_origin("http://localhost:3000") // Local frontend
                        .allowed_origin("https://nama-frontend-anda.vercel.app") // Nanti ganti dengan URL Vercel
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE])
                        .supports_credentials(),
                )
                .wrap(SessionMiddleware::new(
                    CookieSessionStore::default(),
                    secret_key.clone(),
                ))
                .app_data(state)
                // --- DAFTAR SERVICE ANDA ---
                .service(handlers::auth::login)
                .service(handlers::auth::me)
                .service(handlers::auth::logout)
                // User
                .service(handlers::user::create_user)
                .service(handlers::user::get_users)
                .service(handlers::user::update_user)
                .service(handlers::user::get_user_by_id)
                .service(handlers::user::delete_user)
                // Land
                .service(handlers::land::create_land)
                .service(handlers::land::get_user_lands)
                .service(handlers::land::get_land_by_id)
                .service(handlers::land::update_land)
                .service(handlers::land::delete_land)
                // Sensor
                .service(handlers::sensor::create_sensor)
                .service(handlers::sensor::get_land_sensors)
                .service(handlers::sensor::get_sensors)
                .service(handlers::sensor::get_sensor_by_id)
                .service(handlers::sensor::update_sensor)
                .service(handlers::sensor::delete_sensor)
                // Sensor History
                .service(handlers::sensor_history::get_history_by_sensor)
                .service(handlers::sensor_history::get_latest_history_by_sensor)
                // Plant
                .service(handlers::plant::create_plant)
                .service(handlers::plant::get_plants_by_land)
                .service(handlers::plant::get_plant_by_id)
                .service(handlers::plant::update_plant)
                .service(handlers::plant::delete_plant)
                // Valve
                .service(handlers::valve::create_valve)
                .service(handlers::valve::get_valves_by_land)
                .service(handlers::valve::get_valve_by_id)
                .service(handlers::valve::update_valve)
                .service(handlers::valve::delete_valve)
                // Pump
                .service(handlers::pump::create_pump)
                .service(handlers::pump::get_pumps_by_land)
                .service(handlers::pump::get_pump_by_id)
                .service(handlers::pump::update_pump)
                .service(handlers::pump::delete_pump)
                // Automation
                .service(handlers::automation::create_automation)
                .service(handlers::automation::get_automations_by_land)
                .service(handlers::automation::get_automation_by_id)
                .service(handlers::automation::update_automation)
                .service(handlers::automation::delete_automation)
                // Automation History
                .service(handlers::automation_history::get_history_by_automation)
                .service(handlers::automation_history::get_latest_history_by_automation)
        );
    };

    Ok(config.into())
}