use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{Result, Write};

use crate::httprequest::Version;

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a, T>{
    pub head: Parts<'a>,
    pub body: T,
}

impl<'a, T: Default> Default for HttpResponse<'a, T>
    where T: Debug, 
{
    fn default() -> Self {
        HttpResponse::new(T::default())
    }
}

impl<'a, T: Default + Clone + std::fmt::Debug> From<HttpResponse<'a, T>> for String {
    fn from(resp: HttpResponse<'a, T>) -> Self {
        let resp = resp.clone();
        format!(
            "{:?} {:?}",
            resp.head,
            resp.body
        )
    }
}


impl<'a, T: std::fmt::Debug> HttpResponse<'a, T> {
    pub fn new(body: T) -> Self {
         HttpResponse{
            body,
            head: Parts::new(),   
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{:?} {:?}",
            self.head,
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