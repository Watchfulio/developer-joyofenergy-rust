mod datastore;
mod http;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = http::build().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
