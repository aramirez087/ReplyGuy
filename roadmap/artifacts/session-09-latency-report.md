# Session 09 — Latency Report

**Generated:** 2026-02-26 14:19 UTC

**Tools benchmarked:** 16

## Per-tool Results

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| kernel::get_tweet | 0.025 | 0.022 | 0.030 | 0.021 | 0.030 |
| kernel::search_tweets | 0.011 | 0.008 | 0.021 | 0.007 | 0.021 |
| kernel::get_followers | 0.006 | 0.006 | 0.008 | 0.006 | 0.008 |
| kernel::get_user_by_id | 0.007 | 0.007 | 0.009 | 0.007 | 0.009 |
| kernel::get_me | 0.008 | 0.008 | 0.008 | 0.007 | 0.008 |
| kernel::post_tweet | 0.004 | 0.004 | 0.006 | 0.003 | 0.006 |
| kernel::reply_to_tweet | 0.004 | 0.004 | 0.004 | 0.003 | 0.004 |
| score_tweet | 0.056 | 0.013 | 0.231 | 0.012 | 0.231 |
| get_config | 0.084 | 0.081 | 0.094 | 0.080 | 0.094 |
| validate_config | 0.062 | 0.010 | 0.268 | 0.010 | 0.268 |
| get_mcp_tool_metrics | 1.872 | 1.205 | 4.650 | 0.964 | 4.650 |
| get_mcp_error_breakdown | 0.514 | 0.344 | 1.359 | 0.204 | 1.359 |
| get_capabilities | 1.039 | 0.981 | 1.914 | 0.505 | 1.914 |
| health_check | 0.332 | 0.223 | 0.789 | 0.150 | 0.789 |
| get_stats | 2.292 | 2.121 | 3.698 | 1.651 | 3.698 |
| list_pending | 0.902 | 0.307 | 3.085 | 0.215 | 3.085 |

## Category Breakdown

| Category | Tools | P95 (ms) |
|----------|-------|----------|
| Kernel read | 5 | 0.030 |
| Kernel write | 2 | 0.006 |
| Config | 3 | 0.268 |
| Telemetry | 2 | 4.650 |

## Aggregate

**P50:** 0.022 ms | **P95:** 2.121 ms | **Min:** 0.003 ms | **Max:** 4.650 ms

## P95 Gate

**Global P95:** 2.121 ms
**Threshold:** 50.0 ms
**Status:** PASS
