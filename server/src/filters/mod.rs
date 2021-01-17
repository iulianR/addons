pub mod addon;


// #[cfg(test)]
// mod tests {
//     use uuid::Uuid;
//     use warp::http::StatusCode;
//     use warp::test::request;

//     use super::{
//         filters,
//         models::{self, Addon},
//     };

//     #[tokio::test]
//     async fn test_post() {
//         let db = models::blank_db();
//         let api = filters::addons(db);

//         let resp = request()
//             .method("POST")
//             .path("/addon")
//             .json(&Addon {
//                 id: Uuid::new_v4(),
//                 name: "test 1".into(),
//             })
//             .reply(&api)
//             .await;

//         assert_eq!(resp.status(), StatusCode::CREATED);
//     }

//     #[tokio::test]
//     async fn test_post_conflict() {
//         let addon1 = addon1();
//         let db = models::blank_db();
//         db.lock().await.push(addon1.clone());
//         let api = filters::addons(db);

//         let resp = request()
//             .method("POST")
//             .path("/addon")
//             .json(&addon1)
//             .reply(&api)
//             .await;

//         assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
//     }

//     #[tokio::test]
//     async fn test_put_unknown() {
//         let _ = pretty_env_logger::try_init();
//         let db = models::blank_db();
//         let api = filters::addons(db);

//         let resp = request()
//             .method("PUT")
//             .path("/addon/1")
//             .header("authorization", "Bearer admin")
//             .json(&addon1())
//             .reply(&api)
//             .await;

//         assert_eq!(resp.status(), StatusCode::NOT_FOUND);
//     }

//     fn addon1() -> Addon {
//         Addon {
//             id: Uuid::new_v4(),
//             name: "test 1".into(),
//         }
//     }
// }
