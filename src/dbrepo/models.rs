use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct FarmDetail {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub joined_date: NaiveDateTime,
    pub address: Option<String>,
    pub display_address: bool,
    pub primary_produce: Option<String>,
    pub website: Option<String>,
}
