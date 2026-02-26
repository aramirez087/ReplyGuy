# Session 09 — Latency Report

**Generated:** 2026-02-26 06:38 UTC

**Tools benchmarked:** 16

## Per-tool Results

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| kernel::get_tweet | 0.013 | 0.011 | 0.018 | 0.011 | 0.018 |
| kernel::search_tweets | 0.008 | 0.007 | 0.011 | 0.007 | 0.011 |
| kernel::get_followers | 0.006 | 0.005 | 0.008 | 0.005 | 0.008 |
| kernel::get_user_by_id | 0.007 | 0.007 | 0.008 | 0.007 | 0.008 |
| kernel::get_me | 0.007 | 0.007 | 0.008 | 0.007 | 0.008 |
| kernel::post_tweet | 0.004 | 0.003 | 0.006 | 0.003 | 0.006 |
| kernel::reply_to_tweet | 0.004 | 0.003 | 0.004 | 0.003 | 0.004 |
| score_tweet | 0.014 | 0.012 | 0.024 | 0.011 | 0.024 |
| get_config | 0.082 | 0.080 | 0.092 | 0.078 | 0.092 |
| validate_config | 0.018 | 0.010 | 0.052 | 0.009 | 0.052 |
| get_mcp_tool_metrics | 1.337 | 1.280 | 2.613 | 0.644 | 2.613 |
| get_mcp_error_breakdown | 0.360 | 0.297 | 0.806 | 0.157 | 0.806 |
| get_capabilities | 0.745 | 0.729 | 0.985 | 0.562 | 0.985 |
| health_check | 0.316 | 0.373 | 0.498 | 0.125 | 0.498 |
| get_stats | 1.844 | 1.799 | 2.826 | 1.360 | 2.826 |
| list_pending | 0.566 | 0.274 | 1.856 | 0.154 | 1.856 |

## Category Breakdown

| Category | Tools | P95 (ms) |
|----------|-------|----------|
| Kernel read | 5 | 0.012 |
| Kernel write | 2 | 0.006 |
| Config | 3 | 0.092 |
| Telemetry | 2 | 2.613 |

## Aggregate

**P50:** 0.012 ms | **P95:** 1.799 ms | **Min:** 0.003 ms | **Max:** 2.826 ms

## P95 Gate

**Global P95:** 1.799 ms
**Threshold:** 50.0 ms
**Status:** PASS
