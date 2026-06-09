# Fabric Agent

Fabric Agent is the reference agent for the AGenNext Fabric model: a policy-first, identity-aware, event-driven agent that operates inside a governed fabric instead of acting as an unbounded script runner.

The core idea is simple:

- **Fabric** is the governed work/data surface.
- **Box** is the smallest accountable unit of work, state, event, evidence, and decision.
- **Agent** is the operator that reads context, follows policy, executes tools, records evidence, and reconciles state.
- **Human gate** stays available for approval, override, escalation, and accountability.

This repository defines the canonical structure for Fabric Agent implementations.

## Why this exists

Most agent systems fail when the agent is allowed to act without a stable governance substrate. Fabric Agent treats every action as part of a reconciled loop:

```text
Intent -> Context -> Policy -> Plan -> Action -> Evidence -> Reconcile -> State
```

The agent does not own the fabric. The agent operates inside the fabric.

## Principles

1. **Deny by default** — no action without explicit capability, policy, identity, and scope.
2. **Identity-bound execution** — every agent, tool, box, and action must have an accountable identity.
3. **Evidence-first operation** — actions create auditable evidence before they become accepted state.
4. **Human at the gate** — high-risk or ambiguous decisions require approval.
5. **Reconciliation loop** — desired state and observed state must be compared continuously.
6. **Portable implementation** — the model can be implemented with multiple runtimes, not one vendor stack.
7. **Open source commercial core** — the code and contracts are open; delivery and operations can be commercial.

## Repository map

```text
.
├── README.md
├── docs/
│   ├── architecture.md
│   ├── box-model.md
│   ├── governance-loop.md
│   └── implementation-notes.md
├── contracts/
│   ├── fabric-agent.openapi.yaml
│   └── fabric-agent.schema.json
├── examples/
│   └── box-event.json
├── .github/
│   ├── workflows/ci.yml
│   └── pull_request_template.md
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md
├── GOVERNANCE.md
├── SECURITY.md
└── LICENSE
```

## Minimal Fabric Agent contract

A Fabric Agent must be able to:

- receive an intent;
- resolve context;
- check identity, scope, and policy;
- create or update a box;
- execute approved actions;
- attach evidence;
- reconcile expected and observed state;
- emit an auditable event.

## Canonical loop

```text
observe -> decide -> act -> record -> reconcile -> report
```

The agent is not just an instruction follower. It is an accountable operator in a governed work fabric.

## Status

Draft v0.1. This repository is suitable for review, experimentation, and implementation alignment. Breaking changes are expected until v1.0.

## License

Apache License 2.0. See [LICENSE](LICENSE).
