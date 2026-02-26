# Session 07 — Toolset by Profile

## Tool Matrix

| Tool | Profile(s) | Lane | Category | Mutation | X Client | LLM | DB |
|------|-----------|------|----------|----------|----------|-----|-----|
| `get_stats` | WF | workflow | analytics | - | - | - | x |
| `get_follower_trend` | WF | workflow | analytics | - | - | - | x |
| `get_action_log` | WF | workflow | analytics | - | - | - | x |
| `get_action_counts` | WF | workflow | analytics | - | - | - | x |
| `get_rate_limits` | WF | workflow | policy | - | - | - | x |
| `get_recent_replies` | WF | workflow | analytics | - | - | - | x |
| `get_reply_count_today` | WF | workflow | analytics | - | - | - | x |
| `list_target_accounts` | WF | workflow | discovery | - | - | - | x |
| `list_unreplied_tweets` | WF | workflow | discovery | - | - | - | x |
| `score_tweet` | BOTH | shared | scoring | - | - | - | - |
| `list_pending_approvals` | WF | workflow | approval | - | - | - | x |
| `get_pending_count` | WF | workflow | approval | - | - | - | x |
| `approve_item` | WF | workflow | approval | x | x | - | x |
| `reject_item` | WF | workflow | approval | x | - | - | x |
| `approve_all` | WF | workflow | approval | x | x | - | x |
| `generate_reply` | WF | workflow | content | - | - | x | x |
| `generate_tweet` | WF | workflow | content | - | - | x | x |
| `generate_thread` | WF | workflow | content | - | - | x | x |
| `get_config` | BOTH | shared | config | - | - | - | - |
| `validate_config` | BOTH | shared | config | - | - | - | - |
| `get_capabilities` | BOTH | shared | meta | - | - | - | - |
| `health_check` | BOTH | shared | health | - | - | - | x |
| `get_mode` | BOTH | shared | meta | - | - | - | - |
| `get_policy_status` | WF | workflow | policy | - | - | - | x |
| `compose_tweet` | WF | workflow | write | x | x | - | x |
| `get_discovery_feed` | WF | workflow | discovery | - | - | - | x |
| `suggest_topics` | WF | workflow | content | - | - | - | x |
| `get_tweet_by_id` | BOTH | shared | read | - | x | - | - |
| `x_get_user_by_username` | BOTH | shared | read | - | x | - | - |
| `x_search_tweets` | BOTH | shared | read | - | x | - | - |
| `x_get_user_mentions` | BOTH | shared | read | - | x | - | - |
| `x_get_user_tweets` | BOTH | shared | read | - | x | - | - |
| `x_get_home_timeline` | BOTH | shared | read | - | x | - | - |
| `x_get_followers` | BOTH | shared | read | - | x | - | - |
| `x_get_following` | BOTH | shared | read | - | x | - | - |
| `x_get_user_by_id` | BOTH | shared | read | - | x | - | - |
| `x_get_liked_tweets` | BOTH | shared | read | - | x | - | - |
| `x_get_bookmarks` | BOTH | shared | read | - | x | - | - |
| `x_get_users_by_ids` | BOTH | shared | read | - | x | - | - |
| `x_get_tweet_liking_users` | BOTH | shared | read | - | x | - | - |
| `get_x_usage` | WF | workflow | analytics | - | - | - | x |
| `x_get_me` | API | shared | read | - | x | - | - |
| `x_post_tweet` | BOTH | shared | write | x | x | - | x |
| `x_reply_to_tweet` | BOTH | shared | write | x | x | - | x |
| `x_quote_tweet` | BOTH | shared | write | x | x | - | x |
| `x_delete_tweet` | BOTH | shared | write | x | x | - | x |
| `x_post_thread` | BOTH | shared | write | x | x | - | x |
| `x_like_tweet` | BOTH | shared | engage | x | x | - | x |
| `x_unlike_tweet` | BOTH | shared | engage | x | x | - | x |
| `x_follow_user` | BOTH | shared | engage | x | x | - | x |
| `x_unfollow_user` | BOTH | shared | engage | x | x | - | x |
| `x_retweet` | BOTH | shared | engage | x | x | - | x |
| `x_unretweet` | BOTH | shared | engage | x | x | - | x |
| `x_bookmark_tweet` | BOTH | shared | engage | x | x | - | x |
| `x_unbookmark_tweet` | BOTH | shared | engage | x | x | - | x |
| `x_upload_media` | BOTH | shared | media | x | x | - | - |
| `get_author_context` | WF | workflow | context | - | x | - | x |
| `recommend_engagement_action` | WF | workflow | context | - | x | - | x |
| `topic_performance_snapshot` | WF | workflow | context | - | - | - | x |
| `get_mcp_tool_metrics` | WF | workflow | telemetry | - | - | - | x |
| `get_mcp_error_breakdown` | WF | workflow | telemetry | - | - | - | x |
| `find_reply_opportunities` | WF | workflow | composite | - | x | - | x |
| `draft_replies_for_candidates` | WF | workflow | composite | - | - | x | x |
| `propose_and_queue_replies` | WF | workflow | composite | x | x | - | x |
| `generate_thread_plan` | WF | workflow | composite | - | - | x | - |

## Summary

| Profile | Lane | Tools |
|---------|------|-------|
| API only | shared | 1 (`x_get_me`) |
| BOTH | shared | 27 (read/write/engage/media/config/meta/health/scoring) |
| Workflow only | workflow | 27 (analytics/approval/content/discovery/policy/context/telemetry/composite) |
| **Total** | | **55** |
