use axum::{
    middleware,
    routing::get,
    Router
};
mod middlewares;

#[tokio::main]
async fn main() {
    // Compose the routes
    let app = Router::new()
    .layer(middleware::from_fn(middlewares::logger_middleware))
        .route("/", get(|| async { "Hello, World!" }));

    // Run the app on localhost only
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000") // <- localhost
        .await
        .unwrap();

    println!("🚀 Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
