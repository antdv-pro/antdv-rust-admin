use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router};
use time::macros::format_description;
use tracing_subscriber::{
    fmt::{self, time::LocalTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

#[tokio::main]
async fn main() {
    let timer = LocalTime::new(format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second]"
    ));
    tracing_subscriber::registry()
        .with(fmt::layer().with_timer(timer))
        .init();

    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
