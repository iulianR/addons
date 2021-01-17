mod environment;
mod error;
mod filters;
mod handlers;
mod models;
mod sql;

use environment::Environment;
use hyper::Server;
use listenfd::ListenFd;
use sqlx::postgres::PgPoolOptions;
use std::{collections::HashMap, convert::Infallible};
use warp::Filter;

// systemfd --no-pid -s http::3030 -- cargo watch -x 'run'

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    if dotenv::dotenv().is_err() {
        eprintln!("Warning: Did not find .env file in current working directory!");
        return Ok(());
    }

    let env = Environment::new().await?;
    let cors = warp::cors()
        .allow_any_origin() // TODO change this
        .allow_methods(vec!["OPTIONS", "GET", "POST", "DELETE"]);

    let routes = filters::addon::filter(env)
        .recover(error::unpack)
        .with(cors)
        .with(warp::log("addons"));

    let svc = warp::service(routes);
    let make_svc = hyper::service::make_service_fn(|_: _| {
        // the clone is there because not all warp filters impl Copy
        let svc = svc.clone();
        async move { Ok::<_, Infallible>(svc) }
    });

    let mut listenfd = ListenFd::from_env();
    let server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        Server::from_tcp(l)?
    } else {
        Server::bind(&([127, 0, 0, 1], 3030).into())
    };

    server.serve(make_svc).await?;

    Ok(())
}
