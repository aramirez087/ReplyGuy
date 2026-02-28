# Session 09 — Latency Report

**Generated:** 2026-02-28 18:16 UTC

**Tools benchmarked:** 16

## Per-tool Results

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| kernel::get_tweet | 0.012 | 0.011 | 0.016 | 0.011 | 0.016 |
| kernel::search_tweets | 0.009 | 0.008 | 0.011 | 0.007 | 0.011 |
| kernel::get_followers | 0.006 | 0.006 | 0.008 | 0.005 | 0.008 |
| kernel::get_user_by_id | 0.008 | 0.008 | 0.009 | 0.007 | 0.009 |
| kernel::get_me | 0.007 | 0.007 | 0.008 | 0.007 | 0.008 |
| kernel::post_tweet | 0.004 | 0.004 | 0.006 | 0.003 | 0.006 |
| kernel::reply_to_tweet | 0.004 | 0.004 | 0.005 | 0.003 | 0.005 |
| score_tweet | 0.014 | 0.013 | 0.020 | 0.012 | 0.020 |
| get_config | 0.090 | 0.088 | 0.096 | 0.085 | 0.096 |
| validate_config | 0.018 | 0.010 | 0.048 | 0.009 | 0.048 |
| get_mcp_tool_metrics | 0.898 | 0.507 | 2.452 | 0.493 | 2.452 |
| get_mcp_error_breakdown | 0.186 | 0.160 | 0.394 | 0.067 | 0.394 |
| get_capabilities | 1.159 | 0.970 | 1.780 | 0.813 | 1.780 |
| health_check | 0.270 | 0.223 | 0.578 | 0.146 | 0.578 |
| get_stats | 1.855 | 1.114 | 3.721 | 1.099 | 3.721 |
| list_pending | 0.377 | 0.137 | 1.405 | 0.085 | 1.405 |

## Category Breakdown

| Category | Tools | P95 (ms) |
|----------|-------|----------|
| Kernel read | 5 | 0.012 |
| Kernel write | 2 | 0.006 |
| Config | 3 | 0.096 |
| Telemetry | 2 | 2.452 |

## Aggregate

**P50:** 0.013 ms | **P95:** 1.419 ms | **Min:** 0.003 ms | **Max:** 3.721 ms

## P95 Gate

**Global P95:** 1.419 ms
**Threshold:** 50.0 ms
**Status:** PASS
