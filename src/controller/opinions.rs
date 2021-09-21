use crate::repo::opinions;
use sqlx::SqlitePool;
use tide::{Body, Request, Response, Server};

pub fn bind_opinions(server: &mut Server<()>, pool: SqlitePool) {
    server.at("/api/v1/opinions").get(move |_req| {
        let pool = pool.clone();
        all_opinions(_req, pool)
    });
}

async fn all_opinions(_req: Request<()>, pool: SqlitePool) -> tide::Result {
    let opinions = opinions::get_all(pool).await?;
    let mut req = Response::new(tide::StatusCode::Ok);
    req.set_body(Body::from_json(&opinions)?);
    Ok(req)
}
