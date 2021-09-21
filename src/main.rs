pub mod controller;
pub mod model;
pub mod repo;

use crate::controller::home_page::bind_home_page;
use crate::controller::opinions::{bind_beasts, bind_opinions};

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();

    let mut app = tide::new();
    bind_home_page(&mut app);
    bind_opinions(&mut app);
    bind_beasts(&mut app);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
