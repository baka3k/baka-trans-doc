use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Queued,
    Running,
    Cancelling,
    Completed,
    CompletedWithWarnings,
    Failed,
    Cancelled,
}

impl JobStatus {
    pub fn transition(self, next: Self) -> AppResult<Self> {
        let valid = matches!(
            (self, next),
            (Self::Queued, Self::Running)
                | (Self::Queued, Self::Cancelled)
                | (Self::Running, Self::Cancelling)
                | (Self::Running, Self::Completed)
                | (Self::Running, Self::CompletedWithWarnings)
                | (Self::Running, Self::Failed)
                | (Self::Cancelling, Self::Cancelled)
                | (Self::Cancelling, Self::Failed)
                | (Self::Cancelled, Self::Running)
                | (Self::Failed, Self::Running)
        );
        if valid {
            Ok(next)
        } else {
            Err(AppError::Internal(format!(
                "invalid job transition {self:?} -> {next:?}"
            )))
        }
    }

    pub fn is_recoverable(self) -> bool {
        matches!(
            self,
            Self::Queued | Self::Running | Self::Cancelling | Self::Cancelled | Self::Failed
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_terminal_to_running_transition() {
        assert!(JobStatus::Completed.transition(JobStatus::Running).is_err());
    }
}
