use crate::repo::beasts;
use sqlx::SqlitePool;
use tide::{Body, Request, Response, Server};

pub fn bind_beasts(server: &mut Server<()>, pool: SqlitePool) {
    server.at("/api/v1/beasts")
        .get(
            move |req| {
                let pool = pool.clone();
                all_beasts(req, pool)
            }
            )
    // .post(new_beast)
    ;
    // todo
    // server.at("/api/v1/beasts/:id/").get(all_opinions);
    // server.at("/api/v1/beasts/:id/opinions").get(all_opinions);
    // server.at("/api/v1/beasts/:id/courses").get(all_opinions);
}

async fn all_beasts(_req: Request<()>, pool: SqlitePool) -> tide::Result {
    let beasts = beasts::get_all(pool).await?;
    let mut req = Response::new(tide::StatusCode::Ok);
    req.set_body(Body::from_json(&beasts)?);
    Ok(req)
}
