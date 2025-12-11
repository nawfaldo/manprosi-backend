use super::land;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum SensorType {
    #[sea_orm(string_value = "Temperature")]
    Temperature,
    #[sea_orm(string_value = "Humidity")]
    Humidity,
    #[sea_orm(string_value = "SoilMoisture")]
    SoilMoisture,
    #[sea_orm(string_value = "PH")]
    PH,
    #[sea_orm(string_value = "LightIntensity")]
    LightIntensity,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "sensor")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub sensor_type: SensorType, 
    pub land_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "land::Entity",
        from = "Column::LandId",
        to = "land::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Land,

    #[sea_orm(has_many = "super::sensor_history::Entity")]
    SensorHistory,
}

impl Related<land::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Land.def()
    }
}

impl Related<super::sensor_history::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SensorHistory.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}