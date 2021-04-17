use sqlx::{postgres::PgQueryResult, query_as_unchecked, query_unchecked, PgPool};

use crate::{
    error::Error,
    models::accounts::Account,
};

pub async fn create(connection: &PgPool, account: &Account) -> Result<PgQueryResult, Error> {
    query_unchecked!(
        r#"
INSERT INTO accounts (id, username, email, password)
    VALUES ($1, $2, $3, $4)
"#,
        account.id,
        account.username,
        account.email,
        account.password
    )
    .execute(connection)
    .await
    .map_err(|e| e.into())
}

pub async fn get(connection: &PgPool, email: &str) -> Result<Account, Error> {
    query_as_unchecked!(
        Account,
        r#"
SELECT id, username, email, password
FROM accounts
WHERE email = $1
"#,
        email
    )
    .fetch_one(connection)
    .await
    .map_err(|e| e.into())
}
