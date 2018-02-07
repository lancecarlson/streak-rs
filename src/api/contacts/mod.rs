pub mod get;
pub use self::get::get;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub team_key: String,
    pub given_name: String,
    pub family_name: String,
    pub title: Option<String>,
    pub email_addresses: Vec<String>,
    pub phone_numbers: Vec<String>,
    pub last_saved_user_key: String,
    pub creator_key: String,
    pub creation_date: u64,
    pub key: String,
    pub version_timestamp: u64,
    pub last_saved_timestamp: u64,
}
