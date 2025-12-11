use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "automation_history")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub automation_id: i32,
    pub triggered_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::automation::Entity",
        from = "Column::AutomationId",
        to = "super::automation::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Automation,
}

impl Related<super::automation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Automation.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}