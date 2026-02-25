# PRD: TuitBot MCP Superiority Program

## 1. Document Control

- Product: TuitBot MCP + OpenClaw plugin + Dashboard agent controls
- PRD owner: Product Management
- Engineering owner: Core + MCP + Dashboard leads
- Date: 2026-02-25
- Status: Draft for execution

## 2. Executive Summary

TuitBot is already stronger than a thin MCP wrapper for autonomous growth workflows, but it is weaker in one critical perception dimension: direct, flexible, real-time X actions exposed as MCP tools.

`x-v2-server` is attractive because it gives agents many immediate levers (mentions, search, reply, quote, follow, lists). TuitBot must absorb this flexibility while keeping its differentiated moat: safety, approval, scoring, analytics, and strategic automation.

The objective is to position TuitBot as:
- The most capable X MCP action layer for agents
- The safest production-grade execution stack for X automation
- The only offering with built-in growth intelligence loops and closed-loop analytics

## 3. Competitive Analysis

### 3.1 x-v2-server (NexusX-MCP)

Observed strengths:
- Broad direct tool surface for low-level actions.
- Easy mental model: each tool maps to a known X action.
- Fast to integrate into agent workflows.

Observed weaknesses:
- Thin abstraction over endpoints; minimal orchestration intelligence.
- No policy engine, approval queue, or configurable safety gates.
- Limited observability and no growth outcome optimization loop.
- Sparse testing and lifecycle hardening compared to production automation systems.

### 3.2 TuitBot (Current)

Observed strengths:
- Mature automation loops (discovery, mentions, content, analytics, thread, approval poster).
- Configurable safety and dedup guardrails.
- Approval-first operations and Composer mode.
- Analytics and strategic reporting built in.
- Existing MCP server with operational tools and content generation support.

Observed weaknesses relative to x-v2-server’s perceived flexibility:
- Missing direct action MCP tools for common agent actions (follow/unfollow/like/quote/list ops/trends).
- MCP responses are often plain text JSON strings; schema contracts are inconsistent for agents.
- Limited task-oriented composite tools that bundle reasoning + policy + execution.
- Limited first-class “agent control center” UI for MCP policy and execution telemetry.

## 4. Problem Statement

Agents and developers evaluating X tooling perceive “more direct tools” as more capable. TuitBot’s deeper strengths are under-leveraged in MCP-centric workflows, reducing adoption in the tooling-layer segment.

## 5. Product Vision

Build a “Safe Action Graph MCP” where:
- Every direct action is available when needed.
- Every action can be policy-governed and approval-gated.
- Agents can choose low-level primitives or high-level growth workflows.
- Results are measurable, auditable, and continuously optimized.

## 6. Goals and Non-Goals

### Goals

- G1: Achieve direct tool parity-plus with `x-v2-server` for practical agent usage.
- G2: Add policy and approval controls on top of direct tools.
- G3: Launch composite, outcome-oriented MCP tools (growth workflows).
- G4: Standardize deterministic MCP response contracts.
- G5: Add observability/evals proving better quality and safety outcomes.

### Non-Goals

- NG1: No auto-follow/like spam behavior or manipulation loops.
- NG2: No removal of existing approval/safety defaults in Composer mode.
- NG3: No breaking existing CLI and dashboard workflows.

## 7. Target Personas

- P1: Agent builders integrating OpenClaw/Codex/Claude-based systems.
- P2: Technical founders wanting both direct controls and safe automation.
- P3: Operators managing multiple growth experiments with auditability needs.

## 8. Jobs To Be Done

- “When I run an agent on X, I need flexible actions and reliable guardrails so I can automate without risking account health.”
- “When I test growth workflows, I need measurable outcomes and explainable logs so I can improve strategy.”
- “When I delegate to an AI assistant, I need deterministic tool contracts so execution is predictable.”

## 9. Requirements

### Functional Requirements

- FR1: Provide MCP tools for direct X actions comparable to x-v2-server and practical extras.
- FR2: Add centralized policy enforcement for all mutation tools.
- FR3: Support optional approval queue routing for high-risk mutation actions.
- FR4: Add composite tools that combine search/context/scoring/draft/propose-execute.
- FR5: Provide context-enriched recommendation tools using TuitBot historical data.
- FR6: Add capability introspection with policy/rate-limit/approval visibility.
- FR7: Add structured result envelopes with stable machine-friendly schema.
- FR8: Expose execution telemetry and error taxonomy for operators and agents.
- FR9: Extend OpenClaw plugin mapping with policy metadata and clearer failure reasons.
- FR10: Add dashboard controls for MCP policy, approvals, and recent executions.

### Non-Functional Requirements

- NFR1: Backward compatible with current MCP tools where feasible.
- NFR2: New tools must include tests (unit + integration where applicable).
- NFR3: Safety checks must execute before any side-effecting X action.
- NFR4: Latency target for direct read tools: p95 under 1.5s (excluding network volatility).
- NFR5: Deterministic JSON contracts, no unstructured prose-only outputs.

## 10. Success Metrics

Primary:
- M1: MCP adoption lift (weekly active tool calls) by 2x after launch.
- M2: Mutation tool success rate > 98% (excluding external API outages).
- M3: Policy block + approval routing correctness 100% in test suite.

Quality/Safety:
- M4: Zero regressions in dedup and rate-limit safeguards.
- M5: < 1% unknown-error bucket in MCP failures.
- M6: At least 90% of tool responses validated against explicit schema.

Competitive:
- M7: Capability matrix demonstrates parity-plus vs x-v2-server on practical workflows.
- M8: Benchmark scenario completion time improved by 25% through composite tools.

## 11. Scope by Release

### R1: Competitive Baseline (Parity + Contracts)

- Direct action tools (read + write)
- Structured response envelope
- Initial policy framework scaffold

### R2: Differentiation Layer

- Approval-aware mutation gateway
- Composite goal-oriented tools
- Context memory/recommendation tools

### R3: Operational Excellence

- Observability + eval harness + benchmark report
- OpenClaw plugin upgrades
- Dashboard control center

## 12. Risks and Mitigations

- Risk: “Parity sprint” could erode safety posture.
  - Mitigation: Mutation tools must pass through shared policy gateway.

- Risk: API tier variance on X endpoints.
  - Mitigation: Capability detection + explicit unavailable/error semantics per endpoint.

- Risk: Agent confusion due to dual modes.
  - Mitigation: `get_capabilities` and `get_mode` enriched with policy and recommended action paths.

- Risk: Increased maintenance complexity.
  - Mitigation: Shared tool response contracts + centralized adapters + test matrix.

## 13. Acceptance Criteria for Program Completion

- AC1: Direct tool suite implemented with policy and schema contracts.
- AC2: Composite tools outperform raw primitives in benchmark workflows.
- AC3: OpenClaw plugin exposes improved usability and error handling.
- AC4: Dashboard includes MCP governance and execution visibility.
- AC5: Documentation clearly positions TuitBot as safer and more outcome-driven than thin MCP wrappers.

## 14. Execution Artifacts

Use the numbered task prompts in this folder:
- [`01-PROMPT-baseline-contracts-and-benchmarks.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/01-PROMPT-baseline-contracts-and-benchmarks.md)
- [`02-PROMPT-direct-x-tools-parity-plus.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/02-PROMPT-direct-x-tools-parity-plus.md)
- [`03-PROMPT-safety-policy-and-approval-gateway.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/03-PROMPT-safety-policy-and-approval-gateway.md)
- [`04-PROMPT-composite-goal-oriented-tools.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/04-PROMPT-composite-goal-oriented-tools.md)
- [`05-PROMPT-context-memory-and-recommendation-engine.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/05-PROMPT-context-memory-and-recommendation-engine.md)
- [`06-PROMPT-openclaw-integration-upgrade.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/06-PROMPT-openclaw-integration-upgrade.md)
- [`07-PROMPT-observability-evals-and-quality-gates.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/07-PROMPT-observability-evals-and-quality-gates.md)
- [`08-PROMPT-dashboard-control-center-and-ux.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/08-PROMPT-dashboard-control-center-and-ux.md)
- [`09-PROMPT-hardening-release-and-positioning.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/09-PROMPT-hardening-release-and-positioning.md)
