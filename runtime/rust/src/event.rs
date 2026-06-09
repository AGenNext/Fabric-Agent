use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::box_model::IdentityRef;
use crate::evidence::EvidenceRef;
use crate::policy::PolicyDecision;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    IntentReceived,
    ContextResolved,
    PolicyChecked,
    ApprovalRequested,
    ApprovalGranted,
    ApprovalDenied,
    PlanCreated,
    ActionStarted,
    ActionCompleted,
    ActionFailed,
    EvidenceAttached,
    ReconciliationStarted,
    ReconciliationCompleted,
    StateChanged,
    ReportGenerated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FabricEvent {
    pub event_id: String,
    pub box_id: String,
    pub event_type: EventType,
    pub actor: IdentityRef,
    pub target: Option<IdentityRef>,
    pub sequence: u64,
    pub payload: serde_json::Value,
    pub evidence: Vec<EvidenceRef>,
    pub policy: Option<PolicyDecision>,
    pub correlation_id: Option<String>,
    pub causation_id: Option<String>,
    pub observed_at: OffsetDateTime,
}

impl FabricEvent {
    pub fn new(
        box_id: impl Into<String>,
        event_type: EventType,
        actor: IdentityRef,
        sequence: u64,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            event_id: format!("evt_{}", Uuid::new_v4()),
            box_id: box_id.into(),
            event_type,
            actor,
            target: None,
            sequence,
            payload,
            evidence: Vec::new(),
            policy: None,
            correlation_id: None,
            causation_id: None,
            observed_at: OffsetDateTime::now_utc(),
        }
    }

    pub fn with_evidence(mut self, evidence: EvidenceRef) -> Self {
        self.evidence.push(evidence);
        self
    }

    pub fn with_policy(mut self, policy: PolicyDecision) -> Self {
        self.policy = Some(policy);
        self
    }
}
