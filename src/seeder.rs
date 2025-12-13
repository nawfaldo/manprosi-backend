use crate::models::{user, user_role, notification, land, sensor, sensor_history, plant, valve, pump, automation, automation_history, seed, recommendation, pest_control};
use crate::models::sensor::SensorType;
use bcrypt::{DEFAULT_COST, hash};
use chrono::Local;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};
use crate::models::recommendation::RecommendationType;
use crate::models::automation::AutomationType;
use crate::models::pest_control::PestControlStatus;

pub async fn seed_db(db: &DatabaseConnection) -> Result<(), DbErr> {
    // --- ROLES ---
    let admin_role = match user_role::Entity::find().filter(user_role::Column::Name.eq("admin")).one(db).await? {
        Some(r) => r,
        None => user_role::ActiveModel { name: Set("admin".to_owned()), ..Default::default() }.insert(db).await?,
    };

    let farmer_role = match user_role::Entity::find().filter(user_role::Column::Name.eq("farmer")).one(db).await? {
        Some(r) => r,
        None => user_role::ActiveModel { name: Set("farmer".to_owned()), ..Default::default() }.insert(db).await?,
    };

    let consultant_role = match user_role::Entity::find().filter(user_role::Column::Name.eq("consultant")).one(db).await? {
        Some(r) => r,
        None => user_role::ActiveModel { name: Set("consultant".to_owned()), ..Default::default() }.insert(db).await?,
    };

    let password = "1234";
    let hashed_password = hash(password, DEFAULT_COST).expect("Failed to hash password");

    // --- USERS ---
    let admin_username = "miracleandsleeper";
    if user::Entity::find().filter(user::Column::Username.eq(admin_username)).one(db).await?.is_none() {
        user::ActiveModel {
            username: Set(admin_username.to_owned()),
            password: Set(hashed_password.clone()),
            user_role_id: Set(admin_role.id),
            ..Default::default()
        }.insert(db).await?;
    }

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

    let consultant_username = "imagine";
    let _consultant = match user::Entity::find().filter(user::Column::Username.eq(consultant_username)).one(db).await? {
        Some(u) => u,
        None => {
            let new_consultant = user::ActiveModel {
                username: Set(consultant_username.to_owned()),
                password: Set(hashed_password.clone()),
                user_role_id: Set(consultant_role.id),
                ..Default::default()
            };
            new_consultant.insert(db).await?
        }
    };

    // --- SEED (Pastikan Seed dibuat SEBELUM Plant) ---
    let seed_name = "canon rock";
    let seed = match seed::Entity::find()
        .filter(seed::Column::Name.eq(seed_name))
        .one(db)
        .await?
    {
        Some(s) => s,
        None => {
            let new_seed = seed::ActiveModel {
                name: Set(seed_name.to_owned()),
                ..Default::default()
            };
            new_seed.insert(db).await?
        }
    };

    // --- LAND ---
    let land_name = "be nice 2 me";
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

    // --- SENSOR ---
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

    // --- PLANT ---
    let plant_name = "heaven and hell";
    let _plant = match plant::Entity::find()
        .filter(plant::Column::Name.eq(plant_name))
        .filter(plant::Column::LandId.eq(land.id))
        .one(db)
        .await?
    {
        Some(s) => s,
        None => {
            let new_plant = plant::ActiveModel {
                name: Set(plant_name.to_owned()),
                quantity: Set(32),
                land_id: Set(land.id),
                seed_id: Set(seed.id), // Pastikan ini ada
                planted_at: Set(Local::now().naive_local()),
                ..Default::default()
            };
            new_plant.insert(db).await?
        }
    };

    // --- VALVE ---
    let valve_name = "let it be";
    let valve = match valve::Entity::find()
        .filter(valve::Column::Name.eq(valve_name))
        .filter(valve::Column::LandId.eq(land.id))
        .one(db)
        .await?
    {
        Some(s) => s,
        None => {
            let new_valve = valve::ActiveModel {
                name: Set(valve_name.to_owned()),
                land_id: Set(land.id),
                ..Default::default()
            };
            new_valve.insert(db).await?
        }
    };

    // --- PUMP ---
    let pump_name = "One";
    let pump = match pump::Entity::find()
        .filter(pump::Column::Name.eq(pump_name))
        .filter(pump::Column::LandId.eq(land.id))
        .one(db)
        .await?
    {
        Some(s) => s,
        None => {
            let new_pump = pump::ActiveModel {
                name: Set(pump_name.to_owned()),
                land_id: Set(land.id),
                ..Default::default()
            };
            new_pump.insert(db).await?
        }
    };

    // --- AUTOMATION ---
    let automation_name = "fuwa fuwa time";
    let automation = match automation::Entity::find()
        .filter(automation::Column::Name.eq(automation_name))
        .filter(automation::Column::LandId.eq(land.id))
        .one(db)
        .await?
    {
        Some(s) => s,
        None => {
            let new_automation = automation::ActiveModel {
                name: Set(automation_name.to_owned()),
                automation_type: Set(AutomationType::Watering), // Set Type
                sensor_id: Set(sensor.id),
                sensor_value: Set(6.7),
                land_id: Set(land.id),
                pump_id: Set(pump.id),   
                valve_id: Set(valve.id), 
                dispense_amount: Set(5.0), 
                ..Default::default()
            };
            new_automation.insert(db).await?
        }
    };

    // --- AUTOMATION HISTORY ---
    let automation_history_exists = automation_history::Entity::find()
        .filter(automation_history::Column::AutomationId.eq(automation.id))
        .one(db)
        .await?;

    if automation_history_exists.is_none() {
        automation_history::ActiveModel {
            automation_id: Set(automation.id),
            triggered_at: Set(Local::now().naive_local()),
            ..Default::default()
        }
        .insert(db)
        .await?;
    }

    let rec_watering = recommendation::ActiveModel {
        name: Set("Always with me".to_owned()),
        description: Set("idk".to_owned()),
        rec_type: Set(RecommendationType::Watering),
        seed_id: Set(seed.id), // Pakai ID Corn
        ..Default::default()
    };
    recommendation::Entity::insert(rec_watering).exec(db).await?;

    let rec_fertilizer = recommendation::ActiveModel {
        name: Set("One summer day".to_owned()),
        description: Set("idk".to_owned()),
        rec_type: Set(RecommendationType::Fertilization),
        seed_id: Set(seed.id),
        ..Default::default()
    };
    recommendation::Entity::insert(rec_fertilizer).exec(db).await?;

    let rec_pest = recommendation::ActiveModel {
        name: Set("Is there still anything".to_owned()),
        description: Set("idk".to_owned()),
        rec_type: Set(RecommendationType::PestControl),
        seed_id: Set(seed.id),
        ..Default::default()
    };
    recommendation::Entity::insert(rec_pest).exec(db).await?;

    pest_control::ActiveModel {
        name: Set("come sweet death".to_owned()),
        status: Set(PestControlStatus::NoAction),
        land_id: Set(land.id),
        ..Default::default()
    }.insert(db).await?;

    notification::ActiveModel {
        user_id: Set(farmer.id),
        description: Set("coat i would buy".to_owned()),
        ..Default::default()
    }.insert(db).await?;

    println!("Database seeding complete.");
    Ok(())
}