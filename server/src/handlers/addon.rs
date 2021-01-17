use crate::{environment::Environment, models::addon::{Addon, ListOptions}, sql::sql::get_all_addons};

use std::{convert::Infallible, vec};
use uuid::Uuid;
use warp::{http::StatusCode, reject};
use crate::error::Error;

pub async fn list_addons(
    opts: ListOptions,
    env: Environment,
) -> Result<impl warp::Reply, warp::Rejection> {
    let addons = get_all_addons(&env.db()).await?;
    Ok(warp::reply::json(&addons))
}

// pub async fn create_addon(create: Addon, env: Environment) -> Result<impl warp::Reply, Infallible> {
//     log::debug!("create_addon: {:?}", create);

//     let mut vec = env.db.lock().await;

//     for todo in vec.iter() {
//         if todo.id == create.id {
//             log::debug!("    -> id already exists: {}", create.id);
//             // Todo with id already exists, return `400 BadRequest`.
//             return Ok(StatusCode::BAD_REQUEST);
//         }
//     }

//     vec.push(create);

//     Ok(StatusCode::CREATED)
// }

// pub async fn update_addon(
//     id: Uuid,
//     update: Addon,
//     env: Environment,
// ) -> Result<impl warp::Reply, Infallible> {
//     log::debug!("update_addon: id={}, todo={:?}", id, update);
//     let mut vec = env.db.lock().await;

//     // Look for the specified Todo...
//     for todo in vec.iter_mut() {
//         if todo.id == id {
//             *todo = update;
//             return Ok(StatusCode::OK);
//         }
//     }

//     log::debug!("    -> addon id not found!");

//     // If the for loop didn't return OK, then the ID doesn't exist...
//     Ok(StatusCode::NOT_FOUND)
// }

// pub async fn delete_addon(id: Uuid, env: Environment) -> Result<impl warp::Reply, Infallible> {
//     log::debug!("delete_addon: id={}", id);

//     let mut vec = env.db.lock().await;

//     let len = vec.len();
//     vec.retain(|addon| {
//         // Retain all Todos that aren't this id...
//         // In other words, remove all that *are* this id...
//         addon.id != id
//     });

//     // If the vec is smaller, we found and deleted a Addon!
//     let deleted = vec.len() != len;

//     if deleted {
//         // respond with a `204 No Content`, which means successful,
//         // yet no body expected...
//         Ok(StatusCode::NO_CONTENT)
//     } else {
//         log::debug!("    -> todo id not found!");
//         Ok(StatusCode::NOT_FOUND)
//     }
// }
