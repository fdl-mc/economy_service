use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Economy state of user
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "economy_states")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip)]
    pub id: i32,

    #[serde(skip)]
    pub user_id: i32,

    /// Balance of user
    pub balance: i32,

    /// Whether the user has banker permissions
    pub banker: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
