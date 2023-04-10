#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use env_logger::Env;
use log::info;
use rocket::fs::NamedFile;
use rocket::response::content::RawJson;
use rocket_db_pools::Connection;
use rocket_db_pools::{sqlx, Database};
use serde::Serialize;
use std::env;
use std::path::PathBuf;

mod dbrepo;

use dbrepo::get_farm_details;

#[derive(Database)]
#[database("cosmas")]
pub struct DBRepo(sqlx::MySqlPool);

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

#[get("/api/details")]
async fn get_details(db: Connection<DBRepo>) -> RawJson<String> {
    let farm_details_list = get_farm_details(db).await.unwrap();
    RawJson(serde_json::to_string(&farm_details_list).unwrap())
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("...");

    rocket::build()
        .mount("/", routes![index, favicon, health, get_details])
        .attach(DBRepo::init())
}
