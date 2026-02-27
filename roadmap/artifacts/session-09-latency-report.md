# Session 09 — Latency Report

**Generated:** 2026-02-27 00:16 UTC

**Tools benchmarked:** 16

## Per-tool Results

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| kernel::get_tweet | 0.020 | 0.012 | 0.051 | 0.011 | 0.051 |
| kernel::search_tweets | 0.009 | 0.008 | 0.011 | 0.008 | 0.011 |
| kernel::get_followers | 0.006 | 0.006 | 0.009 | 0.005 | 0.009 |
| kernel::get_user_by_id | 0.008 | 0.007 | 0.008 | 0.007 | 0.008 |
| kernel::get_me | 0.007 | 0.007 | 0.008 | 0.007 | 0.008 |
| kernel::post_tweet | 0.004 | 0.003 | 0.006 | 0.003 | 0.006 |
| kernel::reply_to_tweet | 0.004 | 0.004 | 0.004 | 0.003 | 0.004 |
| score_tweet | 0.014 | 0.013 | 0.019 | 0.012 | 0.019 |
| get_config | 0.083 | 0.081 | 0.089 | 0.080 | 0.089 |
| validate_config | 0.019 | 0.010 | 0.053 | 0.010 | 0.053 |
| get_mcp_tool_metrics | 1.248 | 1.019 | 2.482 | 0.826 | 2.482 |
| get_mcp_error_breakdown | 0.323 | 0.198 | 0.906 | 0.109 | 0.906 |
| get_capabilities | 0.794 | 0.637 | 1.320 | 0.557 | 1.320 |
| health_check | 0.267 | 0.231 | 0.414 | 0.165 | 0.414 |
| get_stats | 1.457 | 1.118 | 2.838 | 1.070 | 2.838 |
| list_pending | 0.355 | 0.120 | 1.313 | 0.064 | 1.313 |

## Category Breakdown

| Category | Tools | P95 (ms) |
|----------|-------|----------|
| Kernel read | 5 | 0.014 |
| Kernel write | 2 | 0.006 |
| Config | 3 | 0.089 |
| Telemetry | 2 | 2.482 |

## Aggregate

**P50:** 0.013 ms | **P95:** 1.172 ms | **Min:** 0.003 ms | **Max:** 2.838 ms

## P95 Gate

**Global P95:** 1.172 ms
**Threshold:** 50.0 ms
**Status:** PASS
