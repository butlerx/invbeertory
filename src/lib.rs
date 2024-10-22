use axum::{
    body::Body,
    extract::{Query, State},
    http::{header, Uri},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::stream::{self, StreamExt};
use std::collections::HashMap;
use tower_service::Service;
use worker::*;

mod app;
mod components;
mod formatting;
mod pages;
mod storage;

pub use app::*;

static INDEX_HTML: &str = include_str!("../dist/index.html");
static INDEX_CSS: &str = include_str!("../dist/index.css");
static CLIENT_HYDRATE_JS: &str = include_str!("../dist/client_hydrate.js");
static XKCD: &str =
    include_str!("../dist/snippets/invbeertory-c7f93bc594b6dbcb/src/components/charts/xkcd.js");

fn router() -> Router {
    let (index_html_before, index_html_after) = INDEX_HTML.split_once("<body>").unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str("<body>");
    let index_html_after = index_html_after.to_owned();

    let client_hydrate_bg_wasm = include_bytes!("../dist/client_hydrate_bg.wasm");

    return Router::new()
        .route(
            "/",
            get(|| async { ([(header::CONTENT_TYPE, "text/html")], INDEX_HTML) }),
        )
        .route(
            "/index.css",
            get(|| async { ([(header::CONTENT_TYPE, "text/css")], INDEX_CSS) }),
        )
        .route(
            "/client_hydrate.js",
            get(|| async {
                (
                    [(header::CONTENT_TYPE, "application/javascript")],
                    CLIENT_HYDRATE_JS,
                )
            }),
        )
        .route(
            "/snippets/invbeertory-c7f93bc594b6dbcb/src/components/charts/xkcd.js",
            get(|| async { ([(header::CONTENT_TYPE, "application/javascript")], XKCD) }),
        )
        .route("/client_hydrate_bg.wasm", get(client_hydrate_bg_wasm))
        .fallback(get(render).with_state((index_html_before.clone(), index_html_after.clone())));
}

async fn render(
    url: Uri,
    Query(queries): Query<HashMap<String, String>>,
    State((index_html_before, index_html_after)): State<(String, String)>,
) -> impl IntoResponse {
    let url = url.path().to_owned();

    let renderer = yew::ServerRenderer::<ServerApp>::with_props(move || ServerAppProps {
        url: url.into(),
        queries,
    });

    Body::from_stream(
        stream::once(async move { index_html_before })
            .chain(renderer.render_stream())
            .chain(stream::once(async move { index_html_after }))
            .map(Result::<_>::Ok),
    )
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}
