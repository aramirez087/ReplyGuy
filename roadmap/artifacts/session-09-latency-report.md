# Session 09 — Latency Report

**Generated:** 2026-02-26 19:24 UTC

**Tools benchmarked:** 16

## Per-tool Results

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| kernel::get_tweet | 0.032 | 0.026 | 0.054 | 0.020 | 0.054 |
| kernel::search_tweets | 0.026 | 0.017 | 0.057 | 0.013 | 0.057 |
| kernel::get_followers | 0.016 | 0.013 | 0.032 | 0.006 | 0.032 |
| kernel::get_user_by_id | 0.012 | 0.007 | 0.030 | 0.007 | 0.030 |
| kernel::get_me | 0.011 | 0.008 | 0.023 | 0.007 | 0.023 |
| kernel::post_tweet | 0.004 | 0.003 | 0.007 | 0.003 | 0.007 |
| kernel::reply_to_tweet | 0.004 | 0.003 | 0.004 | 0.003 | 0.004 |
| score_tweet | 0.017 | 0.012 | 0.035 | 0.012 | 0.035 |
| get_config | 0.098 | 0.080 | 0.153 | 0.079 | 0.153 |
| validate_config | 0.013 | 0.009 | 0.025 | 0.009 | 0.025 |
| get_mcp_tool_metrics | 0.249 | 0.191 | 0.438 | 0.180 | 0.438 |
| get_mcp_error_breakdown | 0.064 | 0.056 | 0.092 | 0.052 | 0.092 |
| get_capabilities | 0.386 | 0.368 | 0.473 | 0.358 | 0.473 |
| health_check | 0.063 | 0.057 | 0.085 | 0.056 | 0.085 |
| get_stats | 0.408 | 0.350 | 0.583 | 0.317 | 0.583 |
| list_pending | 0.066 | 0.054 | 0.115 | 0.048 | 0.115 |

## Category Breakdown

| Category | Tools | P95 (ms) |
|----------|-------|----------|
| Kernel read | 5 | 0.054 |
| Kernel write | 2 | 0.007 |
| Config | 3 | 0.153 |
| Telemetry | 2 | 0.438 |

## Aggregate

**P50:** 0.032 ms | **P95:** 0.371 ms | **Min:** 0.003 ms | **Max:** 0.583 ms

## P95 Gate

**Global P95:** 0.371 ms
**Threshold:** 50.0 ms
**Status:** PASS
