# Session 10: Admin/Ads/DM Boundaries and Positioning

Paste this into a new Claude Code session:

```md
Continue from Session 09 artifacts.

Mission:
Eliminate ambiguity around high-risk/high-expectation surfaces (Ads, DMs, admin capabilities) in both product behavior and documentation.

Primary files:
- `docs/mcp-reference.md`
- `docs/configuration.md`
- `docs/getting-started.md`
- `README.md`
- MCP profile and capability files touched in Sessions 05-06

Tasks:
1. Document exactly what "admin" includes and excludes.
2. Make Ads and DMs support status explicit:
   - available/not available
   - required credentials/configuration
   - enterprise gating caveats
3. Ensure profile registration reflects documentation:
   - hidden if not configured/authorized
4. Align product positioning language:
   - if needed, phrase as "max public API coverage" rather than overpromising "everything".

Deliverables:
1. Documentation updates across core docs.
2. Minor code/policy updates if docs and runtime behavior diverge.
3. `docs/roadmap/x-api-surface-expansion/session-10-handoff.md` with:
   - final support matrix
   - explicit non-goals
   - risk communication notes

Quality gates:
- `cargo fmt --all && cargo fmt --all --check`
- `RUSTFLAGS="-D warnings" cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`

Exit criteria:
- No ambiguity on Ads/DM/admin support.
- Runtime capability gating matches documentation.
- Positioning is ambitious but truthful.
```

