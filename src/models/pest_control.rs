use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum PestControlStatus {
    #[sea_orm(string_value = "no_action")]
    NoAction,
    #[sea_orm(string_value = "wip")]
    Wip,
    #[sea_orm(string_value = "done")]
    Done,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "pest_control")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub status: PestControlStatus,
    
    // TAMBAHAN: Land ID
    pub land_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // Relasi ke Land
    #[sea_orm(
        belongs_to = "super::land::Entity",
        from = "Column::LandId",
        to = "super::land::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Land,
}

impl Related<super::land::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Land.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}