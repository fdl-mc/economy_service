use economy_service_entity::economy_state;
use sea_orm::*;

type DbResult<T> = Result<T, DbErr>;

#[derive(Default, Copy, Clone)]
pub struct UpdateEconomyStateForm {
    pub balance: Option<i32>,
    pub banker: Option<bool>,
}

pub async fn get_or_create_economy_state(
    user_id: i32,
    conn: &DbConn,
) -> DbResult<economy_state::Model> {
    match economy_state::Entity::find()
        .filter(economy_state::Column::UserId.eq(user_id))
        .one(conn)
        .await?
    {
        Some(res) => Ok(res),
        None => {
            economy_state::ActiveModel {
                user_id: Set(user_id),
                ..Default::default()
            }
            .insert(conn)
            .await
        }
    }
}

pub async fn update_economy_state(
    mut state: economy_state::ActiveModel,
    form: UpdateEconomyStateForm,
    conn: &DbConn,
) -> DbResult<economy_state::Model> {
    if form.balance.is_some() {
        state.balance = Set(form.balance.unwrap());
    }
    if form.banker.is_some() {
        state.banker = Set(form.banker.unwrap());
    }

    state.update(conn).await
}
