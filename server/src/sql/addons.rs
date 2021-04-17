use sqlx::{postgres::PgQueryResult, query_as_unchecked, query_unchecked, PgPool};

use crate::{
    error::Error,
    models::addons::Addon,
};

pub async fn get_all(connection: &PgPool) -> Result<Vec<Addon>, Error> {
    query_as_unchecked!(Addon, "SELECT id, name FROM addons")
        .fetch_all(connection)
        .await
        .map_err(|e| e.into())
}

pub async fn insert(connection: &PgPool, addon: &Addon) -> Result<PgQueryResult, Error> {
    query_unchecked!(
        r#"
INSERT INTO addons (id, name)
    VALUES ($1, $2)
"#,
        addon.id,
        addon.name
    )
    .execute(connection)
    .await
    .map_err(|e| e.into())
}
