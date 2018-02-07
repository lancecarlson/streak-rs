pub mod list;
pub use self::list::list;

pub mod get;
pub use self::get::get;

use serde_json::Value;

use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FieldValue {
    Integer(u64),
    Bool(bool),
    String(String),
    Array(Vec<Value>),
}

impl FieldValue {
    pub fn as_string(self) -> Option<String> {
        if let FieldValue::String(s) = self {
            Some(s)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Box {
    pub last_saved_timestamp: u64,
    pub pipeline_key: String,
    pub creator_key: String,
    pub creation_timestamp: u64,
    pub last_updated_timestamp: u64,
    pub last_stage_change_timestamp: u64,
    pub last_comment_timestamp: Option<u64>,
    pub total_number_of_emails: u32,
    pub total_number_of_sent_emails: u32,
    pub total_number_of_received_emails: u32,
    pub name: String,
    pub notes: Option<String>,
    pub assigned_to_sharing_entries: Vec<User>,
    pub creator_sharing_entry: User,
    pub follower_sharing_entries: Vec<User>,
    pub stage_key: String,
    pub follower_keys: Vec<String>,
    pub linked_box_keys: Vec<String>,
    pub email_addresses_auto_extracted: Vec<String>,
    pub email_addresses_blacklist: Vec<String>,
    pub email_addresses: Vec<String>,
    pub task_complete_count: u32,
    pub task_incomplete_count: u32,
    pub task_overdue_count: u32,
    //pub task_assignee_key_set: Vec<>,
    //pub overdue_task_assignee_key_set: Vec<>,
    //pub incomplete_task_assignee_key_set: Vec<>,
    //pub task_assignee_sharing_entry_set: Vec<>,
    //pub overdue_task_assignee_sharing_entry_set: Vec<>,
    //pub incomplete_task_assignee_sharing_entry_set: Vec<>,
    pub task_total: u32,
    pub call_log_count: u32,
    pub meeting_notes_count: u32,
    pub total_call_log_duration: u32,
    pub total_meeting_notes_duration: u32,
    pub follower_count: u32,
    pub comment_count: u32,
    pub gmail_thread_count: u32,
    pub file_count: u32,
    pub fields: HashMap<String, FieldValue>,
    pub box_key: String,
    pub key: String,
    pub freshness: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    display_name: String,
    full_name: String,
    email: String,
    image: String,
    user_key: String,
}
