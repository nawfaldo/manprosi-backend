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
    pub seed_id: i32, // <--- Ditambahkan
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
    // <--- Relasi ke Seed Ditambahkan
    #[sea_orm(
        belongs_to = "super::seed::Entity",
        from = "Column::SeedId",
        to = "super::seed::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Seed,
}

impl Related<super::land::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Land.def()
    }
}

// <--- Implementasi Related ke Seed
impl Related<super::seed::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Seed.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}