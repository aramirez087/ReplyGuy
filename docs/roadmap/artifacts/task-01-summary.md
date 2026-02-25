# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 0.777 | 0.752 | 1.040 | 0.588 | 1.040 |
| health_check | 0.165 | 0.109 | 0.397 | 0.095 | 0.397 |
| get_stats | 1.103 | 0.997 | 1.751 | 0.815 | 1.751 |
| list_pending | 0.443 | 0.121 | 1.768 | 0.100 | 1.768 |
| list_unreplied_tweets_with_limit | 0.279 | 0.141 | 0.814 | 0.111 | 0.814 |

**Aggregate** — P50: 0.397 ms, P95: 1.751 ms, Min: 0.095 ms, Max: 1.768 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
