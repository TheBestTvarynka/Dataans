/// Generates a new UUID.
pub fn gen_id() -> String {
    uuid::Uuid::new_v4().to_string()
}
