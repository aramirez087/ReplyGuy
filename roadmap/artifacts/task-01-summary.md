# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 0.982 | 0.824 | 1.452 | 0.781 | 1.452 |
| health_check | 0.202 | 0.188 | 0.321 | 0.153 | 0.321 |
| get_stats | 1.547 | 1.221 | 2.939 | 1.162 | 2.939 |
| list_pending | 0.449 | 0.170 | 1.547 | 0.118 | 1.547 |
| list_unreplied_tweets_with_limit | 0.384 | 0.186 | 1.198 | 0.158 | 1.198 |

**Aggregate** — P50: 0.321 ms, P95: 1.547 ms, Min: 0.118 ms, Max: 2.939 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
