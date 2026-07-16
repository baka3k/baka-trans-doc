use crate::translation::language::{LanguageInfo, validate_pair};
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
    pub source_language: String,
    pub target_language: String,
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
        validate_pair(&self.source_language, &self.target_language)?;
        if !(500..=8_000).contains(&self.chunk_chars) {
            return Err("Chunk size must be between 500 and 8000 characters".into());
        }
        Ok(())
    }

    pub fn language_pair(&self) -> Result<(&'static LanguageInfo, &'static LanguageInfo), String> {
        validate_pair(&self.source_language, &self.target_language)
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
    use sha2::{Digest, Sha256};

    #[test]
    fn rejects_a_loopback_prefix_with_a_remote_host() {
        assert!(validate_loopback_endpoint("http://127.0.0.1:80@evil.example").is_err());
        assert!(validate_loopback_endpoint("http://localhost:11434").is_ok());
    }

    #[test]
    fn rejects_same_or_unknown_languages() {
        let mut config = TranslationConfig {
            endpoint: default_endpoint(),
            model: "fake".into(),
            source_language: "ja".into(),
            target_language: "ja".into(),
            chunk_chars: default_chunk_chars(),
            timeout_secs: default_timeout_secs(),
            max_attempts: default_attempts(),
        };
        assert!(config.validate().is_err());
        config.target_language = "xx".into();
        assert!(config.validate().is_err());
    }

    #[test]
    fn language_pair_changes_the_serialized_config_hash() {
        let base = TranslationConfig {
            endpoint: default_endpoint(),
            model: "fake".into(),
            source_language: "ja".into(),
            target_language: "vi".into(),
            chunk_chars: default_chunk_chars(),
            timeout_secs: default_timeout_secs(),
            max_attempts: default_attempts(),
        };
        let hash = |config: &TranslationConfig| Sha256::digest(serde_json::to_vec(config).unwrap());
        let mut changed = base.clone();
        changed.target_language = "en".into();
        assert_ne!(hash(&base)[..], hash(&changed)[..]);
        changed = base.clone();
        changed.source_language = "ko".into();
        assert_ne!(hash(&base)[..], hash(&changed)[..]);
    }
}
