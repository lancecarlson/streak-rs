//! Box actions

use serde_json;

use error::StreakError;
use client::Client;
use super::Box;

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
///     let client = streak::Client::example();
///     let pipelines = streak::api::pipelines::list(&client).expect("list pipelines");
///     let pipeline_key = &pipelines[0].pipeline_key;
///     let boxes = streak::api::boxes::list(&client, pipeline_key).expect("list boxes by pipeline");
///     let box_key = &boxes[0].box_key;
///     let get_box = streak::api::boxes::get(&client, box_key).expect("get a box");
///     println!("{:?}", get_box);
///     assert!(get_box.name != "");
/// }
/// ```
pub fn get(client: &Client, box_key: &str) -> Result<Box, StreakError> {
    let res = client.get(&format!("boxes/{}", box_key), ())?;
    let result = serde_json::from_value(res.clone())?;
    Ok(result)
}
