pub mod controller;
pub mod model;
pub mod repo;

use crate::controller::beasts::bind_beasts;
use crate::controller::courses::bind_courses;
use crate::controller::home_page::bind_home_page;
use crate::controller::opinions::bind_opinions;
use sqlx::SqlitePool;
use std::env;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    let mut app = tide::new();
    bind_home_page(&mut app);
    bind_opinions(&mut app, pool.clone());
    bind_beasts(&mut app, pool.clone());
    bind_courses(&mut app, pool.clone());
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
