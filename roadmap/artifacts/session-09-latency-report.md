# Session 09 — Latency Report

**Generated:** 2026-02-26 19:47 UTC

**Tools benchmarked:** 16

## Per-tool Results

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| kernel::get_tweet | 0.016 | 0.013 | 0.028 | 0.012 | 0.028 |
| kernel::search_tweets | 0.010 | 0.009 | 0.013 | 0.008 | 0.013 |
| kernel::get_followers | 0.008 | 0.007 | 0.010 | 0.007 | 0.010 |
| kernel::get_user_by_id | 0.010 | 0.009 | 0.011 | 0.009 | 0.011 |
| kernel::get_me | 0.010 | 0.009 | 0.010 | 0.009 | 0.010 |
| kernel::post_tweet | 0.005 | 0.004 | 0.007 | 0.004 | 0.007 |
| kernel::reply_to_tweet | 0.004 | 0.004 | 0.004 | 0.004 | 0.004 |
| score_tweet | 0.024 | 0.014 | 0.062 | 0.014 | 0.062 |
| get_config | 0.094 | 0.092 | 0.103 | 0.089 | 0.103 |
| validate_config | 0.021 | 0.012 | 0.058 | 0.011 | 0.058 |
| get_mcp_tool_metrics | 0.236 | 0.203 | 0.395 | 0.182 | 0.395 |
| get_mcp_error_breakdown | 0.069 | 0.062 | 0.100 | 0.055 | 0.100 |
| get_capabilities | 0.399 | 0.391 | 0.445 | 0.373 | 0.445 |
| health_check | 0.066 | 0.062 | 0.090 | 0.054 | 0.090 |
| get_stats | 0.359 | 0.338 | 0.459 | 0.326 | 0.459 |
| list_pending | 0.079 | 0.055 | 0.188 | 0.045 | 0.188 |

## Category Breakdown

| Category | Tools | P95 (ms) |
|----------|-------|----------|
| Kernel read | 5 | 0.013 |
| Kernel write | 2 | 0.007 |
| Config | 3 | 0.103 |
| Telemetry | 2 | 0.395 |

## Aggregate

**P50:** 0.014 ms | **P95:** 0.391 ms | **Min:** 0.004 ms | **Max:** 0.459 ms

## P95 Gate

**Global P95:** 0.391 ms
**Threshold:** 50.0 ms
**Status:** PASS
