use axum::{Router, middleware, response::Html, routing::get};
use std::path::Path;
use tower_http::services::ServeDir;
mod middlewares;

//// main page handler ////
async fn index() -> Html<String> {
    let html_path = Path::new("static/base.html");
    let contents = tokio::fs::read_to_string(html_path)
        .await
        .unwrap_or_else(|_| "<h1>صفحه یافت نشد!</h1>".to_string());
    Html(contents)
}

//// product page handler ////
async fn products() -> Html<&'static str> {
    Html("<h1>صفحه محصولات</h1>")
}

#[tokio::main]
async fn main() {
    // Compose the routes
    let app = Router::new()
        .layer(middleware::from_fn(middlewares::logger_middleware))
        .route("/", get(index))
        .route("/products", get(products))
        .nest_service("/static", ServeDir::new("static")); // CSS و JS

    // Run the app on localhost only
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000") // <- localhost
        .await
        .unwrap();

    println!("🚀 Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
