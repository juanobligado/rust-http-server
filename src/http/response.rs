use std::fmt::{Display, Formatter, Result as FmtResult};
use std::net::Shutdown::Write;
use super::StatusCode;

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}
impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self {
            status_code,
            body,
        }
    }
    pub fn status_line(&self) -> String {
        format!("HTTP/1.1 {} {}\r\n", self.status_code as u16, self.status_code.reason_phrase())
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let body = match &self.body {
            Some(b) => b,
            None => ""
        };
        write!(f,
               "HTTP/1.1 {} {}\r\n\r\n{}",
               self.status_code,
               self.status_code.reason_phrase(),
               body)
    }
}