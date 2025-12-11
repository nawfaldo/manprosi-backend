use crate::models::{user, user_role, land, sensor, sensor_history};
use crate::models::sensor::SensorType;
use bcrypt::{DEFAULT_COST, hash};
use chrono::Local;
use sea_orm::sea_query::OnConflict;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};

pub async fn seed_db(db: &DatabaseConnection) -> Result<(), DbErr> {
    let admin_user_role = user_role::ActiveModel {
        name: Set("admin".to_owned()),
        ..Default::default()
    };
    let farmer_user_role = user_role::ActiveModel {
        name: Set("farmer".to_owned()),
        ..Default::default()
    };
    let consultant_user_role = user_role::ActiveModel {
        name: Set("consultant".to_owned()),
        ..Default::default()
    };

    user_role::Entity::insert_many([admin_user_role, farmer_user_role, consultant_user_role])
        .on_conflict(
            OnConflict::column(user_role::Column::Name)
                .do_nothing()
                .to_owned(),
        )
        .exec(db)
        .await?;

    let admin_role = user_role::Entity::find()
        .filter(user_role::Column::Name.eq("admin"))
        .one(db)
        .await?
        .ok_or_else(|| DbErr::Custom("Failed to find 'admin' role.".to_owned()))?;

    let farmer_role = user_role::Entity::find()
        .filter(user_role::Column::Name.eq("farmer"))
        .one(db)
        .await?
        .ok_or_else(|| DbErr::Custom("Failed to find 'farmer' role.".to_owned()))?;

    let password = "1234";
    let hashed_password = hash(password, DEFAULT_COST).expect("Failed to hash password");

    let admin_user = user::ActiveModel {
        username: Set("miracleandsleeper".to_owned()),
        password: Set(hashed_password.clone()),
        user_role_id: Set(admin_role.id),
        ..Default::default()
    };

    user::Entity::insert(admin_user)
        .on_conflict(
            OnConflict::column(user::Column::Username)
                .do_nothing()
                .to_owned(),
        )
        .exec(db)
        .await?;

    let farmer_username = "november rain";
    let farmer = match user::Entity::find().filter(user::Column::Username.eq(farmer_username)).one(db).await? {
        Some(u) => u,
        None => {
            let new_farmer = user::ActiveModel {
                username: Set(farmer_username.to_owned()),
                password: Set(hashed_password.clone()),
                user_role_id: Set(farmer_role.id),
                ..Default::default()
            };
            new_farmer.insert(db).await?
        }
    };

    let land_name = "Western Union";
    let land = match land::Entity::find()
        .filter(land::Column::LocationName.eq(land_name))
        .filter(land::Column::UserId.eq(farmer.id))
        .one(db)
        .await? 
    {
        Some(l) => l,
        None => {
            let new_land = land::ActiveModel {
                location_name: Set(land_name.to_owned()),
                size: Set(2.5), // 2.5 Hektar
                user_id: Set(farmer.id),
                ..Default::default()
            };
            new_land.insert(db).await?
        }
    };

    let sensor_name = "We Didn't Start The Fire";
    let sensor = match sensor::Entity::find()
        .filter(sensor::Column::Name.eq(sensor_name))
        .filter(sensor::Column::LandId.eq(land.id))
        .one(db)
        .await?
    {
        Some(s) => s,
        None => {
            let new_sensor = sensor::ActiveModel {
                name: Set(sensor_name.to_owned()),
                sensor_type: Set(SensorType::Temperature),
                land_id: Set(land.id),
                ..Default::default()
            };
            new_sensor.insert(db).await?
        }
    };

    let history = sensor_history::ActiveModel {
        sensor_id: Set(sensor.id),
        value: Set(28.5),
        recorded_at: Set(Local::now().naive_local()),
        ..Default::default()
    };
    
    history.insert(db).await?;

    println!("Database seeding complete (Admin & Farmer Chain Created).");
    Ok(())
}