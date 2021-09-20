pub mod model;

use sqlx::SqlitePool;
use std::env;

use crate::model::{Beast, Course, Opinion};
use tide::{Body, Request, Response};

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();

    let mut app = tide::new();
    app.at("/api/v1/opinions").get(all_opinions);
    app.at("/").get(home_page);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn home_page(mut _req: Request<()>) -> tide::Result {
    Ok("This is UJ Bestiary".into())
}

async fn all_opinions(_req: Request<()>) -> tide::Result {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    let rows = sqlx::query!(
        r#"
select first_name, last_name, titles,  name, semester, year, opinion from opinions
join beasts b on b.id = opinions.beast_id
join courses c on c.id = opinions.course_id
        "#
    )
    .map(|row| Opinion {
        beast: Beast {
            first_name: row.first_name,
            last_name: row.last_name,
            titles: row.titles.unwrap(),
        },
        course: Course {
            name: row.name,
            semester: row.semester.to_string(),
            year: row.year.to_string(),
        },
        opinion: row.opinion.unwrap(),
    })
    .fetch_all(&pool)
    .await?;

    let mut req = Response::new(tide::StatusCode::Ok);
    let body = Body::from_json(&rows)?;
    req.set_body(body);
    Ok(req)
}
