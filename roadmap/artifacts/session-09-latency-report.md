# Session 09 — Latency Report

**Generated:** 2026-02-27 00:43 UTC

**Tools benchmarked:** 16

## Per-tool Results

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| kernel::get_tweet | 0.013 | 0.012 | 0.017 | 0.011 | 0.017 |
| kernel::search_tweets | 0.008 | 0.008 | 0.011 | 0.007 | 0.011 |
| kernel::get_followers | 0.006 | 0.006 | 0.008 | 0.005 | 0.008 |
| kernel::get_user_by_id | 0.008 | 0.008 | 0.009 | 0.007 | 0.009 |
| kernel::get_me | 0.007 | 0.007 | 0.008 | 0.007 | 0.008 |
| kernel::post_tweet | 0.004 | 0.004 | 0.006 | 0.003 | 0.006 |
| kernel::reply_to_tweet | 0.004 | 0.004 | 0.004 | 0.003 | 0.004 |
| score_tweet | 0.014 | 0.013 | 0.020 | 0.012 | 0.020 |
| get_config | 0.083 | 0.083 | 0.089 | 0.079 | 0.089 |
| validate_config | 0.019 | 0.011 | 0.055 | 0.010 | 0.055 |
| get_mcp_tool_metrics | 1.028 | 0.611 | 2.782 | 0.514 | 2.782 |
| get_mcp_error_breakdown | 0.309 | 0.177 | 0.854 | 0.119 | 0.854 |
| get_capabilities | 1.001 | 0.990 | 1.417 | 0.562 | 1.417 |
| health_check | 0.310 | 0.238 | 0.604 | 0.218 | 0.604 |
| get_stats | 1.400 | 1.206 | 2.419 | 1.052 | 2.419 |
| list_pending | 0.458 | 0.138 | 1.755 | 0.110 | 1.755 |

## Category Breakdown

| Category | Tools | P95 (ms) |
|----------|-------|----------|
| Kernel read | 5 | 0.013 |
| Kernel write | 2 | 0.006 |
| Config | 3 | 0.089 |
| Telemetry | 2 | 2.782 |

## Aggregate

**P50:** 0.013 ms | **P95:** 1.268 ms | **Min:** 0.003 ms | **Max:** 2.782 ms

## P95 Gate

**Global P95:** 1.268 ms
**Threshold:** 50.0 ms
**Status:** PASS
