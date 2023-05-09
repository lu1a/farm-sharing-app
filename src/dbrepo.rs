use crate::DBRepo;
use rocket_db_pools::{sqlx, Connection};

mod models;

use models::FarmDetail;

pub async fn get_farm_details(mut db: Connection<DBRepo>) -> Result<Vec<FarmDetail>, sqlx::Error> {
    let farm_details: Vec<FarmDetail> = sqlx::query_as::<_, FarmDetail>(
        "\
                SELECT \
                id, name, description, joined_date, address, display_address, primary_produce, website \
                FROM farm_details\
            "
    ).fetch_all(&mut *db).await?;

    Ok(farm_details)
}
