use sea_orm::entity::prelude::*;
use serde::Serialize;
// Tambahkan 'plant' ke import super
use super::{user}; 

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "land")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub location_name: String,
    pub size: f64, 
    pub user_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "user::Entity",
        from = "Column::UserId",
        to = "user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,

    #[sea_orm(has_many = "super::sensor::Entity")]
    Sensor,

    #[sea_orm(has_many = "super::plant::Entity")]
    Plant,

    #[sea_orm(has_many = "super::valve::Entity")]
    Valve,

    #[sea_orm(has_many = "super::pump::Entity")]
    Pump,

    #[sea_orm(has_many = "super::automation::Entity")]
    Automation,
}

impl Related<user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::sensor::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sensor.def()
    }
}

impl Related<super::plant::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Plant.def()
    }
}

impl Related<super::valve::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Valve.def()
    }
}

impl Related<super::pump::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Pump.def()
    }
}

impl Related<super::automation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Automation.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}