use uuid::Uuid;
use warp::Filter;

use crate::{environment::Environment, handlers, models::addon::{Addon, ListOptions}};

/// The 4 addon filters combined.
pub fn filter(env: Environment) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    addons_list(env.clone())
    // .or(addons_create(env.clone()))
    // .or(addons_update(env.clone()))
    // .or(addon_delete(env))
}

/// GET /addon?offset=3&limit=5
pub fn addons_list(
    env: Environment,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("addons")
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        .and(with_env(env))
        .and_then(handlers::addon::list_addons)
}

// /// POST /addon with JSON body
// pub fn addons_create(
//     env: Environment,
// ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     warp::path!("addons")
//         .and(warp::post())
//         .and(json_body())
//         .and(with_env(env))
//         .and_then(handlers::create_addon)
// }

// /// PUT /addon/:id with JSON body
// pub fn addons_update(
//     env: Environment,
// ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     warp::path!("addons" / Uuid)
//         .and(warp::put())
//         .and(json_body())
//         .and(with_env(env))
//         .and_then(handlers::update_addon)
// }

// /// DELETE /addon/:id
// pub fn addon_delete(
//     env: Environment,
// ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     // We'll make one of our endpoints admin-only to show how authentication filters are used
//     let admin_only = warp::header::exact("authorization", "Bearer admin");

//     warp::path!("addons" / Uuid)
//         // It is important to put the auth check _after_ the path filters.
//         // If we put the auth check before, the request `PUT /addon/invalid-string`
//         // would try this filter and reject because the authorization header doesn't match,
//         // rather because the param is wrong for that other path.
//         .and(admin_only)
//         .and(warp::delete())
//         .and(with_env(env))
//         .and_then(handlers::delete_addon)
// }

fn with_env(env: Environment) -> impl Filter<Extract = (Environment,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || env.clone())
}

fn json_body() -> impl Filter<Extract = (Addon,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
