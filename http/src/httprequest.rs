use std::collections::HashMap;


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


//process the request line of incoming HTTP request
//by splitting the request into three parts(method, resource, version)
//transform the splitted part into more structured data types
//Mrhod, Resource, and Version
fn process_request_line(s: &str) -> (Method, Resource, Version) {
    
    //parse the request line into individual chunks
    //request line typically looks like:
    //"GET /index.html HTTP/1.1" 
    let mut request = s.split_whitespace();

    //Extract the HTTP method(e.g "GET") from first part of the request line
    let method = request.next().unwrap();

    //Extract the resource(URI/URL)(e.g /index.html) from the second part of the req
    let resource = request.next().unwrap();

    //Extract the HTTP version(e.g HTTP/1.1) from third part pf the request line
    let version = request.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

fn process_header_line(s: &str) -> (String, String) {
    
    let mut header = s.split(":");

    let key = header.next().unwrap();

    let value = header.next().unwrap();

    (
        key.to_string(), 
        value.to_string(),
    )

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

#[derive(Debug, PartialEq)]

pub enum Resource {
    Path(String),
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

    #[test]
    fn test_read_http() {
        let s: String = String::from("GET /hello HTTP/1.1\r\nH");

        let mut headers_expected: HashMap<String, String> = HashMap::new();
        headers_expected.insert("Host".into(), "localhost".into());
        headers_expected.insert("Accept".into(), "*/*".into());
        headers_expected.insert("User-Agent".into(), " curl/7.64.1".into());

        let req: HttpRequest = s.into();

        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::VersionOne, req.version);
        assert_eq!(Resource::Path("/hello".to_string()), req.resource);
    }

}