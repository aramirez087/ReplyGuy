# CLI Reference

## Core commands

```bash
tuitbot init
tuitbot auth
tuitbot test
tuitbot run
tuitbot tick
tuitbot stats
tuitbot settings --show
tuitbot health
```

## Tick mode options

```bash
tuitbot tick --dry-run
tuitbot tick --loops discovery,content,analytics
tuitbot tick --ignore-schedule
tuitbot tick --output json
```

## Approval queue

```bash
tuitbot approve --list
tuitbot approve --approve <id>
tuitbot approve --reject <id>
tuitbot approve --approve-all
```

## Output modes

Most read-only commands support:

```bash
--output json
```

Use this for automation, monitoring, and alerting.
