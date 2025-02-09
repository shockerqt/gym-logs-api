use axum::{http::StatusCode, routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    message: &'static str,
}

async fn hello_world() -> (StatusCode, Json<Response>) {
    let response = Response {
        message: "Hello, Wod;l!",
    };
    (StatusCode::OK, Json(response))
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(hello_world));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
