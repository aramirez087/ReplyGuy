# Session 09 — Latency Report

**Generated:** 2026-02-27 03:02 UTC

**Tools benchmarked:** 16

## Per-tool Results

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| kernel::get_tweet | 0.014 | 0.011 | 0.025 | 0.011 | 0.025 |
| kernel::search_tweets | 0.008 | 0.007 | 0.011 | 0.007 | 0.011 |
| kernel::get_followers | 0.006 | 0.006 | 0.008 | 0.005 | 0.008 |
| kernel::get_user_by_id | 0.007 | 0.007 | 0.008 | 0.007 | 0.008 |
| kernel::get_me | 0.007 | 0.007 | 0.008 | 0.007 | 0.008 |
| kernel::post_tweet | 0.004 | 0.003 | 0.005 | 0.003 | 0.005 |
| kernel::reply_to_tweet | 0.003 | 0.003 | 0.004 | 0.003 | 0.004 |
| score_tweet | 0.016 | 0.012 | 0.031 | 0.012 | 0.031 |
| get_config | 0.085 | 0.084 | 0.093 | 0.081 | 0.093 |
| validate_config | 0.013 | 0.010 | 0.024 | 0.010 | 0.024 |
| get_mcp_tool_metrics | 1.159 | 0.622 | 3.357 | 0.485 | 3.357 |
| get_mcp_error_breakdown | 0.220 | 0.147 | 0.590 | 0.089 | 0.590 |
| get_capabilities | 0.989 | 0.747 | 1.524 | 0.601 | 1.524 |
| health_check | 0.310 | 0.251 | 0.694 | 0.130 | 0.694 |
| get_stats | 1.642 | 1.331 | 3.274 | 1.062 | 3.274 |
| list_pending | 0.459 | 0.131 | 1.826 | 0.074 | 1.826 |

## Category Breakdown

| Category | Tools | P95 (ms) |
|----------|-------|----------|
| Kernel read | 5 | 0.012 |
| Kernel write | 2 | 0.005 |
| Config | 3 | 0.093 |
| Telemetry | 2 | 3.357 |

## Aggregate

**P50:** 0.012 ms | **P95:** 1.469 ms | **Min:** 0.003 ms | **Max:** 3.357 ms

## P95 Gate

**Global P95:** 1.469 ms
**Threshold:** 50.0 ms
**Status:** PASS
