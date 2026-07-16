pub mod chunk;
pub mod language;
pub mod ollama;
pub mod prompt;
pub mod protect;
pub mod retry;
pub mod validate;

use crate::error::AppResult;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct TranslationRequest {
    pub model: String,
    pub prompt: String,
}

#[async_trait]
pub trait Translator: Send + Sync {
    async fn translate(&self, request: TranslationRequest) -> AppResult<String>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::TranslationConfig, translation::retry::translate_validated};
    use tokio_util::sync::CancellationToken;

    struct MarkerPreservingTranslator;

    #[async_trait]
    impl Translator for MarkerPreservingTranslator {
        async fn translate(&self, request: TranslationRequest) -> AppResult<String> {
            let text = request.prompt.split("TEXT TO TRANSLATE:\n").last().unwrap();
            Ok(text.replace("赤", "đỏ").replace("青", "xanh"))
        }
    }

    #[tokio::test]
    async fn validates_and_restores_a_translation() {
        let config = TranslationConfig {
            endpoint: "http://localhost:11434".into(),
            model: "fake".into(),
            source_language: "ja".into(),
            target_language: "vi".into(),
            chunk_chars: 1_800,
            timeout_secs: 10,
            max_attempts: 2,
        };
        let output = translate_validated(
            &MarkerPreservingTranslator,
            &config,
            "⟦S0⟧赤 123⟦S1⟧青",
            &CancellationToken::new(),
        )
        .await
        .unwrap();
        assert_eq!(output, "⟦S0⟧đỏ 123⟦S1⟧xanh");
    }
}
