# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 0.950 | 0.828 | 1.579 | 0.667 | 1.579 |
| health_check | 0.310 | 0.214 | 0.627 | 0.185 | 0.627 |
| get_stats | 1.909 | 1.644 | 3.292 | 1.094 | 3.292 |
| list_pending | 0.376 | 0.134 | 1.419 | 0.086 | 1.419 |
| list_unreplied_tweets_with_limit | 0.292 | 0.118 | 0.980 | 0.082 | 0.980 |

**Aggregate** — P50: 0.627 ms, P95: 2.084 ms, Min: 0.082 ms, Max: 3.292 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
