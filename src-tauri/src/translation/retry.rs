use crate::{
    config::TranslationConfig,
    error::{AppError, AppResult},
    translation::{TranslationRequest, Translator, prompt, protect, validate},
};
use std::time::Duration;
use tokio_util::sync::CancellationToken;

pub async fn translate_validated<T: Translator + ?Sized>(
    translator: &T,
    config: &TranslationConfig,
    source: &str,
    cancellation: &CancellationToken,
) -> AppResult<String> {
    let normalized = source.replace("\r\n", "\n");
    let protected = protect::protect(&normalized)?;
    let attempts = config.max_attempts.max(1);
    let mut last_error = None;

    for attempt in 0..attempts {
        if cancellation.is_cancelled() {
            return Err(AppError::Cancelled);
        }
        let request = TranslationRequest {
            model: config.model.clone(),
            prompt: prompt::build_prompt(&protected.value, attempt > 0),
        };
        let response = tokio::select! {
            _ = cancellation.cancelled() => return Err(AppError::Cancelled),
            response = translator.translate(request) => response,
        };
        match response.and_then(|output| {
            validate::validate_response(&protected.value, &output)?;
            protected.restore(&output)
        }) {
            Ok(output) => return Ok(output),
            Err(error @ (AppError::ModelUnavailable(_) | AppError::Cancelled)) => {
                return Err(error);
            }
            Err(error) => last_error = Some(error),
        }
        if attempt + 1 < attempts {
            let delay_ms =
                250_u64.saturating_mul(1_u64 << attempt.min(5)) + u64::from(attempt) * 37;
            tokio::select! {
                _ = cancellation.cancelled() => return Err(AppError::Cancelled),
                _ = tokio::time::sleep(Duration::from_millis(delay_ms)) => {}
            }
        }
    }
    Err(last_error.unwrap_or_else(|| {
        AppError::InvalidTranslation("translation failed without a response".into())
    }))
}
