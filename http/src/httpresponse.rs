use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{Result, Write};

use crate::httprequest::Version;

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a>{
    pub head: Parts<'a>,
    pub body: String,
}

impl<'a> HttpResponse<'a> {
    pub fn new() -> Self {
        let body = String::new();
         HttpResponse{
            body,
            head: Parts::new(),   
        }
    }

    pub fn to_string(&self) -> String {

        let mut headers_string = String::new();
        for (name, value) in &self.head.headers {
            headers_string.push_str(&format!(
                "{}: {}\r\n",
                name,
                value
            ))
        }

        format!(
            "HTTP/{} {} {}\r\n{}\r\n{}",
            match self.head.version {
                Version::VersionOne => "1.1",
                Version::VersionTwo => "2.1",
                Version::Uninitialized => "",
            },
            self.head.status_code,
            self.head.status_text,
            headers_string,
            self.body
        )
    }

    pub fn send_response(&self, stream_writer: &mut impl Write) -> Result<()> {
        let res = self.clone();
        let response_string = res.to_string();
        let _ = write!(stream_writer, "{}", response_string);
        Ok(())
    }
    
}

#[derive(Debug, PartialEq, Clone)]
pub struct Parts<'a> {
    pub status_code : &'a str,
    pub status_text: &'a str,
    pub headers: HashMap<&'a str, &'a str>,
    pub version: Version,
}

impl Default for Parts<'_> {
    fn default() -> Self {
     let mut headers = HashMap::new();
        headers.insert("Host", "localhost");
        Parts { 
            status_code: "200", 
            status_text: "OK", 
            headers, 
            version: Version::Uninitialized,
        }
    }
}

impl<'a> Parts<'a> {
    fn new() -> Self {
        let mut response = Parts::default();

        response.status_text = match  response.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "404" => "NOt Found",
            "500" => "Internal Server Error",
            _=> "Not Found",
        };
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_response_creation(){

        let response = HttpResponse::new();

        assert_eq!(response.body, "");
        assert_eq!(response.head.status_code, "200");
        assert_eq!(response.head.status_text, "OK");
        assert_eq!(response.head.version, Version::Uninitialized);

        assert!(response.head.headers.contains_key("Satan"));

    }

}