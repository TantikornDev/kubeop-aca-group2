#[derive(Debug, PartialEq)]
pub enum StatusCode {
    Continue,
    SwitchingProtocols,
    Ok,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    TemporaryRedirect,
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    Uninitialized,
}

impl From<&str> for StatusCode {
    fn from(s: &str) -> StatusCode {
        match s {
            "100" => StatusCode::Continue,
            "101" => StatusCode::SwitchingProtocols,
            "200" => StatusCode::Ok,
            "201" => StatusCode::Created,
            "202" => StatusCode::Accepted,
            "203" => StatusCode::NonAuthoritativeInformation,
            "204" => StatusCode::NoContent,
            "205" => StatusCode::ResetContent,
            "206" => StatusCode::PartialContent,
            "300" => StatusCode::MultipleChoices,
            "301" => StatusCode::Found,
            "302" => StatusCode::SeeOther,
            "303" => StatusCode::SeeOther,
            "304" => StatusCode::NotModified,
            "305" => StatusCode::UseProxy,
            "307" => StatusCode::TemporaryRedirect,
            "400" => StatusCode::BadRequest,
            "401" => StatusCode::Unauthorized,
            "402" => StatusCode::PaymentRequired,
            "403" => StatusCode::Forbidden,
            "404" => StatusCode::NotFound,
            "405" => StatusCode::MethodNotAllowed,
            "406" => StatusCode::NotAcceptable,
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_status_code() {
        let s: StatusCode = "404".into();
        assert_eq!(s, StatusCode::NotFound);
    }
}
