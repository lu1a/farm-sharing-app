use crate::dbrepo::db_get_farm_details;
use crate::DBRepo;
use rocket::{get, patch, response::content::RawJson};
use rocket_db_pools::Connection;

#[get("/api/details")]
pub async fn get_details(db: Connection<DBRepo>) -> RawJson<String> {
    let my_farm = db_get_farm_details(db).await.unwrap();
    RawJson(serde_json::to_string(&my_farm).unwrap())
}

#[patch("/api/details")]
pub fn patch_details() -> &'static str {
    "patch_details called!"
}
