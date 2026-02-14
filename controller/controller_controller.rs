use axum::{extract::Path, http::StatusCode, Json};

// Create a controller
pub async fn create() -> (StatusCode, String) {
    (StatusCode::CREATED, "Create Controller".to_string())
}

// Get all controllers
pub async fn find_all() -> (StatusCode, String) {
    (StatusCode::OK, "Find all Controllers".to_string())
}

// Get one controller by ID
pub async fn find_one(Path(id): Path<u32>) -> (StatusCode, String) {
    (StatusCode::OK, format!("Find Controller with id {}", id))
}

// Update a controller by ID
pub async fn update(Path(id): Path<u32>) -> (StatusCode, String) {
    (StatusCode::OK, format!("Update Controller with id {}", id))
}

// Remove a controller by ID
pub async fn remove(Path(id): Path<u32>) -> (StatusCode, String) {
    (StatusCode::OK, format!("Remove Controller with id {}", id))
}
