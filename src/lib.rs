//! Streak API Bindings in Rust Documentation
//!
//! Streak is a CRM that integrates with GMail
//!
//! Streak API References:
//!
//! - [Streak API Docs](https://www.streak.com/api/)
//! - [Streak Developer Hub](https://streak.readme.io/)
//!
//! ## Usage
//!
//! Edit your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! streak = "*"
//! ```
//!
//! Create a client and then access any of the endpoints
//!
//! ```rust
//! extern crate streak;
//!
//! use streak::Client;
//! use std::env;
//!
//! fn main() {
//! #   streak::Client::example();
//!     let api_key = env::var("STREAK_API_KEY").expect("to have a STREAK_API_KEY set");
//!     let client = Client::new(&api_key);
//! }
//! ```

extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_url_params;

#[macro_use]
extern crate log;

extern crate dotenv;

pub mod error;
pub use error::StreakError;

mod client;
pub use client::{Client, Status};

pub mod api;
