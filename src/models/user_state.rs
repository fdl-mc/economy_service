use crate::proto::economy::EconomyState as EconomyStateMessage;
use sqlx::PgPool;

type FetchResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(sqlx::FromRow, Default)]
pub struct UserStateModel {
    pub id: i32,
    pub user_id: i32,
    pub balance: i32,
    pub banker: bool,
}

impl UserStateModel {
    pub async fn get_by_user_id(
        user_id: i32,
        pool: &PgPool,
    ) -> FetchResult<Option<UserStateModel>> {
        Ok(
            sqlx::query_as::<_, UserStateModel>("SELECT * FROM users_states WHERE user_id = $1")
                .bind(user_id)
                .fetch_optional(pool)
                .await?,
        )
    }

    pub async fn get_by_user_id_or_create(
        user_id: i32,
        pool: &PgPool,
    ) -> FetchResult<UserStateModel> {
        Ok(sqlx::query_as::<_, UserStateModel>("INSERT INTO users_states (user_id, balance, banker) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING; SELECT * FROM users_states WHERE user_id = $1")
            .bind(user_id)
            .bind(0)
            .bind(false)
            .fetch_one(pool)
            .await?)
    }
}

impl UserStateModel {
    pub fn into_message(&self) -> EconomyStateMessage {
        EconomyStateMessage {
            user_id: self.user_id,
            balance: self.balance,
        }
    }
}
