use crate::models::{user, user_role, land, sensor, sensor_history};
use crate::models::sensor::SensorType;
use bcrypt::{DEFAULT_COST, hash};
use chrono::Local;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};

pub async fn seed_db(db: &DatabaseConnection) -> Result<(), DbErr> {
    // 1. SEED ROLES (Cek satu per satu)
    
    // Admin Role
    let admin_role = match user_role::Entity::find().filter(user_role::Column::Name.eq("admin")).one(db).await? {
        Some(r) => r,
        None => user_role::ActiveModel { name: Set("admin".to_owned()), ..Default::default() }.insert(db).await?,
    };

    // Farmer Role
    let farmer_role = match user_role::Entity::find().filter(user_role::Column::Name.eq("farmer")).one(db).await? {
        Some(r) => r,
        None => user_role::ActiveModel { name: Set("farmer".to_owned()), ..Default::default() }.insert(db).await?,
    };

    // Consultant Role (Cek saja, tidak perlu variable return karena tidak dipakai dibawah)
    if user_role::Entity::find().filter(user_role::Column::Name.eq("consultant")).one(db).await?.is_none() {
        user_role::ActiveModel { name: Set("consultant".to_owned()), ..Default::default() }.insert(db).await?;
    }

    let password = "1234";
    let hashed_password = hash(password, DEFAULT_COST).expect("Failed to hash password");

    // 2. SEED ADMIN USER
    let admin_username = "miracleandsleeper";
    if user::Entity::find().filter(user::Column::Username.eq(admin_username)).one(db).await?.is_none() {
        user::ActiveModel {
            username: Set(admin_username.to_owned()),
            password: Set(hashed_password.clone()),
            user_role_id: Set(admin_role.id),
            ..Default::default()
        }.insert(db).await?;
    }

    // 3. SEED FARMER
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

    // 4. SEED LAND
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
                size: Set(2.5),
                user_id: Set(farmer.id),
                ..Default::default()
            };
            new_land.insert(db).await?
        }
    };

    // 5. SEED SENSOR
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

    // 6. SEED HISTORY (Cek dulu agar tidak duplikat setiap kali deploy)
    let history_exists = sensor_history::Entity::find()
        .filter(sensor_history::Column::SensorId.eq(sensor.id))
        .one(db)
        .await?;

    if history_exists.is_none() {
        sensor_history::ActiveModel {
            sensor_id: Set(sensor.id),
            value: Set(28.5),
            recorded_at: Set(Local::now().naive_local()),
            ..Default::default()
        }.insert(db).await?;
    }

    println!("Database seeding complete (Admin & Farmer Chain Created).");
    Ok(())
}