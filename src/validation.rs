use url::Url;

use crate::error::ApiError;

pub fn validate_target(input: &str) -> Result<(), ApiError> {
    if input.len() > 2048 {
        return Err(ApiError::InvalidUrl);
    }

    let parsed = Url::parse(input).map_err(|_| ApiError::InvalidUrl)?;
    if !matches!(parsed.scheme(), "http" | "https") {
        return Err(ApiError::InvalidUrl);
    }
    if parsed.host_str().is_none() {
        return Err(ApiError::InvalidUrl);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_http_and_https_with_a_host() {
        assert!(validate_target("https://example.com/path?q=1#frag").is_ok());
        assert!(validate_target("http://127.0.0.1:8080/").is_ok());
    }

    #[test]
    fn rejects_unsupported_schemes_and_missing_hosts() {
        assert!(validate_target("ftp://example.com").is_err());
        assert!(validate_target("example.com").is_err());
        assert!(validate_target("https://").is_err());
        assert!(validate_target("").is_err());
        assert!(validate_target("data:text/plain,hello").is_err());
    }

    #[test]
    fn enforces_the_2048_character_boundary() {
        let base = "https://example.com/";
        let accepted = format!("{}{}", base, "a".repeat(2048 - base.len()));
        assert_eq!(accepted.len(), 2048);
        assert!(validate_target(&accepted).is_ok());

        let rejected = format!("{}{}", base, "a".repeat(2049 - base.len()));
        assert_eq!(rejected.len(), 2049);
        assert!(validate_target(&rejected).is_err());
    }
}
