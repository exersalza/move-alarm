mod sql;

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Mutex;

use axum::response::Html;
use axum::{http::StatusCode, Json, response::IntoResponse, routing::{get, Router}};
use axum::extract::Path;
use axum::routing::{delete, post};
use clap::Parser;
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};

use tokio::fs;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use crate::sql::Sql;

lazy_static!{
    pub static ref SQL: Mutex<Sql> = Mutex::new(Sql::new());
}

#[derive(Parser, Debug)]
#[clap(name = "server", about = "a randomly spawned server")]
struct Opt {
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,

    /// Set the listen address
    #[clap(short = 'a', long = "addr", default_value = "localhost")]
    addr: String,

    /// Set the port
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,

    /// set the static dir
    #[clap(long = "static-dir", default_value = "./dist")]
    static_dir: String,

    /// define config path
    #[clap(short = 'c', long = "config", default_value = "./config.toml")]
    config: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }

    drop(SQL.lock().expect("random"));


    let cors = CorsLayer::new()
        .allow_methods(Any) // Allow all methods
        .allow_origin(Any) // Allow all origins
        .allow_headers(Any) // Allow all headers
        .expose_headers(Any);

    // enable consolel logging
    tracing_subscriber::fmt::init();

    let app: Router = Router::new()
        // web endpoints
        .route("/api/web/config", post(post_config))
        .route("/api/web/config/:username", get(get_config))
        .route("/api/user", get(get_all_user))
        .route("/api/user/:username", delete(delete_user))
        .route("/api/user", post(update_user))
        .layer(cors)
        .fallback_service(get(|req| async move {
            let res = ServeDir::new(&opt.static_dir).oneshot(req).await.unwrap(); // serve dir is infallible
            let status = res.status();
            match status {
                StatusCode::NOT_FOUND => {
                    let index_path = PathBuf::from(&opt.static_dir).join("index.html");
                    fs::read_to_string(index_path)
                        .await
                        .map(|index_content| (StatusCode::OK, Html(index_content)).into_response())
                        .unwrap_or_else(|_| {
                            (StatusCode::INTERNAL_SERVER_ERROR, "index.html not found")
                                .into_response()
                        })
                }

                _ => res.into_response(),
            }
        }))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST)),
        opt.port,
    ));

    log::info!("Web listening on http://{addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("Unable to start server");

    log::info!("test");
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    username: String,
    path: String,
}


async fn post_config(Json(data): Json<Config>) -> impl IntoResponse {
    SQL.lock().expect("some lock not working").update_user(data);
    Json::from(HashMap::<String, String>::new())
}

async fn get_all_user() -> impl IntoResponse {
    let sql = SQL.lock().expect("can't lock");


    Json::from(sql.get_all_users())
}

async fn update_user(Json(data): Json<Config>) -> impl IntoResponse {
    SQL.lock().expect("some lock not working").update_user(data);
    Json::from(())
}

async fn delete_user(Path(user): Path<String>) -> impl IntoResponse {
    SQL.lock().expect("threading is fun").delete_user(user);
    Json::from(())
}

async fn get_config(Path(_user): Path<String>) -> impl IntoResponse {
    todo!()
}