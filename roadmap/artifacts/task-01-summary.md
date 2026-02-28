# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 1.080 | 0.933 | 1.688 | 0.760 | 1.688 |
| health_check | 0.318 | 0.215 | 0.667 | 0.126 | 0.667 |
| get_stats | 1.542 | 1.121 | 3.437 | 0.924 | 3.437 |
| list_pending | 0.374 | 0.148 | 1.279 | 0.118 | 1.279 |
| list_unreplied_tweets_with_limit | 0.237 | 0.087 | 0.785 | 0.076 | 0.785 |

**Aggregate** — P50: 0.667 ms, P95: 1.688 ms, Min: 0.076 ms, Max: 3.437 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
