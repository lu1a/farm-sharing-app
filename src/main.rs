#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use env_logger::Env;
use log::info;
use rocket::fs::NamedFile;
use rocket::response::content::RawJson;
use rocket_db_pools::{sqlx, Database};
use serde::Serialize;
use std::env;
use std::path::PathBuf;

mod dbrepo;

mod api {
    pub mod connections;
    pub mod details;
    pub mod requests;
    pub mod resources;
}

#[derive(Database)]
#[database("cosmas")]
pub struct DBRepo(sqlx::PgPool);

#[derive(Serialize)]
struct HealthResponse {
    env: String,
    status: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    let path = PathBuf::from("static/favicon.ico");
    NamedFile::open(path).await.ok()
}

#[get("/health")]
fn health() -> RawJson<String> {
    let env_type = env::var("ENVIRONMENT").unwrap_or_default();
    let response = HealthResponse {
        env: env_type,
        status: "ok".to_string(),
    };
    let json = serde_json::to_string(&response).unwrap();
    RawJson(json)
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("...");

    rocket::build()
        .mount(
            "/",
            // TODO: fix this huge list,
            // make the router auto-pick all functions in the /api files
            routes![
                index,
                favicon,
                health,
                api::details::get_details,
                api::details::patch_details,
                /*
                    ↓ Other saved farms endpoints!
                */
                api::connections::get_all_connections,
                api::connections::post_connection,
                api::connections::get_connection,
                api::connections::patch_connection,
                api::connections::delete_connection,
                /*
                    ↓ Requests made to farms endpoints!
                */
                api::requests::get_all_requests,
                api::requests::post_request,
                api::requests::get_request,
                api::requests::delete_request,
                /*
                    ↓ Resource endpoints!
                */
                api::resources::get_all_resources,
                api::resources::post_resource,
                api::resources::get_resource,
                api::resources::patch_resource,
                api::resources::delete_resource,
            ],
        )
        .attach(DBRepo::init())
}
