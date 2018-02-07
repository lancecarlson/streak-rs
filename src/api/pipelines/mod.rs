mod list;
pub use self::list::list;

mod get;
pub use self::get::get;

use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pipeline {
    pub key: String,
    pub pipeline_key: String,
    pub creator_key: String,
    pub name: String,
    pub description: Option<String>,
    pub org_wide: bool,
    pub fields: Vec<Field>,
    pub stages: HashMap<String, Stage>,
    pub stage_order: Vec<String>,
    pub acl_entries: Vec<AclEntry>,
    pub owner: Option<AclEntry>,
    // Undocumented fields
    pub team_key: String,
    pub team_wide: bool,
    pub creation_timestamp: i64,
    pub last_updated_timestamp: i64,
    pub last_saved_timestamp: i64,
    pub box_count_hint: i32,
    pub box_count: i32,
    pub sharing_restricted_to_org: bool,
    pub sharing_restricted_to_team: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub name: String,
    pub key: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub last_updated_timestamp: Option<i64>,
    pub dropdown_settings: Option<FieldItems>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldItems {
    pub items: Vec<FieldItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldItem {
    pub key: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Stage {
    pub name: String,
    pub key: String,
    pub color: Color,
    pub box_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    pub foreground_color: String,
    pub background_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AclEntry {
    pub full_name: String,
    pub email: String,
    pub is_owner: Option<bool>, // deprecated?
    pub image: String,
    pub display_name: String,
    pub user_key: String,
    pub permission_set_name: String,
}
