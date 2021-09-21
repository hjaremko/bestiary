pub mod opinions {
    use crate::model::Opinion;
    use sqlx::SqlitePool;
    use std::env;

    pub async fn get_all() -> tide::Result<Vec<Opinion>> {
        let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
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
}

pub mod beasts {
    use crate::model::Beast;
    use sqlx::SqlitePool;
    use std::env;

    pub async fn get_all() -> tide::Result<Vec<Beast>> {
        let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
        let rows = sqlx::query_as!(
            Beast,
            r#"
        select * from  beasts
        "#
        )
        .fetch_all(&pool)
        .await?;
        Ok(rows)
    }
}
