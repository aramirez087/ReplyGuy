# Session 09 — Latency Report

**Generated:** 2026-02-26 06:29 UTC

**Tools benchmarked:** 16

## Per-tool Results

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| kernel::get_tweet | 0.014 | 0.012 | 0.025 | 0.012 | 0.025 |
| kernel::search_tweets | 0.008 | 0.008 | 0.012 | 0.007 | 0.012 |
| kernel::get_followers | 0.006 | 0.006 | 0.008 | 0.005 | 0.008 |
| kernel::get_user_by_id | 0.007 | 0.007 | 0.009 | 0.007 | 0.009 |
| kernel::get_me | 0.007 | 0.007 | 0.008 | 0.007 | 0.008 |
| kernel::post_tweet | 0.004 | 0.004 | 0.005 | 0.003 | 0.005 |
| kernel::reply_to_tweet | 0.004 | 0.003 | 0.004 | 0.003 | 0.004 |
| score_tweet | 0.015 | 0.013 | 0.025 | 0.012 | 0.025 |
| get_config | 0.083 | 0.081 | 0.094 | 0.078 | 0.094 |
| validate_config | 0.012 | 0.010 | 0.018 | 0.009 | 0.018 |
| get_mcp_tool_metrics | 1.240 | 0.829 | 3.144 | 0.644 | 3.144 |
| get_mcp_error_breakdown | 0.395 | 0.335 | 0.851 | 0.167 | 0.851 |
| get_capabilities | 0.855 | 0.819 | 1.251 | 0.659 | 1.251 |
| health_check | 0.316 | 0.142 | 0.989 | 0.106 | 0.989 |
| get_stats | 1.884 | 1.497 | 3.249 | 1.416 | 3.249 |
| list_pending | 0.396 | 0.151 | 1.388 | 0.131 | 1.388 |

## Category Breakdown

| Category | Tools | P95 (ms) |
|----------|-------|----------|
| Kernel read | 5 | 0.012 |
| Kernel write | 2 | 0.005 |
| Config | 3 | 0.094 |
| Telemetry | 2 | 3.144 |

## Aggregate

**P50:** 0.013 ms | **P95:** 1.462 ms | **Min:** 0.003 ms | **Max:** 3.249 ms

## P95 Gate

**Global P95:** 1.462 ms
**Threshold:** 50.0 ms
**Status:** PASS
