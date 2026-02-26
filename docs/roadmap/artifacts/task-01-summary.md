# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 1.067 | 0.807 | 1.819 | 0.675 | 1.819 |
| health_check | 0.416 | 0.355 | 0.663 | 0.204 | 0.663 |
| get_stats | 5.443 | 5.982 | 7.477 | 3.367 | 7.477 |
| list_pending | 1.276 | 1.262 | 2.446 | 0.472 | 2.446 |
| list_unreplied_tweets_with_limit | 0.490 | 0.234 | 1.476 | 0.164 | 1.476 |

**Aggregate** — P50: 0.805 ms, P95: 6.062 ms, Min: 0.164 ms, Max: 7.477 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
