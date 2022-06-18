use crate::proto::economy::EconomyState as EconomyStateMessage;
use sqlx::PgPool;

type FetchResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(sqlx::FromRow, Default)]
pub struct EconomyStateModel {
    pub id: i32,
    pub user_id: i32,
    pub balance: i32,
    pub banker: bool,
}

impl EconomyStateModel {
    pub async fn create_or_nothing(user_id: i32, pool: &PgPool) -> FetchResult<()> {
        let res = sqlx::query("INSERT INTO economy_states (user_id, balance, banker) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING")
            .bind(user_id)
            .bind(0)
            .bind(false)
            .execute(pool).await?;
        tracing::debug!("affected: {}", res.rows_affected());
        Ok(())
    }

    pub async fn get_by_user_id(
        user_id: i32,
        pool: &PgPool,
    ) -> FetchResult<Option<EconomyStateModel>> {
        Ok(sqlx::query_as::<_, EconomyStateModel>(
            "SELECT * FROM economy_states WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?)
    }

    pub async fn get_by_user_id_or_create(
        user_id: i32,
        pool: &PgPool,
    ) -> FetchResult<EconomyStateModel> {
        sqlx::query("INSERT INTO economy_states (user_id, balance, banker) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING")
            .bind(user_id)
            .bind(0)
            .bind(false)
            .execute(pool).await?;

        Ok(sqlx::query_as::<_, EconomyStateModel>(
            "SELECT * FROM economy_states WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?)
    }
}

impl EconomyStateModel {
    pub fn into_message(&self) -> EconomyStateMessage {
        EconomyStateMessage {
            user_id: self.user_id,
            balance: self.balance,
        }
    }
}
