use crate::error::{AppError, AppResult};
use regex::Regex;

pub fn validate_response(source: &str, output: &str) -> AppResult<()> {
    if output.trim().is_empty() {
        return Err(AppError::InvalidTranslation(
            "the model returned an empty response".into(),
        ));
    }
    let marker_re = Regex::new(r"⟦(?:S\d+|P\d+_\d{4})⟧")
        .map_err(|error| AppError::Internal(error.to_string()))?;
    let source_markers: Vec<_> = marker_re
        .find_iter(source)
        .map(|item| item.as_str())
        .collect();
    let output_markers: Vec<_> = marker_re
        .find_iter(output)
        .map(|item| item.as_str())
        .collect();
    if source_markers != output_markers {
        return Err(AppError::InvalidTranslation(
            "protected markers were removed, changed, or reordered".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_reordered_formatting_markers() {
        let error = validate_response("⟦S0⟧赤⟦S1⟧青", "⟦S1⟧đỏ⟦S0⟧xanh").unwrap_err();
        assert!(matches!(error, AppError::InvalidTranslation(_)));
    }
}
