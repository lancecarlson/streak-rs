pub struct Pipeline {
    creator_key: String,
    name: String,
    description: String,
    org_wide: bool,
    fields: Vec<Field>,
}

pub struct Field {
    name: String,
    key: String,
    #[serde(rename = "type")]
    field_type: String,
}

pub fn list(client: &Client) -> Result<Pipeline, StreakError> {
    let res = client.get(&format!("boxes/{}.json", box_key), ())?;
    let box = serde_json::from_value(res.clone())?;
    Ok(box)
}
