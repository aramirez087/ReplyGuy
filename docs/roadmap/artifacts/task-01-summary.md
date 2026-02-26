# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 0.903 | 0.805 | 1.289 | 0.668 | 1.289 |
| health_check | 0.410 | 0.331 | 0.682 | 0.209 | 0.682 |
| get_stats | 1.946 | 1.685 | 3.307 | 1.444 | 3.307 |
| list_pending | 0.519 | 0.258 | 1.694 | 0.139 | 1.694 |
| list_unreplied_tweets_with_limit | 0.273 | 0.142 | 0.782 | 0.130 | 0.782 |

**Aggregate** — P50: 0.668 ms, P95: 1.809 ms, Min: 0.130 ms, Max: 3.307 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
