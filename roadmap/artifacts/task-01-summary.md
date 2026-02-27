# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 0.934 | 0.774 | 1.621 | 0.690 | 1.621 |
| health_check | 0.290 | 0.208 | 0.781 | 0.107 | 0.781 |
| get_stats | 1.955 | 1.521 | 4.018 | 1.252 | 4.018 |
| list_pending | 0.478 | 0.142 | 1.847 | 0.125 | 1.847 |
| list_unreplied_tweets_with_limit | 0.240 | 0.132 | 0.795 | 0.051 | 0.795 |

**Aggregate** — P50: 0.690 ms, P95: 1.847 ms, Min: 0.051 ms, Max: 4.018 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
