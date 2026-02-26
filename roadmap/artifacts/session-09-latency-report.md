# Session 09 — Latency Report

**Generated:** 2026-02-26 16:35 UTC

**Tools benchmarked:** 16

## Per-tool Results

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| kernel::get_tweet | 0.026 | 0.021 | 0.048 | 0.021 | 0.048 |
| kernel::search_tweets | 0.012 | 0.009 | 0.022 | 0.008 | 0.022 |
| kernel::get_followers | 0.006 | 0.006 | 0.008 | 0.006 | 0.008 |
| kernel::get_user_by_id | 0.008 | 0.008 | 0.010 | 0.007 | 0.010 |
| kernel::get_me | 0.008 | 0.008 | 0.008 | 0.007 | 0.008 |
| kernel::post_tweet | 0.004 | 0.004 | 0.006 | 0.003 | 0.006 |
| kernel::reply_to_tweet | 0.004 | 0.004 | 0.005 | 0.003 | 0.005 |
| score_tweet | 0.054 | 0.014 | 0.217 | 0.012 | 0.217 |
| get_config | 0.087 | 0.085 | 0.094 | 0.084 | 0.094 |
| validate_config | 0.055 | 0.010 | 0.233 | 0.009 | 0.233 |
| get_mcp_tool_metrics | 1.183 | 0.833 | 2.645 | 0.754 | 2.645 |
| get_mcp_error_breakdown | 0.285 | 0.194 | 0.653 | 0.176 | 0.653 |
| get_capabilities | 0.755 | 0.764 | 1.060 | 0.576 | 1.060 |
| health_check | 0.353 | 0.218 | 0.792 | 0.139 | 0.792 |
| get_stats | 1.942 | 1.737 | 3.091 | 1.421 | 3.091 |
| list_pending | 0.423 | 0.213 | 1.263 | 0.190 | 1.263 |

## Category Breakdown

| Category | Tools | P95 (ms) |
|----------|-------|----------|
| Kernel read | 5 | 0.022 |
| Kernel write | 2 | 0.006 |
| Config | 3 | 0.233 |
| Telemetry | 2 | 2.645 |

## Aggregate

**P50:** 0.022 ms | **P95:** 1.613 ms | **Min:** 0.003 ms | **Max:** 3.091 ms

## P95 Gate

**Global P95:** 1.613 ms
**Threshold:** 50.0 ms
**Status:** PASS
