# Session 09 — Latency Report

**Generated:** 2026-02-26 06:00 UTC

**Tools benchmarked:** 16

## Per-tool Results

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| kernel::get_tweet | 0.024 | 0.021 | 0.037 | 0.021 | 0.037 |
| kernel::search_tweets | 0.018 | 0.014 | 0.039 | 0.008 | 0.039 |
| kernel::get_followers | 0.006 | 0.006 | 0.008 | 0.005 | 0.008 |
| kernel::get_user_by_id | 0.008 | 0.007 | 0.009 | 0.007 | 0.009 |
| kernel::get_me | 0.007 | 0.007 | 0.007 | 0.007 | 0.007 |
| kernel::post_tweet | 0.004 | 0.003 | 0.006 | 0.003 | 0.006 |
| kernel::reply_to_tweet | 0.004 | 0.004 | 0.004 | 0.003 | 0.004 |
| score_tweet | 0.021 | 0.012 | 0.057 | 0.012 | 0.057 |
| get_config | 0.083 | 0.080 | 0.093 | 0.079 | 0.093 |
| validate_config | 0.019 | 0.010 | 0.053 | 0.009 | 0.053 |
| get_mcp_tool_metrics | 2.037 | 1.068 | 5.732 | 0.762 | 5.732 |
| get_mcp_error_breakdown | 0.337 | 0.210 | 0.739 | 0.116 | 0.739 |
| get_capabilities | 1.579 | 1.638 | 2.265 | 1.094 | 2.265 |
| health_check | 0.976 | 0.438 | 3.114 | 0.320 | 3.114 |
| get_stats | 6.593 | 4.635 | 11.236 | 2.086 | 11.236 |
| list_pending | 0.899 | 0.390 | 3.031 | 0.334 | 3.031 |

## Category Breakdown

| Category | Tools | P95 (ms) |
|----------|-------|----------|
| Kernel read | 5 | 0.037 |
| Kernel write | 2 | 0.006 |
| Config | 3 | 0.093 |
| Telemetry | 2 | 5.732 |

## Aggregate

**P50:** 0.022 ms | **P95:** 4.605 ms | **Min:** 0.003 ms | **Max:** 11.236 ms

## P95 Gate

**Global P95:** 4.605 ms
**Threshold:** 50.0 ms
**Status:** PASS
