use serde_json;

use error::StreakError;
use client::Client;
use super::Box;

/// List all boxes in a Pipeline
///
/// API docs:
/// <https://www.streak.com/api/#pipeline>
///
/// ```rust
/// extern crate streak;
///
/// use streak::Client;
///
/// fn main() {
///     let client = streak::Client::example();
///     let pipelines = streak::api::pipelines::list(&client).expect("list all pipelines");
///     let pipeline_key = &pipelines[0].pipeline_key;
///     let boxes = streak::api::boxes::list(&client, pipeline_key).expect("list boxes by pipeline");
///     println!("{:?}", boxes);
///     assert!(boxes.len() > 0);
///     assert!(boxes[0].name != "");
/// }
/// ```
pub fn list(client: &Client, pipeline_key: &str) -> Result<Vec<Box>, StreakError> {
    let res = client.get(&format!("pipelines/{}/boxes", pipeline_key), ())?;
    let result = serde_json::from_value(res.clone())?;
    Ok(result)
}
