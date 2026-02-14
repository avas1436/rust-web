// Service functions for Service

pub fn create() -> String {
    "Create Service".to_string()
}

pub fn find_all() -> String {
    "Find all Services".to_string()
}

pub fn find_one(id: u32) -> String {
    format!("Find Service with id {}", id)
}

pub fn update(id: u32) -> String {
    format!("Update Service with id {}", id)
}

pub fn remove(id: u32) -> String {
    format!("Remove Service with id {}", id)
}
