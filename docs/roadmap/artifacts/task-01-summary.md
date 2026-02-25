# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 0.879 | 0.919 | 1.144 | 0.524 | 1.144 |
| health_check | 0.195 | 0.162 | 0.298 | 0.154 | 0.298 |
| get_stats | 1.435 | 1.144 | 2.676 | 0.984 | 2.676 |
| list_pending | 0.233 | 0.129 | 0.649 | 0.103 | 0.649 |
| list_unreplied_tweets_with_limit | 0.223 | 0.123 | 0.639 | 0.095 | 0.639 |

**Aggregate** — P50: 0.298 ms, P95: 1.339 ms, Min: 0.095 ms, Max: 2.676 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
