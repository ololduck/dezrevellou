use crate::db::init_db;

mod db;
mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let db = init_db();
    let routes = routes::routes(db);
    log::info!("dezrevello├╣ is listening on :3000");
    warp::serve(routes).run(([0, 0, 0, 0], 3000)).await;
}
