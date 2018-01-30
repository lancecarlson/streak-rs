extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_url_params;

#[macro_use]
extern crate log;

mod error;
pub use error::StreakError;

mod client;
pub use client::{Client, Status};

pub mod api;
