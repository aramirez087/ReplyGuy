# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 0.909 | 0.771 | 1.640 | 0.634 | 1.640 |
| health_check | 0.252 | 0.141 | 0.561 | 0.116 | 0.561 |
| get_stats | 1.422 | 1.180 | 2.544 | 1.008 | 2.544 |
| list_pending | 0.333 | 0.116 | 1.205 | 0.068 | 1.205 |
| list_unreplied_tweets_with_limit | 0.197 | 0.094 | 0.627 | 0.065 | 0.627 |

**Aggregate** — P50: 0.561 ms, P95: 1.640 ms, Min: 0.065 ms, Max: 2.544 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
