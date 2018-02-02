//! The Search endpoint allows you to search the contents on boxes. The search functionality returns a relevance sorted list of boxes.

use serde_json;

use client::Client;
use error::StreakError;

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchParamsBuilder {
    query: Option<String>,
    name: Option<String>,
    page: Option<i32>,
    pipeline_key: Option<Vec<String>>,
    stage_key: Option<Vec<String>>,
}

impl SearchParamsBuilder {
    pub fn page(mut self, page: i32) -> SearchParamsBuilder {
        self.page = Some(page);
        self
    }

    pub fn pipeline_key(mut self, pipeline_key: Vec<String>) -> SearchParamsBuilder {
        self.pipeline_key = Some(pipeline_key);
        self
    }

    pub fn stage_key(mut self, stage_key: Vec<String>) -> SearchParamsBuilder {
        self.stage_key = Some(stage_key);
        self
    }

    pub fn send(self, c: &Client) -> Result<SearchResponse, StreakError> {
        let res = c.get("search", self)?;
        let s_res = serde_json::from_value(res.clone())?;
        Ok(s_res)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub results: SearchResults,
    pub page: i32,
    pub query: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResults {
    pub orgs: Option<Vec<Organization>>,
    pub boxes: Vec<Box>,
    pub contacts: Option<Vec<Contact>>,
}

#[derive(Debug, Deserialize)]
pub struct Organization {
    pub name: String,
    pub key: String,
    pub industry: String,
    pub domains: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Box {
    pub box_key: String,
    pub name: String,
    pub last_updated_timestamp: i64, // TODO: Convert to DateTime
    pub stage_key: String,
    pub pipeline_key: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub key: String,
    pub email_addresses: Option<Vec<String>>,
    pub title: Option<String>,
}

/// Searching for boxes, contacts, and organizations by query
///
/// Any search by query will return `Vec<Organization>`, `Vec<Box>` and `Vec<Contact>`
///
/// ```rust
/// extern crate streak;
///
/// use streak::error::StreakError;
/// use streak::api::search::{self, SearchResponse};
///
/// fn main() {
///     let res = run_query().expect("to get query results");
///     println!("{:#?}", res);
///     assert!(res.results.boxes.len() > 0);
/// }
///
/// fn run_query() -> Result<SearchResponse, StreakError> {
///     let c = streak::Client::example();
///     search::query("test").send(&c)
/// }
/// ```
pub fn query(query: &str) -> SearchParamsBuilder {
    SearchParamsBuilder {
        query: Some(query.into()),
        .. SearchParamsBuilder::default()
    }
}

/// Searching for a box by name
///
/// `Box` search is an exact match.
///
/// ```rust
/// extern crate streak;
///
/// use streak::error::StreakError;
/// use streak::api::search::{self, SearchResponse};
///
/// fn main() {
///     let res = run_name().expect("to run search by name");
///     println!("{:#?}", res);
///     assert!(res.results.boxes.len() > 0);
/// }
///
/// fn run_name() -> Result<SearchResponse, StreakError> {
///     let c = streak::Client::example();
///     search::name("AWS").send(&c)
/// }
/// ```
pub fn name(name: &str) -> SearchParamsBuilder {
    SearchParamsBuilder {
        name: Some(name.into()),
        .. SearchParamsBuilder::default()
    }
}
