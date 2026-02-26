# Session 06: Pagination Normalization Spec

## Overview

All list/search MCP tools now include normalized `meta.pagination` in their response envelope, giving agents a consistent contract for paginating through results.

## Wire Format

```json
{
  "success": true,
  "data": { ... },
  "meta": {
    "tool_version": "1.0",
    "elapsed_ms": 42,
    "pagination": {
      "next_token": "abc123",
      "result_count": 10,
      "has_more": true
    }
  }
}
```

## PaginationInfo Fields

| Field | Type | Description |
|-------|------|-------------|
| `next_token` | `string \| null` | Opaque token for the next page. Pass to `pagination_token` parameter. |
| `result_count` | `u32` | Number of results returned in this page. |
| `has_more` | `bool` | Whether more results exist. Derived from `next_token.is_some()`. |

## Extraction Sources

| Response Type | Source Fields | Used By |
|--------------|-------------|---------|
| `SearchResponse.meta` (SearchMeta) | `next_token`, `result_count` | `search_tweets`, `get_user_mentions`, `get_user_tweets`, `get_home_timeline`, `get_liked_tweets`, `get_bookmarks` |
| `UsersResponse.meta` (UsersMeta) | `next_token`, `result_count` | `get_followers`, `get_following`, `get_users_by_ids`, `get_tweet_liking_users` |

## Agent Usage Pattern

```
1. Call search_tweets(query: "...", max_results: 10)
2. Check meta.pagination.has_more
3. If true, call search_tweets(query: "...", pagination_token: meta.pagination.next_token)
4. Repeat until has_more is false
```

## Non-Paginated Tools

Single-entity tools (`get_tweet`, `get_user_by_username`, `get_user_by_id`, `get_me`) do NOT include `pagination` in meta (it is omitted via `skip_serializing_if`).
