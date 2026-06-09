use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PolicyEffect {
    Allow,
    Deny,
    RequiresApproval,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyDecision {
    pub effect: PolicyEffect,
    pub reason: Option<String>,
    pub engine: Option<String>,
    pub checked_at: OffsetDateTime,
}

impl PolicyDecision {
    pub fn allow(reason: impl Into<String>) -> Self {
        Self {
            effect: PolicyEffect::Allow,
            reason: Some(reason.into()),
            engine: Some("fabric-agent-runtime".to_string()),
            checked_at: OffsetDateTime::now_utc(),
        }
    }

    pub fn deny(reason: impl Into<String>) -> Self {
        Self {
            effect: PolicyEffect::Deny,
            reason: Some(reason.into()),
            engine: Some("fabric-agent-runtime".to_string()),
            checked_at: OffsetDateTime::now_utc(),
        }
    }

    pub fn requires_approval(reason: impl Into<String>) -> Self {
        Self {
            effect: PolicyEffect::RequiresApproval,
            reason: Some(reason.into()),
            engine: Some("fabric-agent-runtime".to_string()),
            checked_at: OffsetDateTime::now_utc(),
        }
    }

    pub fn is_allowed(&self) -> bool {
        self.effect == PolicyEffect::Allow
    }

    pub fn needs_approval(&self) -> bool {
        self.effect == PolicyEffect::RequiresApproval
    }
}
