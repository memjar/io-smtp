---
cairn: delta
change: bounded-replies
---

## ADDED Requirements

### Requirement: Replies are bounded
The coroutines reading a reply (the greeting, `EHLO`, `STARTTLS` and every command) SHALL fail with `TooLarge(SMTP_MAX_REPLY_LEN)` once the bytes buffered for one reply exceed `SMTP_MAX_REPLY_LEN`, 64 KiB, instead of reading on.

### Requirement: Replies are fuzzed
A `reply` fuzz target SHALL feed arbitrary reads followed by EOF to the reading coroutines and to the LOGIN and SCRAM-SHA-256 exchanges, and the first read to the reply, greeting, EHLO and challenge parsers, asserting no panic, no read asked for after EOF more than twice, and no `TooLarge` before the limit was passed.

## MODIFIED Requirements

## REMOVED Requirements
