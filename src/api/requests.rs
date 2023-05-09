use rocket::{delete, get, post};

#[get("/api/requests")]
pub fn get_all_requests() -> &'static str {
    "get_all_requests called!"
}

#[post("/api/requests")]
pub fn post_request() -> &'static str {
    "post_request called!"
}

#[get("/api/requests/<id>")]
pub fn get_request(id: String) -> String {
    format!("get_request called for id {}!", id)
}

#[delete("/api/requests/<id>")]
pub fn delete_request(id: String) -> String {
    format!("delete_request called for id {}!", id)
}
