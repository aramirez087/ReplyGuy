# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 1.011 | 0.886 | 1.330 | 0.763 | 1.330 |
| health_check | 0.291 | 0.216 | 0.692 | 0.107 | 0.692 |
| get_stats | 1.482 | 1.162 | 2.593 | 1.034 | 2.593 |
| list_pending | 0.292 | 0.153 | 0.848 | 0.141 | 0.848 |
| list_unreplied_tweets_with_limit | 0.226 | 0.101 | 0.751 | 0.081 | 0.751 |

**Aggregate** — P50: 0.692 ms, P95: 1.569 ms, Min: 0.081 ms, Max: 2.593 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
