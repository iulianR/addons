use warp::Filter;

use crate::{environment::Environment, handlers};

use super::with_env;

pub fn filter(
    env: Environment,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    accounts_register(env.clone())
    // .or(addons_create(env.clone()))
    // .or(addons_update(env.clone()))
    // .or(addon_delete(env))
}

pub fn accounts_register(
    env: Environment,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("accounts" / "register")
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and(with_env(env))
        .and_then(handlers::accounts::register)
}
