use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/livez", get(|| async {}));

    let port = "6000";
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
