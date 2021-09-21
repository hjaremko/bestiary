use crate::model::Opinion;
use sqlx::SqlitePool;

pub async fn get_all(pool: SqlitePool) -> tide::Result<Vec<Opinion>> {
    let rows = sqlx::query_as!(
        Opinion,
        r#"
    select * from opinions
    "#
    )
    .fetch_all(&pool)
    .await?;
    Ok(rows)
}
