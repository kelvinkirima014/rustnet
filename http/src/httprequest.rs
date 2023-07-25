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

        let mut parsed_method = Method::Uninitialized;

        let mut parsed_version = Version::VersionOne;

        let mut parsed_resource = Resource::Path("".to_string());

        let mut parsed_headers = HashMap::new();

        let mut parsed_body = "";

        //Read each line in the incoming HTTP request
        //and Evaluate each line
        for line in req.lines() {

            //if line is an HTTP request line
            if line.contains("HTTP") {

                //extract the method,HTTP version, and path(resource/url)
                let (method, resource, version) = process_request_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = resource;

                //else if line is a header line(contains ':' separator)
            } else if line.contains(":") {

                //extract Key and Value
                let (key, value) = process_header_line(line);

                //and add them to the list of request headers
                parsed_headers.insert(key, value);

                //else if empty, no action
            } else if line.len() == 0 {
                println!("empty line");

                //if none of these, treat it as a message body
                //by scanning and storing the string
            } else {
                parsed_body = line;
            }
        }

        // above, we try to detect the various types
        //of lines in the incoming HTTP Request, and then construct an
        //HTTPRequest struct with the parsed values
        
        HttpRequest {
             method: parsed_method, 
             version: parsed_version, 
             resource: parsed_resource, 
             headers: parsed_headers, 
             body: parsed_body.to_string(), 
        }


    }
}

fn process_request_line(_s: &str) -> (Method, Resource, Version) {
    todo!()
}

fn process_header_line(_s: &str) -> (String, String) {
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