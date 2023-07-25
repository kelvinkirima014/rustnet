
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