use serde_json;

use error::StreakError;
use client::Client;
use super::Contact;

/// Get a Contact
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
///     let box_with_contacts = boxes.iter().find(|b| b.contacts.is_some()).expect("find box with contacts");
///     let contact_key = &box_with_contacts.contacts.clone().unwrap()[0].key;
///     let contact = streak::api::contacts::get(&client, contact_key).expect("get a contact");
///     println!("{:?}", contact);
///     assert!(contact.key != "");
/// }
/// ```
pub fn get(client: &Client, key: &str) -> Result<Contact, StreakError> {
    let res = client.get_v2(&format!("contacts/{}", key), ())?;
    let result = serde_json::from_value(res.clone())?;
    Ok(result)
}
