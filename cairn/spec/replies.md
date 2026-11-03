---
cairn: spec
capability: replies
status: current
---

# Replies

How the coroutines read what a server answers: the greeting, the `EHLO` response and the reply to every other command (RFC 5321 4.2).

## Requirement: Replies are bounded

The coroutines reading a reply (the greeting, `EHLO`, `STARTTLS` and every command) SHALL fail with `TooLarge(SMTP_MAX_REPLY_LEN)` once the bytes buffered for one reply exceed `SMTP_MAX_REPLY_LEN`, 64 KiB, instead of reading on. A reply line holds 512 bytes at most (RFC 5321 4.5.3.1.5), so the limit leaves room for the longest `EHLO` response while a server that never ends its reply cannot exhaust memory.

### Scenario: An endless multi-line reply
- GIVEN a server answering with `250-` continuation lines forever
- WHEN the coroutine has buffered more than 64 KiB
- THEN it completes with `TooLarge(65536)` and asks for no further read

## Requirement: Replies are fuzzed

A `reply` fuzz target SHALL feed arbitrary reads followed by EOF to the reading coroutines and to the LOGIN and SCRAM-SHA-256 exchanges, and the first read to the reply, greeting, EHLO and challenge parsers, asserting no panic, no read asked for after EOF more than twice, and no `TooLarge` before the limit was passed.
