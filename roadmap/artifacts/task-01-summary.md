# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 1.050 | 1.179 | 1.465 | 0.435 | 1.465 |
| health_check | 0.314 | 0.248 | 0.603 | 0.211 | 0.603 |
| get_stats | 1.795 | 1.736 | 2.671 | 1.290 | 2.671 |
| list_pending | 0.406 | 0.215 | 1.141 | 0.134 | 1.141 |
| list_unreplied_tweets_with_limit | 0.332 | 0.089 | 1.261 | 0.083 | 1.261 |

**Aggregate** — P50: 0.435 ms, P95: 1.879 ms, Min: 0.083 ms, Max: 2.671 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
