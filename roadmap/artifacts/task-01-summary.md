# Task 01 — Baseline Benchmark

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| get_capabilities | 0.956 | 0.949 | 1.007 | 0.910 | 1.007 |
| health_check | 0.375 | 0.276 | 0.818 | 0.231 | 0.818 |
| get_stats | 1.769 | 1.512 | 3.172 | 1.236 | 3.172 |
| list_pending | 0.431 | 0.322 | 1.179 | 0.139 | 1.179 |
| list_unreplied_tweets_with_limit | 0.353 | 0.150 | 1.209 | 0.116 | 1.209 |

**Aggregate** — P50: 0.818 ms, P95: 1.609 ms, Min: 0.116 ms, Max: 3.172 ms

Migrated: 5 / 27 tools — Schema pass rate: 100%
