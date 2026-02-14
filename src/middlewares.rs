use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

pub async fn logger_middleware(req: Request, next: Next) -> Response {
    println!("➡️ [logger_middleware] {} {}", req.method(), req.uri().path());
    let res = next.run(req).await;
    println!("⬅️ [logger_middleware] {}", res.status());
    res
}
