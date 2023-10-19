#![feature(iter_map_windows)]
mod api;
mod db;

use axum::{
    routing::{get, get_service},
    Router,
};
use db::db::connect_db;
use tracing::instrument;

#[instrument()]
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let _conn = connect_db().await;

    let app = Router::new()
        .route("/api", get(|| async { "Hello, World!" }))
        .fallback(get_service(
            tower_http::services::fs::ServeDir::new("../yousofmersal_client/dist")
                .not_found_service(tower_http::services::fs::ServeFile::new(
                    "../yousofmersal_client/dist/index.html",
                )),
        ));

    tracing::info!("serving!");
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() {}

// fn setup_logging() {
//     let Ok(log_file) = File::create(format!(
//         "log_start_{}",
//         chrono::Local::now().format("%b-%-d_%-I-%M")
//     )) else {
//         panic!("Could not create file");
//     };
//     let subscriber = tracing_subscriber::fmt()
//         .with_writer(std::sync::Mutex::new(log_file))
//         .finish();
//     tracing::info!("Logging started...");
// }
