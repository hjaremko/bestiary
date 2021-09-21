use crate::repo;
use sqlx::SqlitePool;
use tide::{Body, Request, Response, Server};

pub fn bind_courses(server: &mut Server<()>, pool: SqlitePool) {
    server.at("/api/v1/courses").get(move |req| {
        let pool = pool.clone();
        all_courses(req, pool)
    });
}

async fn all_courses(_req: Request<()>, pool: SqlitePool) -> tide::Result {
    let courses = repo::courses::get_all(pool).await?;
    let mut req = Response::new(tide::StatusCode::Ok);
    req.set_body(Body::from_json(&courses)?);
    Ok(req)
}
