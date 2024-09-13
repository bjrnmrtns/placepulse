use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use lib_utils::b64::b64u_encode;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let encoded_data = b64u_encode([0, 1, 2, 3]);
    println!("{}", encoded_data);
    let app = Router::new().route("/", get(root)).route("/users", post(create_user));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello World!"
}

async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
