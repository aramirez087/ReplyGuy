# Session 09 — Latency Report

**Generated:** 2026-02-28 03:49 UTC

**Tools benchmarked:** 16

## Per-tool Results

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| kernel::get_tweet | 0.015 | 0.011 | 0.027 | 0.011 | 0.027 |
| kernel::search_tweets | 0.009 | 0.008 | 0.013 | 0.007 | 0.013 |
| kernel::get_followers | 0.006 | 0.006 | 0.008 | 0.006 | 0.008 |
| kernel::get_user_by_id | 0.008 | 0.007 | 0.009 | 0.007 | 0.009 |
| kernel::get_me | 0.008 | 0.008 | 0.008 | 0.007 | 0.008 |
| kernel::post_tweet | 0.004 | 0.004 | 0.006 | 0.003 | 0.006 |
| kernel::reply_to_tweet | 0.003 | 0.003 | 0.004 | 0.003 | 0.004 |
| score_tweet | 0.015 | 0.013 | 0.020 | 0.013 | 0.020 |
| get_config | 0.093 | 0.092 | 0.102 | 0.086 | 0.102 |
| validate_config | 0.022 | 0.011 | 0.065 | 0.010 | 0.065 |
| get_mcp_tool_metrics | 1.117 | 0.748 | 2.668 | 0.573 | 2.668 |
| get_mcp_error_breakdown | 0.212 | 0.141 | 0.451 | 0.121 | 0.451 |
| get_capabilities | 0.933 | 0.816 | 1.314 | 0.800 | 1.314 |
| health_check | 0.293 | 0.249 | 0.627 | 0.159 | 0.627 |
| get_stats | 2.189 | 1.774 | 3.434 | 1.297 | 3.434 |
| list_pending | 0.437 | 0.160 | 1.537 | 0.135 | 1.537 |

## Category Breakdown

| Category | Tools | P95 (ms) |
|----------|-------|----------|
| Kernel read | 5 | 0.013 |
| Kernel write | 2 | 0.006 |
| Config | 3 | 0.102 |
| Telemetry | 2 | 2.668 |

## Aggregate

**P50:** 0.013 ms | **P95:** 1.657 ms | **Min:** 0.003 ms | **Max:** 3.434 ms

## P95 Gate

**Global P95:** 1.657 ms
**Threshold:** 50.0 ms
**Status:** PASS
