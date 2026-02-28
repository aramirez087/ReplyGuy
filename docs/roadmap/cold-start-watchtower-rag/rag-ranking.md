# RAG Ranking: Scoring Rules & Retrieval Algorithm

## Overview

The Winning DNA pipeline enriches draft generation with historical performance data and ingested note context. When generating a new reply, the system retrieves high-performing "ancestors" (past tweets/replies) and injects them into the LLM prompt as reference patterns.

## Scoring Formula

### Engagement Score (Normalization)

Raw `performance_score` values are normalized to a 0.0-1.0 range:

```
engagement_score = performance_score / max_performance_score
```

Where `max_performance_score` is the highest `performance_score` across all tweet and reply performance records. If no performance data exists (cold-start), the score defaults to `0.5`.

### Retrieval Weight (Recency Decay)

Retrieval weight combines engagement score with exponential recency decay:

```
retrieval_weight = engagement_score * exp(-0.693 * days_since_posted / half_life)
```

This ensures recent successes are weighted more heavily than older ones, while still allowing historically strong content to contribute.

| Days Since Posted | Decay Factor (at 14-day half-life) |
|---|---|
| 0 | 1.000 |
| 7 | 0.707 |
| 14 | 0.500 |
| 28 | 0.250 |
| 56 | 0.063 |

## Classification Heuristics

### Reply Archetype Classification

Replies are classified into one of 5 archetypes using keyword/pattern matching:

| Archetype | Trigger Patterns |
|---|---|
| `ask_question` | Ends with `?`, starts with "what/how/why/have you/do you" |
| `share_experience` | Contains "I've found/noticed/experienced", "in my experience" |
| `add_data` | Contains "data/stats/study/research shows", "according to", "% of" |
| `respectful_disagree` | Contains "actually/however/but" AND "I think/I'd argue/not sure" |
| `agree_and_expand` | Default fallback |

### Tweet Format Classification

Tweets are classified into one of 7 formats:

| Format | Trigger Patterns |
|---|---|
| `list` | Contains "1." and "2." |
| `most_people_think_x` | Contains "most people think" or "everyone says" |
| `contrarian_take` | Contains "actually," and "but" |
| `before_after` | Contains "before:" or "after:" or "before ->" |
| `question` | Ends with `?` |
| `tip` | Starts with "tip:" or "pro tip:", contains "->", and < 200 chars |
| `storytelling` | Default fallback |

**Accuracy expectations:** These heuristics are intentionally conservative with broad fallback categories. They don't need to be highly accurate since they're used for retrieval weighting (grouping similar content), not for display or critical decisions. The defaults (`agree_and_expand` / `storytelling`) are safe catch-alls.

## Retrieval Algorithm

1. **Query scored ancestors** from `tweet_performance` + `reply_performance` where `engagement_score >= MIN_ENGAGEMENT_SCORE` (0.1). Fetch up to 50 candidates.
2. **Filter** ancestors missing `engagement_score` (not yet scored).
3. **Compute retrieval weight** for each ancestor using recency decay.
4. **Sort** by `retrieval_weight DESC`.
5. **Take top K** ancestors (default 5).
6. **Format prompt block** with character cap (2000 chars).
7. **Cold-start fallback:** If no ancestors exist, retrieve pending draft seeds from the `draft_seeds` table instead.

## Cold-Start Behavior

When a new user has no performance data:
- `get_max_performance_score()` returns 0.0
- `compute_engagement_score()` returns the cold-start baseline (0.5)
- No ancestors will have `engagement_score` set in the DB yet
- The system falls back to `draft_seeds` from ingested notes
- All new seeds start with `engagement_weight = 0.5`

As the user publishes content and performance data accumulates, ancestors will be scored and the cold-start path becomes unused.

## Thresholds

| Parameter | Value | Rationale |
|---|---|---|
| `RECENCY_HALF_LIFE_DAYS` | 14.0 | Two weeks: recent successes matter more. Decays to ~6% at 8 weeks. |
| `MAX_ANCESTORS` | 5 | Keeps prompt additions to ~2500 chars. Diminishing returns beyond 5 examples. |
| `COLD_START_WEIGHT` | 0.5 | Midpoint: unscored content gets equal chance. Neither favored nor penalized. |
| `MIN_ENGAGEMENT_SCORE` | 0.1 | Excludes bottom ~10% performers. Prevents worst content from influencing new drafts. |
| `RAG_MAX_CHARS` | 2000 | Conservative estimate at ~500 tokens. Keeps total system prompt under 4K tokens. |
| `SEED_BATCH_SIZE` | 5 | Process 5 content nodes per worker tick. Balances throughput vs. LLM cost. |
| `SEED_WORKER_INTERVAL_SECS` | 300 | 5 minutes. Low priority: no urgency in seed generation. |
| `MAX_COLD_START_SEEDS` | 5 | Same as MAX_ANCESTORS for consistent context size. |

All thresholds are defined as `const` values in `context/winning_dna.rs` and `automation/seed_worker.rs`. They are not configurable via TOML in v1.

## Prompt Injection Format

### With Ancestors (Normal Path)

```
Winning patterns from your best-performing content:
1. [tip] (tweet): "Truncated content preview..."
2. [ask_question] (reply): "Another preview..."
Use these patterns as inspiration but don't copy them directly.
```

### With Seeds (Cold-Start Path)

```
Relevant ideas from your notes:
1. "Hook about Rust ownership" (from: Rust Tips)
2. "Why async matters for web servers" (from: Blog Draft)
Draw on these ideas to make your response more informed.
```

### No Data

When neither ancestors nor seeds exist, the prompt block is empty and no RAG section is injected. This preserves the exact behavior of the pre-RAG draft pipeline.
