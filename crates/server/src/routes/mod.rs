use axum::{
    http::{StatusCode, header::CACHE_CONTROL},
    response::{Html, IntoResponse},
    routing::get,
};
use hypertext::prelude::*;

pub type Router = axum::Router;

pub fn router() -> Router {
    Router::new()
        .route("/healthcheck", get(healthcheck))
        .route("/", get(index))
}

async fn healthcheck() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

async fn index() -> impl IntoResponse {
    let page = Html(
        maud! {
            !DOCTYPE
            html lang="en" {
                head {
                    meta charset="utf-8";
                    meta name="viewport" content="width=device-width, initial-scale=1";
                    title { "Bevy App" }
                    link rel="stylesheet" href="/assets/stylesheet.css";
                }
                body {
                    div id="loading" { "Loading…" }
                }
            }
        }
        .render()
        .as_inner()
        .to_owned(),
    );
    ([(CACHE_CONTROL, "no-cache")], page).into_response()
}
