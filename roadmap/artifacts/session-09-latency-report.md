# Session 09 — Latency Report

**Generated:** 2026-02-26 16:41 UTC

**Tools benchmarked:** 16

## Per-tool Results

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| kernel::get_tweet | 0.013 | 0.012 | 0.020 | 0.011 | 0.020 |
| kernel::search_tweets | 0.009 | 0.008 | 0.013 | 0.007 | 0.013 |
| kernel::get_followers | 0.006 | 0.006 | 0.008 | 0.006 | 0.008 |
| kernel::get_user_by_id | 0.008 | 0.008 | 0.009 | 0.007 | 0.009 |
| kernel::get_me | 0.008 | 0.008 | 0.009 | 0.007 | 0.009 |
| kernel::post_tweet | 0.004 | 0.004 | 0.006 | 0.003 | 0.006 |
| kernel::reply_to_tweet | 0.004 | 0.004 | 0.004 | 0.003 | 0.004 |
| score_tweet | 0.059 | 0.014 | 0.242 | 0.012 | 0.242 |
| get_config | 0.088 | 0.087 | 0.095 | 0.083 | 0.095 |
| validate_config | 0.019 | 0.011 | 0.048 | 0.010 | 0.048 |
| get_mcp_tool_metrics | 1.424 | 0.938 | 3.572 | 0.732 | 3.572 |
| get_mcp_error_breakdown | 0.348 | 0.228 | 0.791 | 0.206 | 0.791 |
| get_capabilities | 0.903 | 0.865 | 1.437 | 0.508 | 1.437 |
| health_check | 0.363 | 0.338 | 0.595 | 0.224 | 0.595 |
| get_stats | 2.088 | 1.896 | 3.338 | 1.461 | 3.338 |
| list_pending | 0.415 | 0.192 | 1.389 | 0.129 | 1.389 |

## Category Breakdown

| Category | Tools | P95 (ms) |
|----------|-------|----------|
| Kernel read | 5 | 0.013 |
| Kernel write | 2 | 0.006 |
| Config | 3 | 0.242 |
| Telemetry | 2 | 3.572 |

## Aggregate

**P50:** 0.014 ms | **P95:** 1.821 ms | **Min:** 0.003 ms | **Max:** 3.572 ms

## P95 Gate

**Global P95:** 1.821 ms
**Threshold:** 50.0 ms
**Status:** PASS
