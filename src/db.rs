use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Schema, Statement};

use crate::models::{land, sensor, user, user_role, sensor_history, plant, valve, notification, pump, automation, automation_history, seed, recommendation, pest_control};

pub async fn init_db(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    Database::connect(database_url).await
}

pub async fn setup_tables(db: &DatabaseConnection) -> Result<(), DbErr> {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    // ====================================================
    // 1. DROP TABLES (Raw SQL agar lebih pasti)
    // Menggunakan CASCADE agar relasi otomatis terputus
    // ====================================================
    
    // List tabel yang akan dihapus
    let tables = [
        "notification", // <--- Tambahkan ini (sebelum user dihapus)
        "automation_history",
        "automation",
        "sensor_history",
        "sensor",
        "plant",
        "valve",
        "pump",
        "land",
        "seed",
        "recommendation",
        "pest_control", 
        "user",
        "user_role"
    ];

    for table in tables {
        // Query: DROP TABLE IF EXISTS "nama_tabel" CASCADE;
        let sql = format!("DROP TABLE IF EXISTS \"{}\" CASCADE;", table);
        let stmt = Statement::from_string(builder, sql);
        db.execute(stmt).await?;
    }

    // ====================================================
    // 2. CREATE TABLES (Buat ulang dengan struktur baru)
    // ====================================================

    // Role & User
    db.execute(builder.build(schema.create_table_from_entity(user_role::Entity).if_not_exists())).await?;
    db.execute(builder.build(schema.create_table_from_entity(user::Entity).if_not_exists())).await?;

    // Seed (HARUS DIBUAT SEBELUM PLANT)
    db.execute(builder.build(schema.create_table_from_entity(seed::Entity).if_not_exists())).await?;

    // Land
    db.execute(builder.build(schema.create_table_from_entity(land::Entity).if_not_exists())).await?;

    // Components
    db.execute(builder.build(schema.create_table_from_entity(sensor::Entity).if_not_exists())).await?;
    db.execute(builder.build(schema.create_table_from_entity(sensor_history::Entity).if_not_exists())).await?;

    // Plant (Sekarang aman dibuat karena Seed dan Land sudah ada)
    db.execute(builder.build(schema.create_table_from_entity(plant::Entity).if_not_exists())).await?;

    db.execute(builder.build(schema.create_table_from_entity(valve::Entity).if_not_exists())).await?;
    db.execute(builder.build(schema.create_table_from_entity(pump::Entity).if_not_exists())).await?;

    // Automation
    db.execute(builder.build(schema.create_table_from_entity(automation::Entity).if_not_exists())).await?;
    db.execute(builder.build(schema.create_table_from_entity(automation_history::Entity).if_not_exists())).await?;

    db.execute(builder.build(schema.create_table_from_entity(recommendation::Entity).if_not_exists())).await?;

    db.execute(builder.build(schema.create_table_from_entity(pest_control::Entity).if_not_exists())).await?;

    db.execute(builder.build(schema.create_table_from_entity(notification::Entity).if_not_exists())).await?;

    Ok(())
}