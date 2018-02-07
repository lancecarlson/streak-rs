//! Pipeline actions

use serde_json;

use error::StreakError;
use client::Client;
use super::Pipeline;

/// List all Pipelines
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
///     println!("{:?}", pipelines);
///     assert!(pipelines.len() > 0);
///     assert!(pipelines[0].name != "");
/// }
/// ```
pub fn list(client: &Client) -> Result<Vec<Pipeline>, StreakError> {
    let res = client.get("pipelines", ())?;
    let pipeline = serde_json::from_value(res.clone())?;
    Ok(pipeline)
}
