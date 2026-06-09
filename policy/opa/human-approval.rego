package fabric.agent.approval

# Human approval policy for Fabric Agent.
# High-risk, irreversible, externally visible, or governance-changing actions
# must pass through a human gate before execution.

default requires_human_approval := false

requires_human_approval if {
  input.action in high_risk_actions
}

requires_human_approval if {
  input.box.state == "requires_approval"
}

requires_human_approval if {
  input.risk.level in {"high", "critical"}
}

requires_human_approval if {
  input.external_visibility == true
}

requires_human_approval if {
  input.irreversible == true
}

requires_human_approval if {
  input.policy.override_requested == true
}

approval_reason contains "high-risk action" if {
  input.action in high_risk_actions
}

approval_reason contains "box is already waiting for approval" if {
  input.box.state == "requires_approval"
}

approval_reason contains "high or critical risk" if {
  input.risk.level in {"high", "critical"}
}

approval_reason contains "externally visible action" if {
  input.external_visibility == true
}

approval_reason contains "irreversible action" if {
  input.irreversible == true
}

approval_reason contains "policy override requested" if {
  input.policy.override_requested == true
}

high_risk_actions := {
  "tool.execute",
  "state.mutate",
  "external.send",
  "secret.read",
  "payment.initiate",
  "identity.issue",
  "identity.revoke",
  "policy.override",
  "fabric.admin",
  "box.archive"
}
