use std::cell::RefCell;

use client::web_connection::{WebConnection, WebError, WebResponse};
use reqwest::StatusCode;

pub enum Method {
    GET, POST, DELETE
}

pub struct MockWebConnection {
    pub method: Method,
    pub status: StatusCode,
    pub return_body_text: String,
    pub is_timeout: bool,
    pub was_get_called: RefCell<bool>,
    pub was_post_called: RefCell<bool>,
    pub was_delete_called: RefCell<bool>
}

impl MockWebConnection {
    pub fn new(method: Method, status: StatusCode, is_timeout: bool, return_body_text: String) -> Self {
        MockWebConnection {
            method: method,
            status: status,
            return_body_text: return_body_text,
            is_timeout: is_timeout,
            was_get_called: RefCell::new(false),
            was_post_called: RefCell::new(false),
            was_delete_called: RefCell::new(false)
        }
    }
}

impl WebConnection for MockWebConnection {
    fn get(&self, path: String) -> Result<WebResponse, WebError> {
        *self.was_get_called.borrow_mut() = true;
        match self.method {
            Method::GET => Ok(WebResponse {
                status: self.status,
                body: Ok(self.return_body_text.to_string())
            }),
            _ => Err(WebError {
                is_timeout: false,
                text: "GET failed".to_string()
            })
        }
    }

    fn post(&self, path: String, body: String) -> Result<WebResponse, WebError> {
        *self.was_post_called.borrow_mut() = true;
        if self.is_timeout {
            Err(WebError {
                is_timeout: true,
                text: "Timeout".to_string()
            })
        }
        else {
            match self.method {
                Method::POST => Ok(WebResponse {
                    status: self.status,
                    body: Ok(self.return_body_text.to_string())
                }),
                _ => Err(WebError {
                    is_timeout: false,
                    text: "POST failed".to_string()
                })
            }
        }
    }

    fn delete(&self, path: String) -> Result<WebResponse, WebError> {
        *self.was_delete_called.borrow_mut() = true;
        match self.method {
            Method::DELETE => Ok(WebResponse {
                status: self.status,
                body: Ok(self.return_body_text.to_string())
            }),
            _ => Err(WebError {
                is_timeout: false,
                text: "DELETE failed".to_string()
            })
        }
    }
}