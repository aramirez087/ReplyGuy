# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 1.009 | 0.854 | 1.836 | 0.672 | 1.836 |
| health_check | 0.380 | 0.302 | 0.700 | 0.241 | 0.700 |
| get_stats | 2.057 | 1.679 | 3.555 | 1.536 | 3.555 |
| list_pending | 0.476 | 0.225 | 1.586 | 0.142 | 1.586 |
| list_unreplied_tweets_with_limit | 0.271 | 0.170 | 0.760 | 0.097 | 0.760 |

**Aggregate** — P50: 0.672 ms, P95: 1.920 ms, Min: 0.097 ms, Max: 3.555 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
