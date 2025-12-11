use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "plant")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub quantity: i32,
    pub land_id: i32,
    pub planted_at: DateTime,
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
}

impl Related<super::land::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Land.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}