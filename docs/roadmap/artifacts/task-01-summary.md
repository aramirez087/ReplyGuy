# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 0.729 | 0.680 | 1.181 | 0.460 | 1.181 |
| health_check | 0.175 | 0.123 | 0.394 | 0.111 | 0.394 |
| get_stats | 1.391 | 0.994 | 3.088 | 0.845 | 3.088 |
| list_pending | 0.401 | 0.153 | 1.471 | 0.103 | 1.471 |
| list_unreplied_tweets_with_limit | 0.224 | 0.079 | 0.805 | 0.066 | 0.805 |

**Aggregate** — P50: 0.394 ms, P95: 1.471 ms, Min: 0.066 ms, Max: 3.088 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
