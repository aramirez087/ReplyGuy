# Session 09 — Latency Report

**Generated:** 2026-02-28 02:16 UTC

**Tools benchmarked:** 16

## Per-tool Results

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| kernel::get_tweet | 0.014 | 0.011 | 0.026 | 0.011 | 0.026 |
| kernel::search_tweets | 0.009 | 0.008 | 0.011 | 0.007 | 0.011 |
| kernel::get_followers | 0.006 | 0.006 | 0.008 | 0.005 | 0.008 |
| kernel::get_user_by_id | 0.008 | 0.008 | 0.009 | 0.007 | 0.009 |
| kernel::get_me | 0.008 | 0.007 | 0.008 | 0.007 | 0.008 |
| kernel::post_tweet | 0.004 | 0.003 | 0.006 | 0.003 | 0.006 |
| kernel::reply_to_tweet | 0.003 | 0.003 | 0.004 | 0.003 | 0.004 |
| score_tweet | 0.014 | 0.012 | 0.019 | 0.012 | 0.019 |
| get_config | 0.087 | 0.086 | 0.094 | 0.083 | 0.094 |
| validate_config | 0.019 | 0.010 | 0.052 | 0.010 | 0.052 |
| get_mcp_tool_metrics | 1.124 | 0.604 | 2.809 | 0.459 | 2.809 |
| get_mcp_error_breakdown | 0.275 | 0.172 | 0.679 | 0.148 | 0.679 |
| get_capabilities | 1.100 | 1.045 | 1.428 | 0.915 | 1.428 |
| health_check | 0.383 | 0.248 | 0.811 | 0.196 | 0.811 |
| get_stats | 1.496 | 1.219 | 2.733 | 1.085 | 2.733 |
| list_pending | 0.332 | 0.087 | 1.248 | 0.076 | 1.248 |

## Category Breakdown

| Category | Tools | P95 (ms) |
|----------|-------|----------|
| Kernel read | 5 | 0.012 |
| Kernel write | 2 | 0.006 |
| Config | 3 | 0.094 |
| Telemetry | 2 | 2.809 |

## Aggregate

**P50:** 0.012 ms | **P95:** 1.284 ms | **Min:** 0.003 ms | **Max:** 2.809 ms

## P95 Gate

**Global P95:** 1.284 ms
**Threshold:** 50.0 ms
**Status:** PASS
