use tide::{Request, Server};

pub fn bind_home_page(server: &mut Server<()>) {
    server.at("/").get(home_page);
}

async fn home_page(mut _req: Request<()>) -> tide::Result {
    Ok("This is UJ Bestiary".into())
}
