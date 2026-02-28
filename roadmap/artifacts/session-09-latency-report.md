# Session 09 — Latency Report

**Generated:** 2026-02-28 04:13 UTC

**Tools benchmarked:** 16

## Per-tool Results

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| kernel::get_tweet | 0.014 | 0.012 | 0.019 | 0.012 | 0.019 |
| kernel::search_tweets | 0.009 | 0.008 | 0.013 | 0.007 | 0.013 |
| kernel::get_followers | 0.006 | 0.006 | 0.009 | 0.006 | 0.009 |
| kernel::get_user_by_id | 0.008 | 0.007 | 0.010 | 0.007 | 0.010 |
| kernel::get_me | 0.007 | 0.007 | 0.008 | 0.007 | 0.008 |
| kernel::post_tweet | 0.004 | 0.004 | 0.006 | 0.004 | 0.006 |
| kernel::reply_to_tweet | 0.004 | 0.003 | 0.004 | 0.003 | 0.004 |
| score_tweet | 0.021 | 0.013 | 0.055 | 0.012 | 0.055 |
| get_config | 0.113 | 0.091 | 0.186 | 0.090 | 0.186 |
| validate_config | 0.081 | 0.011 | 0.361 | 0.011 | 0.361 |
| get_mcp_tool_metrics | 1.090 | 0.895 | 2.054 | 0.543 | 2.054 |
| get_mcp_error_breakdown | 0.292 | 0.186 | 0.809 | 0.114 | 0.809 |
| get_capabilities | 0.837 | 0.729 | 1.380 | 0.600 | 1.380 |
| health_check | 0.196 | 0.171 | 0.331 | 0.136 | 0.331 |
| get_stats | 1.646 | 1.539 | 2.571 | 1.055 | 2.571 |
| list_pending | 0.368 | 0.136 | 1.216 | 0.113 | 1.216 |

## Category Breakdown

| Category | Tools | P95 (ms) |
|----------|-------|----------|
| Kernel read | 5 | 0.013 |
| Kernel write | 2 | 0.006 |
| Config | 3 | 0.361 |
| Telemetry | 2 | 2.054 |

## Aggregate

**P50:** 0.013 ms | **P95:** 1.380 ms | **Min:** 0.003 ms | **Max:** 2.571 ms

## P95 Gate

**Global P95:** 1.380 ms
**Threshold:** 50.0 ms
**Status:** PASS
