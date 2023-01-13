use std::collections::HashMap;

use crate::models::http::{Request, RequestType};
use anyhow::{Context, Result};

use reqwest::{
    blocking::{Client, Response},
    header::{HeaderName, HeaderValue},
};

pub fn parse_request_options(options: HashMap<&str, &str>) -> Result<Request> {
    let url = options.get("url").expect("Url is required").to_string();
    let mut request = Request::new(url);
    for (key, value) in options {
        match key {
            "method" => {
                request.method = match value {
                    "POST" => RequestType::POST,
                    "GET" => RequestType::GET,
                    "PUT" => RequestType::PUT,
                    "DELETE" => RequestType::DELETE,
                    _ => RequestType::GET,
                }
            }
            "headers" => {
                let headers = value.split(",").collect::<Vec<&str>>();
                for header in headers {
                    let header = header.split(":").collect::<Vec<&str>>();
                    println!("{:?}", header[0]);
                    request.headers.insert(
                        HeaderName::from_bytes(header[0].trim_start().as_bytes())
                            .context("Invalid Header name")?,
                        HeaderValue::from_bytes(header[1].as_bytes())
                            .context("Invalid Header value")?,
                    );
                }
            }
            "body" => request.body = value.to_string(),
            "timeout" => request.timeout = value.parse::<u64>().context("Invalid timeout")?,
            "output" => request.output = value.to_string(),
            "verbose" => request.verbose = true,
            "download" => request.download = true,
            "url" => (),
            _ => panic!("Error: Invalid option {}", key),
        }
    }
    Ok(request)
}

pub fn build_request(options: &Request) -> Result<reqwest::blocking::Request> {
    let client = Client::builder()
        .connection_verbose(options.verbose)
        .build()?;
    let mut builder = match options.method {
        RequestType::DELETE => client.delete(&options.url),
        RequestType::GET => client.get(&options.url),
        RequestType::POST => client.post(&options.url),
        RequestType::PUT => client.put(&options.url),
    };

    if options.timeout > 0 {
        builder = builder.timeout(std::time::Duration::from_secs(options.timeout));
    }

    if options.headers.len() > 0 {
        let headers = options.headers.clone();
        builder = builder.headers(headers);
    }

    if options.body.len() > 0 {
        builder = builder.body(options.body.clone());
    }

    let request = builder.build()?;

    Ok(request)
}

pub fn parse_response(response: Response) -> Result<String> {
    let response_type = response
        .headers()
        .get("content-type")
        .context("Error parsing response")?
        .to_str()?;
    if response_type.contains("json") {
        let body = response.json::<serde_json::Value>()?;
        return Ok(body.to_string());
    }
    let body = response.text()?;
    Ok(body)
}
