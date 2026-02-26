# Session 09 — Latency Report

**Generated:** 2026-02-26 15:58 UTC

**Tools benchmarked:** 16

## Per-tool Results

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| kernel::get_tweet | 0.015 | 0.011 | 0.030 | 0.011 | 0.030 |
| kernel::search_tweets | 0.008 | 0.007 | 0.012 | 0.007 | 0.012 |
| kernel::get_followers | 0.006 | 0.006 | 0.009 | 0.006 | 0.009 |
| kernel::get_user_by_id | 0.008 | 0.007 | 0.009 | 0.007 | 0.009 |
| kernel::get_me | 0.008 | 0.007 | 0.008 | 0.007 | 0.008 |
| kernel::post_tweet | 0.004 | 0.003 | 0.006 | 0.003 | 0.006 |
| kernel::reply_to_tweet | 0.004 | 0.003 | 0.004 | 0.003 | 0.004 |
| score_tweet | 0.016 | 0.013 | 0.032 | 0.012 | 0.032 |
| get_config | 0.082 | 0.079 | 0.094 | 0.079 | 0.094 |
| validate_config | 0.013 | 0.011 | 0.021 | 0.010 | 0.021 |
| get_mcp_tool_metrics | 1.215 | 0.833 | 2.979 | 0.638 | 2.979 |
| get_mcp_error_breakdown | 0.317 | 0.206 | 0.768 | 0.153 | 0.768 |
| get_capabilities | 0.926 | 0.903 | 1.124 | 0.734 | 1.124 |
| health_check | 0.598 | 0.490 | 1.125 | 0.242 | 1.125 |
| get_stats | 1.660 | 1.323 | 3.233 | 1.126 | 3.233 |
| list_pending | 0.398 | 0.162 | 1.455 | 0.092 | 1.455 |

## Category Breakdown

| Category | Tools | P95 (ms) |
|----------|-------|----------|
| Kernel read | 5 | 0.012 |
| Kernel write | 2 | 0.006 |
| Config | 3 | 0.094 |
| Telemetry | 2 | 2.979 |

## Aggregate

**P50:** 0.013 ms | **P95:** 1.323 ms | **Min:** 0.003 ms | **Max:** 3.233 ms

## P95 Gate

**Global P95:** 1.323 ms
**Threshold:** 50.0 ms
**Status:** PASS
