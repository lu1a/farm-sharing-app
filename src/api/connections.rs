use crate::dbrepo::db_get_all_connections;
use crate::DBRepo;
use rocket::{get, patch, post, delete, response::content::RawJson};
use rocket_db_pools::Connection;

#[get("/api/connections")]
pub async fn get_all_connections(db: Connection<DBRepo>) -> RawJson<String> {
    let farm_details_list = db_get_all_connections(db).await.unwrap();
    RawJson(serde_json::to_string(&farm_details_list).unwrap())
}

#[post("/api/connections")]
pub fn post_connection() -> &'static str {
    "post_connection called!"
}

#[get("/api/connections/<id>")]
pub fn get_connection(id: String) -> String {
    format!("get_connection called for id {}!", id)
}

#[patch("/api/connections/<id>")]
pub fn patch_connection(id: String) -> String {
    format!("patch_connection called for id {}!", id)
}

#[delete("/api/connections/<id>")]
pub fn delete_connection(id: String) -> String {
    format!("delete_connection called for id {}!", id)
}
