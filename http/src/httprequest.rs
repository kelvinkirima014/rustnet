use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,//e.g <Key: Host,  Value: localhost:3000> 
    pub body: String,
}


impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        todo!()        
    }
}

fn process_request_line(s: &str) -> (Method, Resource, Version) {
    todo!()
}

fn process_header_line(s: &str) -> (String, String) {
    todo!()
}


//`Method` specifies the allowed values(variants)
//for HTTP Methods
#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}


//`FROM` trait converts the incoming string slice
//into HttpRequest data structure
impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum  Version {
    VersionOne,
    VersionTwo,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Version::VersionOne,
            _ => Version::Uninitialized,
        }
    }    
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::VersionOne);
    }

}