# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 0.878 | 0.851 | 1.210 | 0.593 | 1.210 |
| health_check | 0.366 | 0.335 | 0.579 | 0.251 | 0.579 |
| get_stats | 2.095 | 1.513 | 4.295 | 1.378 | 4.295 |
| list_pending | 0.468 | 0.222 | 1.596 | 0.119 | 1.596 |
| list_unreplied_tweets_with_limit | 0.238 | 0.135 | 0.717 | 0.081 | 0.717 |

**Aggregate** — P50: 0.579 ms, P95: 1.859 ms, Min: 0.081 ms, Max: 4.295 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
