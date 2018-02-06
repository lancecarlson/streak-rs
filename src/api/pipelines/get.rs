//! Pipeline actions

use serde_json;

use error::StreakError;
use client::Client;
use super::Pipeline;

/// Get a specific Pipeline
///
/// API docs:
/// <https://www.streak.com/api/#specificpipeline>
///
/// ```rust
/// extern crate streak;
///
/// use streak::Client;
///
/// fn main() {
///     let client = streak::Client::example();
///     let pipelines = streak::api::pipelines::list(&client).expect("list all pipelines");
///     let pipeline = streak::api::pipelines::get(&client, &pipelines[0].pipeline_key).expect("get a pipeline");
///     println!("pipeline {:?}", pipeline);
///     assert!(pipeline.fields.len() > 0);
/// }
/// ```
pub fn get(client: &Client, pipeline_key: &str) -> Result<Pipeline, StreakError> {
    let res = client.get(&format!("pipelines/{}", pipeline_key), ())?;
    let pipeline = serde_json::from_value(res.clone())?;
    Ok(pipeline)
}
