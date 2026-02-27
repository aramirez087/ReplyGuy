# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 0.993 | 0.820 | 1.811 | 0.623 | 1.811 |
| health_check | 0.276 | 0.277 | 0.405 | 0.179 | 0.405 |
| get_stats | 1.776 | 1.143 | 3.613 | 0.891 | 3.613 |
| list_pending | 0.380 | 0.220 | 1.041 | 0.156 | 1.041 |
| list_unreplied_tweets_with_limit | 0.319 | 0.168 | 1.009 | 0.098 | 1.009 |

**Aggregate** — P50: 0.405 ms, P95: 2.178 ms, Min: 0.098 ms, Max: 3.613 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
