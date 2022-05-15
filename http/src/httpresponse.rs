#[derive(Debug, PartialEq)]
pub enum StatusCode {
    Ok,
    Accepted,
    NotFound,
    Uninitialized,
}

impl From<&str> for StatusCode {
    fn from(s: &str) -> StatusCode {
        match s {
            "200" => StatusCode::Ok,
            "202" => StatusCode::Accepted,
            "404" => StatusCode::NotFound,
            _ => StatusCode::Uninitialized, 
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}

use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: StatusCode,
    pub version: Version,
    pub header_line: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpResponse {
    fn from(res :String) -> Self {
        let mut parsed_version = Version::V1_1;
        let mut parsed_code = StatusCode::Uninitialized;
        let mut parsed_body = "".to_string();
        let mut pared_headers = HashMap::new();
        for line in res.lines() {
            if line.contains("HTTP") {
                let (version, code) = process_res_line(line);
                parsed_version = version;
                parsed_code = code;
            }
            else if line.contains(":") {
                let (key, value) = process_header_line(line);
                pared_headers.insert(key, value);
            }
        }
        if parsed_code == StatusCode::Ok {
            parsed_body = "<h1>200 Ok</h1>".to_string();
        } else if parsed_code == StatusCode::Accepted {
            parsed_body = "<h1>202 Accepted</h1>".to_string();
        } else if parsed_code == StatusCode::NotFound {
            parsed_body = "<h1>404 Not Found</h1>".to_string();
        }
        HttpResponse {
            version: parsed_version,
            status_code: parsed_code,
            header_line: pared_headers,
            msg_body: parsed_body.to_string(),
        }
    }
}

fn process_res_line(s: &str) -> (Version, StatusCode) {
    // Parse the request line into individual chunks split by whitespaces.
    let mut words = s.split_whitespace();
    // Extract the HTTP version from first part of the request line
    let version = words.next().unwrap();
    // Extract the status code
    let code = words.next().unwrap();

    (
        version.into(),
        code.into(),
    )
}

fn process_header_line(s: &str) -> (String, String) {
    // Parse the header line into words split by separator (':')
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    // Extract the key part of the header
    if let Some(k) = header_items.next() {
        key = k.to_string();
    }
    // Extract the value part of the header
    if let Some(v) = header_items.next() {
        value = v.to_string()
    }
    (key, value)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_status_code() {
        let s: StatusCode = "404".into();
        assert_eq!(s, StatusCode::NotFound);
    }
    #[test]
    fn test_version_into() {
        let m: Version = "HTTP/1.1".into();
        assert_eq!(m, Version::V1_1);
    }
    #[test]
    fn test_res_line() {
        let s: String = String::from("HTTP/1.1 200 Ok\r\nContent-type: text/html");
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Content-type".into(), " text/html".into());
        let res: HttpResponse = s.into();
        let body = "<h1>200 Ok</h1>";
        assert_eq!(Version::V1_1, res.version);
        assert_eq!(StatusCode::Ok, res.status_code);
        assert_eq!(headers_expected, res.header_line);
        assert_eq!(body, res.msg_body);
    }
}