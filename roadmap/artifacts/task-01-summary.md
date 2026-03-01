# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 0.904 | 0.813 | 1.344 | 0.700 | 1.344 |
| health_check | 0.231 | 0.205 | 0.473 | 0.102 | 0.473 |
| get_stats | 1.627 | 1.325 | 3.257 | 0.903 | 3.257 |
| list_pending | 0.292 | 0.162 | 0.894 | 0.074 | 0.894 |
| list_unreplied_tweets_with_limit | 0.244 | 0.126 | 0.801 | 0.072 | 0.801 |

**Aggregate** — P50: 0.473 ms, P95: 1.563 ms, Min: 0.072 ms, Max: 3.257 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
