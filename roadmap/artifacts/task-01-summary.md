# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 1.068 | 0.975 | 1.685 | 0.654 | 1.685 |
| health_check | 0.262 | 0.199 | 0.580 | 0.129 | 0.580 |
| get_stats | 1.562 | 1.346 | 2.735 | 0.983 | 2.735 |
| list_pending | 0.391 | 0.131 | 1.524 | 0.064 | 1.524 |
| list_unreplied_tweets_with_limit | 0.290 | 0.113 | 1.036 | 0.072 | 1.036 |

**Aggregate** — P50: 0.580 ms, P95: 1.685 ms, Min: 0.064 ms, Max: 2.735 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
