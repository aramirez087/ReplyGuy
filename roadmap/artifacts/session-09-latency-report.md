# Session 09 — Latency Report

**Generated:** 2026-02-27 00:59 UTC

**Tools benchmarked:** 16

## Per-tool Results

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| kernel::get_tweet | 0.014 | 0.013 | 0.018 | 0.012 | 0.018 |
| kernel::search_tweets | 0.009 | 0.009 | 0.012 | 0.008 | 0.012 |
| kernel::get_followers | 0.007 | 0.006 | 0.009 | 0.005 | 0.009 |
| kernel::get_user_by_id | 0.008 | 0.007 | 0.009 | 0.007 | 0.009 |
| kernel::get_me | 0.007 | 0.007 | 0.008 | 0.007 | 0.008 |
| kernel::post_tweet | 0.004 | 0.003 | 0.006 | 0.003 | 0.006 |
| kernel::reply_to_tweet | 0.003 | 0.003 | 0.004 | 0.003 | 0.004 |
| score_tweet | 0.014 | 0.012 | 0.021 | 0.012 | 0.021 |
| get_config | 0.085 | 0.084 | 0.090 | 0.081 | 0.090 |
| validate_config | 0.019 | 0.010 | 0.054 | 0.010 | 0.054 |
| get_mcp_tool_metrics | 0.890 | 0.597 | 2.056 | 0.472 | 2.056 |
| get_mcp_error_breakdown | 0.245 | 0.161 | 0.585 | 0.134 | 0.585 |
| get_capabilities | 1.004 | 1.049 | 1.417 | 0.670 | 1.417 |
| health_check | 0.384 | 0.394 | 0.709 | 0.089 | 0.709 |
| get_stats | 1.694 | 1.433 | 3.383 | 1.071 | 3.383 |
| list_pending | 0.405 | 0.170 | 1.359 | 0.135 | 1.359 |

## Category Breakdown

| Category | Tools | P95 (ms) |
|----------|-------|----------|
| Kernel read | 5 | 0.013 |
| Kernel write | 2 | 0.006 |
| Config | 3 | 0.090 |
| Telemetry | 2 | 2.056 |

## Aggregate

**P50:** 0.013 ms | **P95:** 1.417 ms | **Min:** 0.003 ms | **Max:** 3.383 ms

## P95 Gate

**Global P95:** 1.417 ms
**Threshold:** 50.0 ms
**Status:** PASS
