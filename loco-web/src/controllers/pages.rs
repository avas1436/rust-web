use loco_rs::prelude::*;
use serde_json::json;

/// A lightweight landing page rendered with Tera.
///
/// We keep this endpoint very small so it's easy to learn the flow:
/// 1) Axum/Loco extracts the configured `ViewEngine`.
/// 2) We pass some dynamic values to the template.
/// 3) We return an HTML response.
#[debug_handler]
async fn home(ViewEngine(v): ViewEngine<TeraView>) -> Result<Response> {
    format::render().view(
        &v,
        "pages/home.html",
        json!({
            // Values injected into template placeholders.
            "title": "Loko فارسی",
            "subtitle": "شروع سریع با یک ظاهر ساده و زیبا"
        }),
    )
}

/// Login page that includes a full auth playground UI.
///
/// The visual design matches the landing page and contains client-side forms
/// for all auth endpoints defined in `controllers/auth.rs`.
#[debug_handler]
async fn login_page(ViewEngine(v): ViewEngine<TeraView>) -> Result<Response> {
    format::render().view(
        &v,
        "pages/login.html",
        json!({
            "title": "ورود و تست احراز هویت"
        }),
    )
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(home))
        .add("/login", get(login_page))
}
