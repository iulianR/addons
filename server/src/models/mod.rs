use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::reply::Response;

pub mod accounts;
pub mod addons;

#[derive(Debug, Serialize, Deserialize)]
pub struct IdResponse {
    id: Uuid,
}

impl IdResponse {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

// impl warp::reply::Reply for IdResponse {
//     fn into_response(self) -> warp::reply::Response {
//         Response::from(self)
//     }
// }
