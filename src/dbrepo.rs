use crate::DBRepo;
use rocket_db_pools::{sqlx, Connection};
use sqlx::mysql::MySqlRow;
use sqlx::Row;

mod models;

use models::FarmDetail;

// use chrono::{NaiveDateTime, ParseError};
// use sqlx::{Decode, Error, MySql};

// impl<'r> Decode<'r, MySql> for NaiveDateTime {
//     fn decode(value: sqlx::mysql::MySqlValueRef<'r>) -> Result<Self, Error> {
//         let s = <&str as Decode<MySql>>::decode(value)?;
//         NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
//             .map_err(|err: ParseError| Error::Decode(Box::new(err)))
//     }
// }

pub async fn get_farm_details(mut db: Connection<DBRepo>) -> Result<Vec<FarmDetail>, sqlx::Error> {
    let select_query = sqlx::query(
        "SELECT id, name, description, joined_date, address, display_address, primary_produce, website FROM farm_details"
    );

    let farm_details: Vec<FarmDetail> = select_query
        .map(|row: MySqlRow| FarmDetail {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            joined_date: row.get("joined_date"),
            address: row.get("address"),
            display_address: row.get("display_address"),
            primary_produce: row.get("primary_produce"),
            website: row.get("website"),
        })
        .fetch_all(&mut *db)
        .await?;

    Ok(farm_details)
}
