use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Text")] 
pub enum RecommendationType {
    #[sea_orm(string_value = "Watering")]
    Watering,
    #[sea_orm(string_value = "Fertilization")]
    Fertilization,
    #[sea_orm(string_value = "PestControl")]
    PestControl,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "recommendation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    
    #[sea_orm(column_name = "type")]
    pub rec_type: RecommendationType,

    // UBAH JADI INT (Foreign Key)
    pub seed_id: i32, 
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // Relasi ke tabel Seed
    #[sea_orm(
        belongs_to = "super::seed::Entity",
        from = "Column::SeedId",
        to = "super::seed::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Seed,
}

// Implementasi Related agar bisa join
impl Related<super::seed::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Seed.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}