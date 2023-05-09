use rocket::{delete, get, patch, post};

#[get("/api/resources")]
pub fn get_all_resources() -> &'static str {
    "get_all_resources called!"
}

#[post("/api/resources")]
pub fn post_resource() -> &'static str {
    "post_resource called!"
}

#[get("/api/resources/<id>")]
pub fn get_resource(id: String) -> String {
    format!("get_resource called for id {}!", id)
}

#[patch("/api/resources/<id>")]
pub fn patch_resource(id: String) -> String {
    format!("patch_resource called for id {}!", id)
}

#[delete("/api/resources/<id>")]
pub fn delete_resource(id: String) -> String {
    format!("delete_resource called for id {}!", id)
}
