use reqwest::header::HeaderMap;

pub struct Request {
    pub url: String,
    pub method: RequestType,
    pub headers: HeaderMap,
    pub body: String,
    pub timeout: u64,
    pub output: String,
    pub verbose: bool,
    pub download: bool,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            url: String::new(),
            method: RequestType::GET,
            headers: HeaderMap::new(),
            body: String::new(),
            timeout: 0,
            output: String::new(),
            verbose: false,
            download: false,
        }
    }
}

impl Request {
    pub fn new(url: String) -> Self {
        Self {
            url,
            ..Default::default()
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum RequestType {
    POST,
    GET,
    PUT,
    DELETE,
}
