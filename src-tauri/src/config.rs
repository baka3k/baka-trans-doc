use serde::{Deserialize, Serialize};

fn default_endpoint() -> String {
    "http://localhost:11434".into()
}

fn default_chunk_chars() -> usize {
    1_800
}

fn default_timeout_secs() -> u64 {
    120
}

fn default_attempts() -> u8 {
    3
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TranslationConfig {
    #[serde(default = "default_endpoint")]
    pub endpoint: String,
    pub model: String,
    #[serde(default = "default_chunk_chars")]
    pub chunk_chars: usize,
    #[serde(default = "default_timeout_secs")]
    pub timeout_secs: u64,
    #[serde(default = "default_attempts")]
    pub max_attempts: u8,
}

impl TranslationConfig {
    pub fn validate(&self) -> Result<(), String> {
        validate_loopback_endpoint(&self.endpoint)?;
        if self.model.trim().is_empty() {
            return Err("A model must be selected".into());
        }
        if !(500..=8_000).contains(&self.chunk_chars) {
            return Err("Chunk size must be between 500 and 8000 characters".into());
        }
        Ok(())
    }
}

pub fn validate_loopback_endpoint(endpoint: &str) -> Result<(), String> {
    let url = reqwest::Url::parse(endpoint).map_err(|_| "Ollama endpoint is not a valid URL")?;
    if url.scheme() != "http" || !url.username().is_empty() || url.password().is_some() {
        return Err("Ollama endpoint must be an unauthenticated HTTP loopback URL".into());
    }
    let is_loopback = match url.host_str() {
        Some("localhost") => true,
        Some(host) => host
            .parse::<std::net::IpAddr>()
            .is_ok_and(|ip| ip.is_loopback()),
        None => false,
    };
    if !is_loopback
        || !matches!(url.path(), "" | "/")
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err("MVP only permits a root localhost Ollama endpoint".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_a_loopback_prefix_with_a_remote_host() {
        assert!(validate_loopback_endpoint("http://127.0.0.1:80@evil.example").is_err());
        assert!(validate_loopback_endpoint("http://localhost:11434").is_ok());
    }
}
