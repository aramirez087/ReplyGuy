# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 0.914 | 0.865 | 1.300 | 0.753 | 1.300 |
| health_check | 0.240 | 0.217 | 0.455 | 0.126 | 0.455 |
| get_stats | 1.621 | 1.360 | 2.699 | 1.017 | 2.699 |
| list_pending | 0.396 | 0.156 | 1.398 | 0.126 | 1.398 |
| list_unreplied_tweets_with_limit | 0.245 | 0.131 | 0.760 | 0.088 | 0.760 |

**Aggregate** — P50: 0.455 ms, P95: 1.977 ms, Min: 0.088 ms, Max: 2.699 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
