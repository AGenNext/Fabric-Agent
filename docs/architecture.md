# Fabric Agent Architecture

Fabric Agent is the operating framework for governed agent work. It is not a single chatbot, worker, or tool-calling loop. It is the framework layer that makes agent work accountable, inspectable, policy-bound, and reconcilable.

## Core distinction

```text
Fabric       = governed work and data surface
Box          = atomic accountable unit
Fabric Agent = operating framework for governed agent execution
Agent        = runtime/operator instance inside the framework
Human Gate   = approval, accountability, and override boundary
```

The agent is replaceable. The fabric and its governance loop are the durable system.

## Layer model

```text
Fabric Agent
├── Identity Layer
├── Access Layer
├── Policy Layer
├── Context Layer
├── Box Layer
├── Event Layer
├── Evidence Layer
├── Reconciliation Layer
└── Human Approval Layer
```

### Identity Layer

Every actor must have an accountable identity. An actor may be a human, agent, service, or organization.

The identity layer answers:

- who is acting;
- who owns the box;
- who is responsible;
- which scopes are available;
- which credentials or attestations are valid.

### Access Layer

The access layer defines relationships between users, agents, organizations, fabrics, boxes, and evidence.

The default model is relationship-based access control using OpenFGA-style tuples.

### Policy Layer

The policy layer evaluates whether an action is allowed, denied, or requires human approval.

The default posture is deny-by-default.

### Context Layer

The context layer resolves the information needed for the agent to act. Context must be scoped, explainable, and policy-filtered.

### Box Layer

The box is the smallest accountable unit of work, state, decision, evidence, event, and reconciliation.

A box contains:

```text
Identity
Objective
Context
Constraints
Policy
Evidence
Links
State
Time
```

### Event Layer

Every meaningful state transition emits a Fabric Event. Events are append-only records that make the workstream auditable.

### Evidence Layer

Evidence attaches proof to actions. Evidence can include logs, traces, files, approvals, diffs, API responses, and attestations.

### Reconciliation Layer

The reconciliation layer compares desired state, observed state, policy state, and evidence state.

```text
Desired State
    +
Observed State
    +
Policy Decision
    +
Evidence
    =
Reconciled State
```

### Human Approval Layer

High-risk, ambiguous, irreversible, or externally visible actions must be routed to a human gate.

## Canonical loop

```text
observe -> decide -> act -> record -> reconcile -> report
```

Expanded:

```text
Intent
  ↓
Context Resolution
  ↓
Identity Check
  ↓
Access Check
  ↓
Policy Check
  ↓
Approval Gate
  ↓
Plan
  ↓
Action
  ↓
Evidence
  ↓
Event
  ↓
Reconciliation
  ↓
State
```

## What Fabric Agent is not

Fabric Agent is not:

- only a prompt wrapper;
- only a tool runner;
- only a chatbot;
- only an agent SDK;
- only a Kubernetes operator;
- only a workflow engine.

It is the governance and execution framework that can host all of these safely.

## Implementation direction

The canonical implementation direction is:

```text
Contracts  -> OpenAPI and JSON Schema
Access     -> OpenFGA-style relationship model
Policy     -> OPA/Rego deny-by-default rules
Runtime    -> Rust core reconciler
Operator   -> Kubernetes-native controller
SDKs       -> TypeScript, Go, and Rust clients
```

## Design rule

Agents do not directly mutate the world. Agents submit intended actions into the Fabric Agent framework. The framework checks identity, access, policy, approval, evidence, and reconciliation before state becomes accepted.
