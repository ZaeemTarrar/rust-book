#![allow(unused)]

pub use self::error::{ Error, Result };
use std::{ net::{ IpAddr, Ipv4Addr, SocketAddr } };
use axum::{
    extract::{ Path, Query },
    middleware,
    response::{ Html, IntoResponse, Response },
    routing::{ get, get_service },
    Router,
};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
mod web;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let ip = [127, 0, 0, 1];
    let port = 8080;
    let addr = SocketAddr::from((ip, port));
    println!("\n->> Listening on: {:?}\n", addr);
    let server = axum::Server::bind(&addr);
    server.serve(routes_all.into_make_service()).await.unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();
    return res;
}

fn routes_hello() -> Router {
    return Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2));
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - my_handler - {:?}", "HANDLER", params);
    let name = params.name.as_deref().unwrap_or("World!");
    let page = format!("Hello <strong> {name} !!! </strong>");
    return Html(page);
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - my_handler2 - {:?}", "HANDLER", name);
    let page = format!("Hello <strong> {name} !!! </strong>");
    return Html(page);
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./src/public/")))
}
