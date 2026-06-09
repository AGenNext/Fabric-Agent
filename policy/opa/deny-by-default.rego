package fabric.agent.authz

# Fabric Agent policy baseline.
# Default posture: deny every action unless an explicit allow rule matches.

default allow := false

default requires_approval := false

allow if {
  input.actor.id != ""
  input.actor.type in {"human", "agent", "service", "organization"}
  input.action in allowed_actions
  input.box.state != "archived"
  not high_risk_action
  not missing_required_scope
}

requires_approval if {
  high_risk_action
}

requires_approval if {
  input.policy.effect == "requires_approval"
}

deny_reason contains "missing actor identity" if {
  input.actor.id == ""
}

deny_reason contains "unsupported actor type" if {
  not input.actor.type in {"human", "agent", "service", "organization"}
}

deny_reason contains "action not explicitly allowed" if {
  not input.action in allowed_actions
}

deny_reason contains "archived boxes cannot be mutated" if {
  input.box.state == "archived"
}

deny_reason contains "missing required scope" if {
  missing_required_scope
}

allowed_actions := {
  "intent.receive",
  "context.resolve",
  "policy.check",
  "plan.create",
  "evidence.attach",
  "state.read",
  "reconciliation.start",
  "reconciliation.complete",
  "report.generate"
}

high_risk_actions := {
  "tool.execute",
  "state.mutate",
  "external.send",
  "secret.read",
  "payment.initiate",
  "identity.issue",
  "policy.override"
}

high_risk_action if {
  input.action in high_risk_actions
}

missing_required_scope if {
  required := input.required_scope
  required != ""
  not required in input.actor.scopes
}
