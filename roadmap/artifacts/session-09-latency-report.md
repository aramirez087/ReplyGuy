# Session 09 — Latency Report

**Generated:** 2026-02-26 18:39 UTC

**Tools benchmarked:** 16

## Per-tool Results

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| kernel::get_tweet | 0.015 | 0.012 | 0.026 | 0.012 | 0.026 |
| kernel::search_tweets | 0.008 | 0.008 | 0.010 | 0.007 | 0.010 |
| kernel::get_followers | 0.006 | 0.006 | 0.008 | 0.005 | 0.008 |
| kernel::get_user_by_id | 0.007 | 0.007 | 0.008 | 0.007 | 0.008 |
| kernel::get_me | 0.008 | 0.007 | 0.009 | 0.007 | 0.009 |
| kernel::post_tweet | 0.004 | 0.004 | 0.005 | 0.003 | 0.005 |
| kernel::reply_to_tweet | 0.003 | 0.003 | 0.004 | 0.003 | 0.004 |
| score_tweet | 0.014 | 0.013 | 0.020 | 0.012 | 0.020 |
| get_config | 0.086 | 0.084 | 0.096 | 0.079 | 0.096 |
| validate_config | 0.018 | 0.011 | 0.043 | 0.011 | 0.043 |
| get_mcp_tool_metrics | 1.031 | 0.591 | 3.021 | 0.453 | 3.021 |
| get_mcp_error_breakdown | 0.215 | 0.139 | 0.593 | 0.088 | 0.593 |
| get_capabilities | 0.882 | 0.805 | 1.119 | 0.704 | 1.119 |
| health_check | 0.231 | 0.155 | 0.450 | 0.114 | 0.450 |
| get_stats | 1.607 | 1.304 | 3.007 | 1.139 | 3.007 |
| list_pending | 0.395 | 0.143 | 1.464 | 0.095 | 1.464 |

## Category Breakdown

| Category | Tools | P95 (ms) |
|----------|-------|----------|
| Kernel read | 5 | 0.013 |
| Kernel write | 2 | 0.005 |
| Config | 3 | 0.096 |
| Telemetry | 2 | 3.021 |

## Aggregate

**P50:** 0.013 ms | **P95:** 1.304 ms | **Min:** 0.003 ms | **Max:** 3.021 ms

## P95 Gate

**Global P95:** 1.304 ms
**Threshold:** 50.0 ms
**Status:** PASS
