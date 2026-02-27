# Session 09 — Latency Report

**Generated:** 2026-02-27 02:51 UTC

**Tools benchmarked:** 16

## Per-tool Results

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| kernel::get_tweet | 0.015 | 0.012 | 0.027 | 0.011 | 0.027 |
| kernel::search_tweets | 0.008 | 0.007 | 0.011 | 0.007 | 0.011 |
| kernel::get_followers | 0.006 | 0.006 | 0.008 | 0.006 | 0.008 |
| kernel::get_user_by_id | 0.008 | 0.007 | 0.009 | 0.007 | 0.009 |
| kernel::get_me | 0.008 | 0.008 | 0.008 | 0.007 | 0.008 |
| kernel::post_tweet | 0.004 | 0.004 | 0.006 | 0.003 | 0.006 |
| kernel::reply_to_tweet | 0.004 | 0.003 | 0.004 | 0.003 | 0.004 |
| score_tweet | 0.018 | 0.012 | 0.042 | 0.012 | 0.042 |
| get_config | 0.085 | 0.083 | 0.096 | 0.081 | 0.096 |
| validate_config | 0.056 | 0.010 | 0.237 | 0.010 | 0.237 |
| get_mcp_tool_metrics | 0.988 | 0.602 | 2.510 | 0.592 | 2.510 |
| get_mcp_error_breakdown | 0.210 | 0.168 | 0.502 | 0.091 | 0.502 |
| get_capabilities | 0.800 | 0.703 | 1.281 | 0.608 | 1.281 |
| health_check | 0.285 | 0.197 | 0.638 | 0.161 | 0.638 |
| get_stats | 1.587 | 1.320 | 2.513 | 1.294 | 2.513 |
| list_pending | 0.389 | 0.172 | 1.314 | 0.086 | 1.314 |

## Category Breakdown

| Category | Tools | P95 (ms) |
|----------|-------|----------|
| Kernel read | 5 | 0.013 |
| Kernel write | 2 | 0.006 |
| Config | 3 | 0.237 |
| Telemetry | 2 | 2.510 |

## Aggregate

**P50:** 0.013 ms | **P95:** 1.319 ms | **Min:** 0.003 ms | **Max:** 2.513 ms

## P95 Gate

**Global P95:** 1.319 ms
**Threshold:** 50.0 ms
**Status:** PASS
