# Session 09 — Latency Report

**Generated:** 2026-02-26 23:18 UTC

**Tools benchmarked:** 16

## Per-tool Results

| Tool | Avg (ms) | P50 (ms) | P95 (ms) | Min (ms) | Max (ms) |
|------|----------|----------|----------|----------|----------|
| kernel::get_tweet | 0.024 | 0.013 | 0.065 | 0.012 | 0.065 |
| kernel::search_tweets | 0.009 | 0.009 | 0.011 | 0.008 | 0.011 |
| kernel::get_followers | 0.006 | 0.006 | 0.008 | 0.005 | 0.008 |
| kernel::get_user_by_id | 0.008 | 0.008 | 0.008 | 0.007 | 0.008 |
| kernel::get_me | 0.007 | 0.007 | 0.008 | 0.007 | 0.008 |
| kernel::post_tweet | 0.004 | 0.003 | 0.006 | 0.003 | 0.006 |
| kernel::reply_to_tweet | 0.004 | 0.004 | 0.004 | 0.003 | 0.004 |
| score_tweet | 0.015 | 0.014 | 0.021 | 0.013 | 0.021 |
| get_config | 0.086 | 0.085 | 0.095 | 0.080 | 0.095 |
| validate_config | 0.017 | 0.010 | 0.046 | 0.010 | 0.046 |
| get_mcp_tool_metrics | 0.880 | 0.560 | 2.372 | 0.368 | 2.372 |
| get_mcp_error_breakdown | 0.286 | 0.165 | 0.820 | 0.104 | 0.820 |
| get_capabilities | 0.863 | 0.699 | 1.445 | 0.654 | 1.445 |
| health_check | 0.310 | 0.182 | 0.834 | 0.149 | 0.834 |
| get_stats | 1.655 | 1.380 | 2.822 | 1.305 | 2.822 |
| list_pending | 0.399 | 0.150 | 1.488 | 0.099 | 1.488 |

## Category Breakdown

| Category | Tools | P95 (ms) |
|----------|-------|----------|
| Kernel read | 5 | 0.017 |
| Kernel write | 2 | 0.006 |
| Config | 3 | 0.095 |
| Telemetry | 2 | 2.372 |

## Aggregate

**P50:** 0.014 ms | **P95:** 1.444 ms | **Min:** 0.003 ms | **Max:** 2.822 ms

## P95 Gate

**Global P95:** 1.444 ms
**Threshold:** 50.0 ms
**Status:** PASS
