//! Fabric Agent Runtime
//!
//! Reference Rust runtime skeleton for the Fabric Agent operating framework.
//! The runtime models the core loop:
//!
//! observe -> decide -> act -> record -> reconcile -> report

pub mod box_model;
pub mod event;
pub mod evidence;
pub mod policy;
pub mod reconciler;

pub use box_model::{BoxState, BoxType, FabricBox, IdentityRef};
pub use event::{EventType, FabricEvent};
pub use evidence::{EvidenceKind, EvidenceRef};
pub use policy::{PolicyDecision, PolicyEffect};
pub use reconciler::{ReconcileInput, ReconcileOutcome, Reconciler};
