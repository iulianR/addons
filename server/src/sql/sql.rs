use sqlx::{PgPool, query_as_unchecked};

use crate::{error::Error, models::addon::Addon};

pub async fn get_all_addons(connection: &PgPool) -> Result<Vec<Addon>, Error> {
    query_as_unchecked!(
        Addon,
        "SELECT id, name FROM addons"
    )
    .fetch_all(connection)
    .await
    .map_err(|e| e.into())
}