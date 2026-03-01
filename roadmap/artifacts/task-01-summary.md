# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 1.050 | 0.975 | 1.647 | 0.730 | 1.647 |
| health_check | 0.299 | 0.236 | 0.462 | 0.161 | 0.462 |
| get_stats | 1.522 | 1.407 | 2.241 | 1.155 | 2.241 |
| list_pending | 0.381 | 0.142 | 1.329 | 0.105 | 1.329 |
| list_unreplied_tweets_with_limit | 0.221 | 0.161 | 0.534 | 0.106 | 0.534 |

**Aggregate** — P50: 0.462 ms, P95: 1.647 ms, Min: 0.105 ms, Max: 2.241 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
