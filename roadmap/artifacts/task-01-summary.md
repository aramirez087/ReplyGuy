# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 0.938 | 0.884 | 1.388 | 0.676 | 1.388 |
| health_check | 0.339 | 0.215 | 0.711 | 0.204 | 0.711 |
| get_stats | 1.821 | 1.441 | 2.944 | 1.335 | 2.944 |
| list_pending | 0.503 | 0.189 | 1.817 | 0.118 | 1.817 |
| list_unreplied_tweets_with_limit | 0.344 | 0.172 | 1.023 | 0.148 | 1.023 |

**Aggregate** — P50: 0.676 ms, P95: 2.027 ms, Min: 0.118 ms, Max: 2.944 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
