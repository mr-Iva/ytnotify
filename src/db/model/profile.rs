#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: i32,
    pub telegram_name: String,
    pub telegram_id: String,
}
