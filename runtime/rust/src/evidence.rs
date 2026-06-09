use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EvidenceKind {
    Log,
    Trace,
    File,
    Approval,
    ApiResponse,
    Diff,
    Attestation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceRef {
    pub evidence_id: String,
    pub kind: EvidenceKind,
    pub uri: String,
    pub hash: Option<String>,
    pub captured_at: OffsetDateTime,
}

impl EvidenceRef {
    pub fn new(kind: EvidenceKind, uri: impl Into<String>) -> Self {
        Self {
            evidence_id: format!("evd_{}", Uuid::new_v4()),
            kind,
            uri: uri.into(),
            hash: None,
            captured_at: OffsetDateTime::now_utc(),
        }
    }

    pub fn with_hash(mut self, hash: impl Into<String>) -> Self {
        self.hash = Some(hash.into());
        self
    }
}
