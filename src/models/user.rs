use super::{user_role};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub username: String,
    pub password: String,
    pub user_role_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "user_role::Entity",
        from = "Column::UserRoleId",
        to = "user_role::Column::Id"
    )]
    UserRole,

    #[sea_orm(has_many = "super::land::Entity")]
    Land,
}

impl Related<user_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserRole.def()
    }
}

impl Related<super::land::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Land.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}