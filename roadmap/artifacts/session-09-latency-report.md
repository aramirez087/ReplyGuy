# Session 09 — Latency Report

**Generated:** 2026-03-01 01:08 UTC

**Tools benchmarked:** 16

## Per-tool Results

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| kernel::get_tweet | 0.015 | 0.012 | 0.029 | 0.011 | 0.029 |
| kernel::search_tweets | 0.009 | 0.008 | 0.013 | 0.007 | 0.013 |
| kernel::get_followers | 0.006 | 0.006 | 0.008 | 0.006 | 0.008 |
| kernel::get_user_by_id | 0.008 | 0.007 | 0.009 | 0.007 | 0.009 |
| kernel::get_me | 0.007 | 0.007 | 0.008 | 0.007 | 0.008 |
| kernel::post_tweet | 0.004 | 0.003 | 0.007 | 0.003 | 0.007 |
| kernel::reply_to_tweet | 0.004 | 0.003 | 0.004 | 0.003 | 0.004 |
| score_tweet | 0.020 | 0.013 | 0.049 | 0.012 | 0.049 |
| get_config | 0.089 | 0.087 | 0.103 | 0.083 | 0.103 |
| validate_config | 0.019 | 0.010 | 0.056 | 0.010 | 0.056 |
| get_mcp_tool_metrics | 1.254 | 0.870 | 2.886 | 0.776 | 2.886 |
| get_mcp_error_breakdown | 0.379 | 0.203 | 1.130 | 0.153 | 1.130 |
| get_capabilities | 0.949 | 0.918 | 1.405 | 0.723 | 1.405 |
| health_check | 0.445 | 0.295 | 0.779 | 0.228 | 0.779 |
| get_stats | 2.198 | 1.933 | 3.168 | 1.224 | 3.168 |
| list_pending | 0.519 | 0.244 | 1.725 | 0.157 | 1.725 |

## Category Breakdown

| Category | Tools | P95 (ms) |
|----------|-------|----------|
| Kernel read | 5 | 0.013 |
| Kernel write | 2 | 0.007 |
| Config | 3 | 0.103 |
| Telemetry | 2 | 2.886 |

## Aggregate

**P50:** 0.013 ms | **P95:** 1.725 ms | **Min:** 0.003 ms | **Max:** 3.168 ms

## P95 Gate

**Global P95:** 1.725 ms
**Threshold:** 50.0 ms
**Status:** PASS
