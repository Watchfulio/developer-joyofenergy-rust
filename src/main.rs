use developer_joyofenergy_rust::routes::build;

#[tokio::main]
async fn main() {
    // Build our application with a single route
    let app = build().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
