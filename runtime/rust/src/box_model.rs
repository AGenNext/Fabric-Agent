use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::evidence::EvidenceRef;
use crate::policy::PolicyDecision;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IdentityType {
    Human,
    Agent,
    Service,
    Organization,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IdentityRef {
    pub id: String,
    pub identity_type: IdentityType,
    pub display_name: Option<String>,
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BoxType {
    Intent,
    Task,
    Decision,
    Event,
    Evidence,
    Approval,
    Reconciliation,
    Artifact,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BoxState {
    Proposed,
    PendingPolicy,
    RequiresApproval,
    Active,
    Blocked,
    Executed,
    Reconciled,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BoxRelation {
    Parent,
    Child,
    DependsOn,
    Blocks,
    Evidences,
    Supersedes,
    DerivedFrom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BoxLink {
    pub rel: BoxRelation,
    pub box_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FabricBox {
    pub box_id: String,
    pub box_type: BoxType,
    pub state: BoxState,
    pub owner: IdentityRef,
    pub actor: Option<IdentityRef>,
    pub objective: Option<String>,
    pub constraints: Vec<String>,
    pub context: serde_json::Value,
    pub policy: PolicyDecision,
    pub evidence: Vec<EvidenceRef>,
    pub links: Vec<BoxLink>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl FabricBox {
    pub fn transition(&mut self, next_state: BoxState) {
        self.state = next_state;
        self.updated_at = OffsetDateTime::now_utc();
    }

    pub fn attach_evidence(&mut self, evidence: EvidenceRef) {
        self.evidence.push(evidence);
        self.updated_at = OffsetDateTime::now_utc();
    }
}
