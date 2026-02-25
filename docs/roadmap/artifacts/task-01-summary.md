# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 0.920 | 0.658 | 1.963 | 0.609 | 1.963 |
| health_check | 0.198 | 0.207 | 0.261 | 0.130 | 0.261 |
| get_stats | 1.643 | 1.326 | 3.253 | 0.887 | 3.253 |
| list_pending | 0.391 | 0.177 | 1.117 | 0.170 | 1.117 |
| list_unreplied_tweets_with_limit | 0.285 | 0.137 | 0.884 | 0.109 | 0.884 |

**Aggregate** — P50: 0.318 ms, P95: 1.963 ms, Min: 0.109 ms, Max: 3.253 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
