use argon2::hash_encoded;
use uuid::Uuid;
use warp::http::StatusCode;

use crate::{
    environment::Environment,
    models::accounts::{Account, RegisterPost},
    sql,
};

pub async fn register(
    register: RegisterPost,
    env: Environment,
) -> Result<impl warp::Reply, warp::Rejection> {
    let config = argon2::Config::default();
    let hash = argon2::hash_encoded(
        &register.password.as_bytes(),
        Uuid::new_v4().as_bytes(),
        &config,
    )
    .map_err(crate::error::Error::Argon)?;

    let account = Account::new(register.username, register.email, hash);
    sql::accounts::create(&env.db(), &account).await?;

    let account = sql::accounts::get(&env.db(), &account.email).await?;

    Ok(warp::reply::with_status(
        warp::reply::json(&account),
        StatusCode::CREATED,
    ))
}
