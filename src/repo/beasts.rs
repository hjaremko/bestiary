use crate::model::Beast;
use sqlx::SqlitePool;

pub async fn get_all(pool: SqlitePool) -> tide::Result<Vec<Beast>> {
    let rows = sqlx::query_as!(
        Beast,
        r#"
    select * from beasts
    "#
    )
    .fetch_all(&pool)
    .await?;
    Ok(rows)
}
