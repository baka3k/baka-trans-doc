use crate::error::{AppError, AppResult};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct ProtectedText {
    pub value: String,
    replacements: Vec<(String, String)>,
}

impl ProtectedText {
    pub fn restore(&self, translated: &str) -> AppResult<String> {
        let mut output = translated.to_owned();
        for (marker, original) in &self.replacements {
            let count = output.matches(marker).count();
            if count != 1 {
                return Err(AppError::InvalidTranslation(format!(
                    "placeholder {marker} occurred {count} times"
                )));
            }
            output = output.replace(marker, original);
        }
        Ok(output)
    }
}

pub fn protect(text: &str) -> AppResult<ProtectedText> {
    let token_re = Regex::new(r"https?://[^\s<>]+|`[^`\r\n]+`|\b[0-9][0-9.,:/%+-]*\b")
        .map_err(|error| AppError::Internal(error.to_string()))?;
    let mut prefix_index = 0;
    let prefix = loop {
        let candidate = format!("P{prefix_index}_");
        if !text.contains(&format!("⟦{candidate}")) {
            break candidate;
        }
        prefix_index += 1;
    };

    let mut replacements = Vec::new();
    let mut output = String::with_capacity(text.len());
    let mut cursor = 0;
    for (index, matched) in token_re.find_iter(text).enumerate() {
        output.push_str(&text[cursor..matched.start()]);
        let marker = format!("⟦{prefix}{index:04}⟧");
        output.push_str(&marker);
        replacements.push((marker, matched.as_str().to_owned()));
        cursor = matched.end();
    }
    output.push_str(&text[cursor..]);
    Ok(ProtectedText {
        value: output,
        replacements,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_urls_code_and_numbers() {
        let original = "詳細は https://example.com/a を参照し、`foo()` を 123 回実行。";
        let protected = protect(original).unwrap();
        assert!(!protected.value.contains("https://"));
        assert_eq!(protected.restore(&protected.value).unwrap(), original);
    }
}
