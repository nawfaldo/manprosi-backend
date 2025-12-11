use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "automation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub sensor_id: i32,
    pub sensor_value: f64,
    pub pump_id: Option<i32>,
    pub valve_id: Option<i32>,
    pub land_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::land::Entity",
        from = "Column::LandId",
        to = "super::land::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Land,
    #[sea_orm(
        belongs_to = "super::sensor::Entity",
        from = "Column::SensorId",
        to = "super::sensor::Column::Id",
    )]
    Sensor,
    #[sea_orm(
        belongs_to = "super::pump::Entity",
        from = "Column::PumpId",
        to = "super::pump::Column::Id",
    )]
    Pump,
    #[sea_orm(
        belongs_to = "super::valve::Entity",
        from = "Column::ValveId",
        to = "super::valve::Column::Id",
    )]
    Valve,

    #[sea_orm(has_many = "super::automation_history::Entity")]
    AutomationHistory,
}

impl Related<super::land::Entity> for Entity { fn to() -> RelationDef { Relation::Land.def() } }
impl Related<super::sensor::Entity> for Entity { fn to() -> RelationDef { Relation::Sensor.def() } }
impl Related<super::pump::Entity> for Entity { fn to() -> RelationDef { Relation::Pump.def() } }
impl Related<super::valve::Entity> for Entity { fn to() -> RelationDef { Relation::Valve.def() } }

impl Related<super::automation_history::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AutomationHistory.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}