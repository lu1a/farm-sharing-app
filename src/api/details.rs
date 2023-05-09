use crate::dbrepo::get_farm_details;
use crate::DBRepo;
use rocket::{get, patch, response::content::RawJson};
use rocket_db_pools::Connection;

#[get("/api/details")]
pub async fn get_details(db: Connection<DBRepo>) -> RawJson<String> {
    let farm_details_list = get_farm_details(db).await.unwrap();
    RawJson(serde_json::to_string(&farm_details_list).unwrap())
}

#[patch("/api/details")]
pub fn patch_details() -> &'static str {
    "patch_details called!"
}
