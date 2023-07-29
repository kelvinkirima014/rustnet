use std::collections::HashMap;
use std::io::Result;

pub struct HttpResponse<'a> {
    pub head: HashMap<String, String>,
    pub body: String,
    pub status_code: &'a str
}