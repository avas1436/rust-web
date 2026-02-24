use loco_rs::{data, Result};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

const DATA_FILE: &str = "products/data.json";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Products {
    pub is_loaded: bool,
}

#[allow(dead_code)]
/// Reads the data from the JSON file.
///
/// # Errors
/// This function returns an error if the file cannot be read or deserialized.
pub async fn read() -> Result<Products> {
    data::load_json_file(DATA_FILE).await
}

static DATA: OnceLock<Products> = OnceLock::new();
#[allow(dead_code)]
pub fn get() -> &'static Products {
    DATA.get_or_init(|| data::load_json_file_sync(DATA_FILE).unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access() {
        assert!(&get().is_loaded);
    }
}
