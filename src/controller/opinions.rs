use crate::repo::{beasts, opinions};
use tide::{Body, Request, Response, Server};

pub fn bind_opinions(server: &mut Server<()>) {
    server.at("/api/v1/opinions").get(all_opinions);
}

async fn all_opinions(_req: Request<()>) -> tide::Result {
    let opinions = opinions::get_all().await?;
    let mut req = Response::new(tide::StatusCode::Ok);
    req.set_body(Body::from_json(&opinions)?);
    Ok(req)
}

pub fn bind_beasts(server: &mut Server<()>) {
    server.at("/api/v1/beasts")
        .get(all_beasts)
    // .post(new_beast)
    ;
    // todo
    // server.at("/api/v1/beasts/:id/").get(all_opinions);
    // server.at("/api/v1/beasts/:id/opinions").get(all_opinions);
    // server.at("/api/v1/beasts/:id/courses").get(all_opinions);
}

async fn all_beasts(_req: Request<()>) -> tide::Result {
    let beasts = beasts::get_all().await?;
    let mut req = Response::new(tide::StatusCode::Ok);
    req.set_body(Body::from_json(&beasts)?);
    Ok(req)
}
