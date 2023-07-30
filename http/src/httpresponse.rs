use std::collections::HashMap;
use std::io::Result;

use crate::httprequest::Version;

pub struct HttpResponse<'a, T> {
    pub head: Parts<'a>,
    pub body: T,
}

impl<'a, T: Default> Default for HttpResponse<'a, T> {
    fn default() -> Self {
        HttpResponse::new(T::default())
    }
}

impl<'a, T> HttpResponse<'a, T> {
    fn new(body: T) -> Self {
         HttpResponse{
            body,
            head: Parts::new(),   
        }
        
    }
}

pub struct Parts<'a> {
    pub status_code : &'a str,
    pub status_text: &'a str,
    pub headers: HashMap<String, String>,
    pub version: Version,
}

impl<'a> Parts<'a> {
    fn new() -> Self {
        let mut headers = HashMap::new();
        headers.insert("Host".into(), "localhost".into());
        Parts { 
            status_code: "OK", 
            status_text: "", 
            headers, 
            version: Version::Uninitialized,
        }
    }
}