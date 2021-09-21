use crate::model::Course;
use sqlx::SqlitePool;

pub async fn get_all(pool: SqlitePool) -> tide::Result<Vec<Course>> {
    let rows = sqlx::query_as!(
        Course,
        r#"
    select * from courses
    "#
    )
    .fetch_all(&pool)
    .await?;
    Ok(rows)
}
