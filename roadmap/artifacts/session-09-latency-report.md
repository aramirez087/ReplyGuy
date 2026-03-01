# Session 09 — Latency Report

**Generated:** 2026-03-01 01:23 UTC

**Tools benchmarked:** 16

## Per-tool Results

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| kernel::get_tweet | 0.025 | 0.021 | 0.037 | 0.021 | 0.037 |
| kernel::search_tweets | 0.015 | 0.014 | 0.019 | 0.014 | 0.019 |
| kernel::get_followers | 0.012 | 0.012 | 0.014 | 0.011 | 0.014 |
| kernel::get_user_by_id | 0.014 | 0.014 | 0.015 | 0.013 | 0.015 |
| kernel::get_me | 0.014 | 0.014 | 0.016 | 0.013 | 0.016 |
| kernel::post_tweet | 0.008 | 0.007 | 0.010 | 0.007 | 0.010 |
| kernel::reply_to_tweet | 0.010 | 0.008 | 0.023 | 0.004 | 0.023 |
| score_tweet | 0.019 | 0.012 | 0.044 | 0.012 | 0.044 |
| get_config | 0.089 | 0.085 | 0.101 | 0.084 | 0.101 |
| validate_config | 0.056 | 0.011 | 0.238 | 0.010 | 0.238 |
| get_mcp_tool_metrics | 1.250 | 0.666 | 2.937 | 0.518 | 2.937 |
| get_mcp_error_breakdown | 0.268 | 0.129 | 0.812 | 0.103 | 0.812 |
| get_capabilities | 0.906 | 0.780 | 1.434 | 0.733 | 1.434 |
| health_check | 0.227 | 0.197 | 0.363 | 0.134 | 0.363 |
| get_stats | 1.605 | 1.465 | 2.723 | 1.075 | 2.723 |
| list_pending | 0.343 | 0.186 | 1.005 | 0.118 | 1.005 |

## Category Breakdown

| Category | Tools | P95 (ms) |
|----------|-------|----------|
| Kernel read | 5 | 0.023 |
| Kernel write | 2 | 0.023 |
| Config | 3 | 0.238 |
| Telemetry | 2 | 2.937 |

## Aggregate

**P50:** 0.023 ms | **P95:** 1.465 ms | **Min:** 0.004 ms | **Max:** 2.937 ms

## P95 Gate

**Global P95:** 1.465 ms
**Threshold:** 50.0 ms
**Status:** PASS
