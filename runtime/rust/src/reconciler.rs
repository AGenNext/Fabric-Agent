use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::box_model::{BoxState, FabricBox};
use crate::event::{EventType, FabricEvent};
use crate::policy::{PolicyDecision, PolicyEffect};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconcileInput {
    pub desired: FabricBox,
    pub observed: Option<FabricBox>,
    pub policy: PolicyDecision,
    pub sequence: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconcileOutcome {
    pub box_state: FabricBox,
    pub events: Vec<FabricEvent>,
    pub accepted: bool,
    pub requires_approval: bool,
    pub reason: Option<String>,
}

#[derive(Debug, Default)]
pub struct Reconciler;

impl Reconciler {
    pub fn new() -> Self {
        Self
    }

    pub fn reconcile(&self, input: ReconcileInput) -> ReconcileOutcome {
        let mut box_state = input.desired;
        let actor = box_state
            .actor
            .clone()
            .unwrap_or_else(|| box_state.owner.clone());

        let mut events = Vec::new();

        events.push(
            FabricEvent::new(
                box_state.box_id.clone(),
                EventType::ReconciliationStarted,
                actor.clone(),
                input.sequence,
                json!({
                    "observed": input.observed.is_some(),
                    "state": format!("{:?}", box_state.state)
                }),
            )
            .with_policy(input.policy.clone()),
        );

        match input.policy.effect {
            PolicyEffect::Allow => {
                box_state.transition(BoxState::Reconciled);

                events.push(
                    FabricEvent::new(
                        box_state.box_id.clone(),
                        EventType::ReconciliationCompleted,
                        actor,
                        input.sequence + 1,
                        json!({
                            "result": "accepted",
                            "state": "Reconciled"
                        }),
                    )
                    .with_policy(input.policy.clone()),
                );

                ReconcileOutcome {
                    box_state,
                    events,
                    accepted: true,
                    requires_approval: false,
                    reason: input.policy.reason,
                }
            }
            PolicyEffect::RequiresApproval => {
                box_state.transition(BoxState::RequiresApproval);

                events.push(
                    FabricEvent::new(
                        box_state.box_id.clone(),
                        EventType::ApprovalRequested,
                        actor,
                        input.sequence + 1,
                        json!({
                            "result": "requires_approval",
                            "state": "RequiresApproval"
                        }),
                    )
                    .with_policy(input.policy.clone()),
                );

                ReconcileOutcome {
                    box_state,
                    events,
                    accepted: false,
                    requires_approval: true,
                    reason: input.policy.reason,
                }
            }
            PolicyEffect::Deny => {
                box_state.transition(BoxState::Blocked);

                events.push(
                    FabricEvent::new(
                        box_state.box_id.clone(),
                        EventType::ActionFailed,
                        actor,
                        input.sequence + 1,
                        json!({
                            "result": "denied",
                            "state": "Blocked"
                        }),
                    )
                    .with_policy(input.policy.clone()),
                );

                ReconcileOutcome {
                    box_state,
                    events,
                    accepted: false,
                    requires_approval: false,
                    reason: input.policy.reason,
                }
            }
        }
    }
}
