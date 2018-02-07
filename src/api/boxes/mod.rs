pub mod list;
pub use self::list::list;

pub mod get;
pub use self::get::get;

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Box {
    pub pipeline_key: String,
    pub creator_key: String,
    pub creation_timestamp: i64,
    pub last_updated_timestamp: i64,
    pub name: String,
    pub stage_key: String,
    //pub fields: HashMap<String, Value>,
    pub follower_keys: Vec<String>,
    pub follower_count: i32,
    pub comment_count: i32,
    pub task_total: i32,
    pub gmail_thread_count: i32,
    pub file_count: i32,
    pub box_key: String,
    pub key: String,
}
