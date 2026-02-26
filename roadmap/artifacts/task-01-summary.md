# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 0.467 | 0.445 | 0.552 | 0.424 | 0.552 |
| health_check | 0.068 | 0.062 | 0.103 | 0.054 | 0.103 |
| get_stats | 0.410 | 0.391 | 0.499 | 0.365 | 0.499 |
| list_pending | 0.071 | 0.054 | 0.143 | 0.046 | 0.143 |
| list_unreplied_tweets_with_limit | 0.058 | 0.051 | 0.091 | 0.046 | 0.091 |

**Aggregate** — P50: 0.091 ms, P95: 0.499 ms, Min: 0.046 ms, Max: 0.552 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
