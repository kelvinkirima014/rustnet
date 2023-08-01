use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{Result, Write};

use crate::httprequest::Version;

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a>{
    pub head: Parts<'a>,
    pub body: String,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        let body = String::new();
        HttpResponse::new(body)
    }
}


impl<'a> HttpResponse<'a> {
    pub fn new(body: String) -> Self {
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
            self.body.to_string()
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

impl<'a> Parts<'a> {
    fn new() -> Self {
        let mut headers = HashMap::new();
        headers.insert("Host".into(), "localhost".into());
        Parts { 
            status_code: "200", 
            status_text: "OK", 
            headers, 
            version: Version::Uninitialized,
        }
    }
}


impl From<Parts<'_>> for String{
    fn from(value: Parts) -> Self {
        let parts = value.clone();
        format!(
            "{} {} {:?}\r\n{:?}",
            &parts.status_code,
            &parts.status_text,
            &parts.headers,
            &parts.version

        )
    }
}