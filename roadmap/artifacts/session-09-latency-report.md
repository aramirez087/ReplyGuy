# Session 09 — Latency Report

**Generated:** 2026-03-01 00:07 UTC

**Tools benchmarked:** 16

## Per-tool Results

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| kernel::get_tweet | 0.014 | 0.012 | 0.024 | 0.011 | 0.024 |
| kernel::search_tweets | 0.008 | 0.007 | 0.013 | 0.007 | 0.013 |
| kernel::get_followers | 0.006 | 0.006 | 0.008 | 0.006 | 0.008 |
| kernel::get_user_by_id | 0.008 | 0.008 | 0.009 | 0.008 | 0.009 |
| kernel::get_me | 0.009 | 0.009 | 0.010 | 0.007 | 0.010 |
| kernel::post_tweet | 0.004 | 0.004 | 0.006 | 0.004 | 0.006 |
| kernel::reply_to_tweet | 0.004 | 0.004 | 0.004 | 0.003 | 0.004 |
| score_tweet | 0.016 | 0.014 | 0.024 | 0.013 | 0.024 |
| get_config | 0.091 | 0.090 | 0.100 | 0.087 | 0.100 |
| validate_config | 0.014 | 0.012 | 0.024 | 0.010 | 0.024 |
| get_mcp_tool_metrics | 0.970 | 0.528 | 2.782 | 0.495 | 2.782 |
| get_mcp_error_breakdown | 0.257 | 0.163 | 0.595 | 0.114 | 0.595 |
| get_capabilities | 0.814 | 0.770 | 1.106 | 0.572 | 1.106 |
| health_check | 0.201 | 0.183 | 0.306 | 0.134 | 0.306 |
| get_stats | 1.516 | 1.208 | 2.828 | 1.039 | 2.828 |
| list_pending | 0.406 | 0.213 | 1.201 | 0.148 | 1.201 |

## Category Breakdown

| Category | Tools | P95 (ms) |
|----------|-------|----------|
| Kernel read | 5 | 0.013 |
| Kernel write | 2 | 0.006 |
| Config | 3 | 0.100 |
| Telemetry | 2 | 2.782 |

## Aggregate

**P50:** 0.014 ms | **P95:** 1.201 ms | **Min:** 0.003 ms | **Max:** 2.828 ms

## P95 Gate

**Global P95:** 1.201 ms
**Threshold:** 50.0 ms
**Status:** PASS
