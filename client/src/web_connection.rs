use reqwest::blocking::Response;
use reqwest::{Error, StatusCode};
use rocket::http::Status;

pub struct WebResponse {
    pub status: StatusCode,
    pub body: Result<String, Error>
}

pub struct WebError {
    pub is_timeout: bool,
    pub text: String
}

pub trait WebConnection {
    fn get(&self, path: String) -> Result<WebResponse, WebError>;
    fn post(&self, path: String, body: String) -> Result<WebResponse, WebError>;
    fn delete(&self, path: String) -> Result<WebResponse, WebError>;
}

pub struct DefaultWebConnection { }

impl WebConnection for DefaultWebConnection {
    fn get(&self, path: String) -> Result<WebResponse, WebError> {
        let response = reqwest::blocking::get(path)
            .map_err(|e| WebError {
                is_timeout: e.is_timeout(),
                text: e.to_string()
            })?;
        Ok(WebResponse {
            status: response.status(),
            body: response.text()
        })
    }

    fn post(&self, path: String, body: String) -> Result<WebResponse, WebError> {
        let client = reqwest::blocking::Client::new();
        let response = client.post(path)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .map_err(|e| WebError {
                is_timeout: e.is_timeout(),
                text: e.to_string()
            })?;
        Ok(WebResponse {
            status: response.status(),
            body: response.text()
        })
    }

    fn delete(&self, path: String) -> Result<WebResponse, WebError> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .delete(path)
            .send()
            .map_err(|e| WebError {
                is_timeout: e.is_timeout(),
                text: e.to_string()
            })?;
        Ok(WebResponse {
            status: response.status(),
            body: response.text()
        })
    }
}