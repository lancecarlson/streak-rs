use std::thread;
use std::io::Read;
use std::time::Duration;

use reqwest::{self, StatusCode, Method, Url};
use reqwest::header::{Headers, Authorization, Basic, ContentType, Location};
use serde;
use serde_json::{self, Value};
use serde_url_params;

use error::StreakError;

/// The Streak API Rust client.
///
/// This will allow the rest of this library to interact with the Streak API!
///
#[derive(Debug)]
pub struct Client {
    /// Configure the client to retry the request `retry_count` number of times
    /// when the service is unavailable.
    pub retry_count: u8,

    /// Duration of time to wait between retry attempts.
    pub retry_wait: u16,

    api_url: String,
    api_key: String,
    reqwest: reqwest::Client,
}

/// Status message returned by every API request.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub code: Option<i32>,
    pub error: String,
}

impl Client {
    /// Create a new client to the Streak service.
    pub fn new(api_key: &str) -> Client {
        Client {
            retry_count: 3,
            retry_wait: 250,
            api_url: "https://www.streak.com/api/v1".into(),
            api_key: api_key.into(),
            reqwest: reqwest::Client::new(),
        }
    }

    /// Send a `get` request to the Streak service. This is intended to be used
    /// by the library and not the user.
    pub fn get<T>(&self, path: &str, url_params: T) -> Result<Value, StreakError>
        where T: serde::Serialize
    {
        self.request(Method::Get, self.url(path, url_params)?, None)
    }

    fn url<T>(&self, path: &str, params: T) -> Result<Url, StreakError>
        where T: serde::Serialize
    {
        let mut base = format!("{api_url}/{path}",
                           api_url = self.api_url,
                           path = path);

        let encoded = serde_url_params::to_string(&params)?;
        base = format!("{}?{}", base, encoded);
        Ok(Url::parse(&base)?)
    }

    fn request(&self, method: Method, url: Url, request_body: Option<String>) -> Result<Value, StreakError> {
        let mut count = self.retry_count;
        loop {
            let url = url.clone();

            debug!("Attempting request - Method: {}. Url: {}", method, url);

            let mut headers = Headers::new();
            let credentials = Basic {
                username: self.api_key.clone(),
                password: Some("X".into()),
            };
            headers.set(Authorization(credentials));
            headers.set(ContentType::json());
            let mut res = match request_body.clone() {
                Some(b) => {
                    debug!("Request body - {}", b);
                    self.reqwest.request(method.clone(), url).headers(headers).body(b).send()?
                },
                None => self.reqwest.request(method.clone(), url).headers(headers).send()?,
            };

            let mut body = String::new();
            res.read_to_string(&mut body)?;

            debug!("Response body: {}", body);

            match serde_json::from_str::<Value>(&body) {
                Ok(value) => {
                    let status = serde_json::from_value(value.clone());

                    match res.status() {
                        StatusCode::Ok => return Ok(value),
                        StatusCode::Created => return Ok(value),
                        StatusCode::BadRequest => return Err(StreakError::BadRequest(status?)),
                        StatusCode::Unauthorized => return Err(StreakError::UnauthorizedKey(status?)),
                        StatusCode::Forbidden => return Err(StreakError::Forbidden(status?)),
                        StatusCode::NotFound => return Err(StreakError::UserNotFound(status?)),
                        StatusCode::InternalServerError => return Err(StreakError::InternalServerError(status?)),
                        s => panic!("Status code not covered in Streak REST specification: {}", s),
                    };
                },
                Err(_) => {
                    debug!("response headers: {}",res.headers());
                    if res.headers().has::<Location>() {
                        return Ok(serde_json::Value::String("Created".into()));
                    } else { 
                            match res.status() {
                            StatusCode::ServiceUnavailable => {
                                count -= 1;
                                if count == 0 {
                                    return Err(StreakError::ServiceUnavailable);
                                }
                                    else {
                                        thread::sleep(Duration::from_millis(self.retry_wait.into()));
                                        continue;
                                    }
                            },
                            _ => return Err(StreakError::InvalidServerResponse),
                        }
                    }
                },
            };
        }
}
}
