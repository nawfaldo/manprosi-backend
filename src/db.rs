use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Schema};

use crate::models::{land, sensor, user, user_role, sensor_history, plant, valve, pump, automation, automation_history};

pub async fn init_db(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    Database::connect(database_url).await
}

pub async fn setup_tables(db: &DatabaseConnection) -> Result<(), DbErr> {
    let schema = Schema::new(db.get_database_backend());
    let backend = db.get_database_backend();

    db.execute(
        backend.build(
            schema
                .create_table_from_entity(user_role::Entity)
                .if_not_exists(),
        ),
    )
    .await?;

    db.execute(
        backend.build(
            schema
                .create_table_from_entity(user::Entity)
                .if_not_exists(),
        ),
    )
    .await?;

    db.execute(
        backend.build(
            schema
                .create_table_from_entity(land::Entity)
                .if_not_exists(),
        ),
    )
    .await?;

    db.execute(
        backend.build(
            schema
                .create_table_from_entity(sensor::Entity)
                .if_not_exists(),
        ),
    )
    .await?;

    db.execute(
        backend.build(
            schema
                .create_table_from_entity(sensor_history::Entity)
                .if_not_exists(),
        ),
    )
    .await?;

    db.execute(
        backend.build(
            schema
                .create_table_from_entity(plant::Entity)
                .if_not_exists(),
        ),
    )
    .await?;

    db.execute(
        backend.build(
            schema
                .create_table_from_entity(valve::Entity)
                .if_not_exists(),
        ),
    )
    .await?;

    db.execute(
        backend.build(
            schema
                .create_table_from_entity(pump::Entity)
                .if_not_exists(),
        ),
    )
    .await?;

    db.execute(
        backend.build(
            schema
                .create_table_from_entity(automation::Entity)
                .if_not_exists(),
        ),
    )
    .await?;

    db.execute(
        backend.build(
            schema
                .create_table_from_entity(automation_history::Entity)
                .if_not_exists(),
        ),
    )
    .await?;

    Ok(())
}