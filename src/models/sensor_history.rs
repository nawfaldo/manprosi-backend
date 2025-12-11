use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "sensor_history")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub sensor_id: i32,
    pub value: f64,
    pub recorded_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::sensor::Entity",
        from = "Column::SensorId",
        to = "super::sensor::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Sensor,
}

impl Related<super::sensor::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sensor.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}