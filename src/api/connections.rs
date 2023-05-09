use rocket::{delete, get, patch, post};

#[get("/api/connections")]
pub fn get_all_connections() -> &'static str {
    "get_all_connections called!"
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
