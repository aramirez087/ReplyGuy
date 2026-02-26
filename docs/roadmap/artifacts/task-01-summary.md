# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 0.802 | 0.621 | 1.593 | 0.545 | 1.593 |
| health_check | 0.335 | 0.317 | 0.611 | 0.203 | 0.611 |
| get_stats | 1.984 | 1.616 | 3.532 | 1.416 | 3.532 |
| list_pending | 0.357 | 0.171 | 1.103 | 0.112 | 1.103 |
| list_unreplied_tweets_with_limit | 0.316 | 0.201 | 0.916 | 0.107 | 0.916 |

**Aggregate** — P50: 0.545 ms, P95: 1.756 ms, Min: 0.107 ms, Max: 3.532 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
