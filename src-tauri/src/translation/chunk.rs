#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkDecision {
    pub text: String,
    pub exceeds_limit: bool,
}

pub fn plan_atomic_unit(text: &str, max_chars: usize) -> ChunkDecision {
    ChunkDecision {
        text: text.to_owned(),
        exceeds_limit: text.chars().count() > max_chars,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_an_oversized_atomic_unit_whole() {
        let input = "あ".repeat(12);
        let result = plan_atomic_unit(&input, 10);
        assert_eq!(result.text, input);
        assert!(result.exceeds_limit);
    }
}
