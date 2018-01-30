// The Search endpoint allows you to search the contents on boxes. The search functionality returns a relevance sorted list of boxes.

#[derive(Default, Serialize)]
pub struct SearchParamsBuilder {
    query: Option<String>,
    name: Option<String>,
    page: Option<i32>,
    pipeline_key: Option<Vec<String>>,
    stage_key: Option<Vec<String>>,
}

pub fn query(query: String) -> SearchParamsBuilder {
    SearchParamsBuilder {
        query: Some(query),
        .. SearchParamsBuilder::default()
    }
}

pub fn name(name: String) -> SearchParamsBuilder {
    SearchParamsBuilder {
        name: Some(name),
        .. SearchParamsBuilder::default()
    }
}
