# Session 09 — Latency Report

**Generated:** 2026-02-27 18:06 UTC

**Tools benchmarked:** 16

## Per-tool Results

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| kernel::get_tweet | 0.012 | 0.012 | 0.016 | 0.011 | 0.016 |
| kernel::search_tweets | 0.008 | 0.007 | 0.012 | 0.007 | 0.012 |
| kernel::get_followers | 0.006 | 0.006 | 0.008 | 0.006 | 0.008 |
| kernel::get_user_by_id | 0.008 | 0.008 | 0.009 | 0.008 | 0.009 |
| kernel::get_me | 0.008 | 0.008 | 0.008 | 0.008 | 0.008 |
| kernel::post_tweet | 0.004 | 0.004 | 0.006 | 0.004 | 0.006 |
| kernel::reply_to_tweet | 0.005 | 0.004 | 0.006 | 0.004 | 0.006 |
| score_tweet | 0.015 | 0.013 | 0.024 | 0.012 | 0.024 |
| get_config | 0.085 | 0.083 | 0.090 | 0.082 | 0.090 |
| validate_config | 0.019 | 0.010 | 0.056 | 0.010 | 0.056 |
| get_mcp_tool_metrics | 0.963 | 0.651 | 2.297 | 0.515 | 2.297 |
| get_mcp_error_breakdown | 0.263 | 0.218 | 0.524 | 0.141 | 0.524 |
| get_capabilities | 0.935 | 0.672 | 1.872 | 0.609 | 1.872 |
| health_check | 0.463 | 0.317 | 0.897 | 0.165 | 0.897 |
| get_stats | 1.660 | 1.493 | 2.879 | 1.102 | 2.879 |
| list_pending | 0.286 | 0.139 | 0.899 | 0.099 | 0.899 |

## Category Breakdown

| Category | Tools | P95 (ms) |
|----------|-------|----------|
| Kernel read | 5 | 0.012 |
| Kernel write | 2 | 0.006 |
| Config | 3 | 0.090 |
| Telemetry | 2 | 2.297 |

## Aggregate

**P50:** 0.013 ms | **P95:** 1.493 ms | **Min:** 0.004 ms | **Max:** 2.879 ms

## P95 Gate

**Global P95:** 1.493 ms
**Threshold:** 50.0 ms
**Status:** PASS
