# Task 09 Prompt: Hardening, Release, and Positioning

## Objective

Finalize production readiness, publish benchmark-backed differentiation, and package the release narrative that clearly outclasses thin MCP wrappers.

## Strategic Rationale

This step converts engineering work into market credibility and adoption.

## Dependencies

- All prior roadmap tasks in this folder

## Prompt To Run In Claude Code

```text
You are implementing Task 09 of the MCP superiority roadmap.

Goal:
Perform final hardening, doc updates, release notes, and comparative positioning.

Read first:
- docs/mcp-reference.md
- docs/architecture.md
- README.md
- CHANGELOG.md
- docs/roadmap/artifacts/*

Implementation requirements:
1) Test and hardening sweep:
   - run full relevant cargo tests
   - run dashboard/plugin build checks
   - identify and fix any MCP regression uncovered
2) Documentation updates:
   - create/refresh capability matrix: TuitBot vs x-v2-server
   - include explicit notes on safety, approval, analytics, and composite workflows
   - add migration guidance for users currently using thin X MCP wrappers
3) Release notes and changelog:
   - summarize major MCP enhancements and risk controls
4) Final benchmark report:
   - consolidate Task 01 and Task 07 artifacts into one executive summary
   - output: docs/roadmap/artifacts/final-mcp-superiority-report.md
5) Prepare a launch checklist with go/no-go criteria:
   - schema stability
   - policy enforcement
   - endpoint support coverage
   - operational metrics visibility

Constraints:
- Claims must be backed by implemented capabilities and artifacts.
- Keep language factual and defensible.

Validation:
- cargo test --workspace
- npm --prefix dashboard run build
- npm --prefix plugins/openclaw-tuitbot run build

Deliverables:
- updated docs and changelog
- final superiority report artifact
- release checklist markdown
```

## PM Acceptance Checklist

- Capability matrix is concrete and evidence-backed.
- Final report exists and cites benchmark artifacts.
- Go/no-go checklist is complete and actionable.
