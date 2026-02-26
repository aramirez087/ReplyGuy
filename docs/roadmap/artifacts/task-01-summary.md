# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 1.231 | 0.924 | 1.920 | 0.779 | 1.920 |
| health_check | 0.743 | 0.674 | 1.412 | 0.290 | 1.412 |
| get_stats | 4.435 | 4.377 | 6.857 | 2.349 | 6.857 |
| list_pending | 0.909 | 0.669 | 2.213 | 0.428 | 2.213 |
| list_unreplied_tweets_with_limit | 0.393 | 0.251 | 1.044 | 0.202 | 1.044 |

**Aggregate** — P50: 0.794 ms, P95: 5.828 ms, Min: 0.202 ms, Max: 6.857 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
