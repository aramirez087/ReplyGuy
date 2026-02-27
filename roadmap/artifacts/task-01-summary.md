# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 0.971 | 0.818 | 1.593 | 0.717 | 1.593 |
| health_check | 0.327 | 0.196 | 0.746 | 0.178 | 0.746 |
| get_stats | 1.531 | 1.248 | 2.642 | 1.125 | 2.642 |
| list_pending | 0.378 | 0.141 | 1.304 | 0.103 | 1.304 |
| list_unreplied_tweets_with_limit | 0.342 | 0.126 | 1.200 | 0.115 | 1.200 |

**Aggregate** — P50: 0.717 ms, P95: 1.593 ms, Min: 0.103 ms, Max: 2.642 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
