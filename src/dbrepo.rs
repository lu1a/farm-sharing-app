use crate::DBRepo;
use rocket_db_pools::{sqlx, Connection};

mod models;

use models::Farm;

pub async fn db_get_farm_details(mut db: Connection<DBRepo>) -> Result<Farm, sqlx::Error> {
    let my_farm: Farm = sqlx::query_as::<_, Farm>(
        "\
                SELECT \
                id, name, description, joined_date, address, display_address, primary_produce, website \
                FROM farm \
                WHERE is_me = true \
                LIMIT 1 \
            "
    ).fetch_one(&mut *db).await?;

    Ok(my_farm)
}

pub async fn db_get_all_connections(mut db: Connection<DBRepo>) -> Result<Vec<Farm>, sqlx::Error> {
    let farms: Vec<Farm> = sqlx::query_as::<_, Farm>(
        "\
            SELECT \
            id, name, description, joined_date, address, display_address, primary_produce, website \
            FROM farm \
            WHERE is_me = false \
        "
    ).fetch_all(&mut *db).await?;

    Ok(farms)
}
