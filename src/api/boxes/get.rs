//! Box actions

use serde_json;

use error::StreakError;
use client::Client;

pub struct Box {
    pipeline_key: String,
    creator_key: String,
    creation_timestamp: i64,
    last_updated_timestamp: i64,
    name: String,
    stage_key: String,
    fields: Map<String, Value>,
    follower_keys: Vec<String>,
    follower_count: i32,
    comment_count: i32,
    task_total: i32,
    gmail_thread_count: i32,
    file_count: i32,
    box_key: String,
    key: String,
}

/// Get a specific Box
///
/// API docs:
/// <https://www.streak.com/api/#specificbox>
///
/// ```rust
/// extern crate streak;
///
/// use streak::Client;
///
/// fn main() {
///     streak::boxes::get(&c, box_key);
/// }
/// ```
pub fn get(client: &Client, box_key: &str) -> Result<Box, StreakError> {
    let res = client.get(&format!("boxes/{}.json", box_key), ())?;
    let box = serde_json::from_value(res.clone())?;
    Ok(box)
}
