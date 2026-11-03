---
cairn: change
id: bounded-replies
status: landed
created: 2026-11-03
---

# Bound the replies the coroutines buffer

## Why

The four read loops (`SmtpCommandSend`, `SmtpEhlo`, `SmtpGreetingGet`, `SmtpStartTls`) append every read to their buffer until the reply completes, with no limit: a server that never ends its reply, or never sends a CRLF, grows the client's memory until it dies. io-http closed the same hole before its 0.6.0 (e181530). No fuzz target covers the reply parsers either.

## What

- `send::SMTP_MAX_REPLY_LEN`, 64 KiB: a reply line holds 512 bytes at most (RFC 5321 4.5.3.1.5), so the limit leaves room for the longest EHLO response.
- Each loop fails once its buffer passes the limit: `TooLarge(usize)` on `SmtpCommandSendError` (which STARTTLS wraps), `SmtpEhloError` and `SmtpGreetingGetError`. Breaking, the enums being exhaustive.
- A fixed limit rather than a field: no SMTP reply legitimately approaches it, unlike an HTTP body.
- A `reply` fuzz target in fuzz/, with a `fuzz` devShell in the flake, as io-http has.
