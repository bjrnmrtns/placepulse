use axum::extract::Path;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use lib_utils::b64::b64u_encode;

async fn handler_hello(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 {name:?}", "HANDLER");
    Html(format!("Hello <strong>{name}</strong>"))
}

fn routes_hello() -> Router {
    Router::new().route("/hello/:name", get(handler_hello))
}

#[tokio::main]
async fn main() {
    let encoded_data = b64u_encode([0, 1, 2, 3]);
    println!("{}", encoded_data);
    let routes_all = Router::new().merge(routes_hello());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes_all).await.unwrap();
}
