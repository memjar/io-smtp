---
cairn: log
change: bounded-replies
landed: 2026-11-03
---

# Replies are bounded and fuzzed

The four read loops appended every read to their buffer until the reply completed, so a server never ending its reply grew the client's memory without end, the hole io-http closed in e181530. Each loop now fails once its buffer passes `send::SMTP_MAX_REPLY_LEN` (64 KiB): `TooLarge(usize)` on `SmtpCommandSendError`, which `SmtpStartTlsError::Send` wraps, `SmtpEhloError` and `SmtpGreetingGetError`. **Breaking**, the three enums being exhaustive. The limit is a constant rather than a field since no reply comes near it; 512 bytes a line (RFC 5321 4.5.3.1.5) leaves room for over a hundred `EHLO` lines.

`SmtpCommandSendOk` derives `Debug`, which the new send test needed to report an unexpected state.

The new `reply` fuzz target (fuzz/fuzz_targets/reply.rs, `fuzz` devShell added to flake.nix as io-http has it) drives the greeting, EHLO, STARTTLS, NOOP, LOGIN and SCRAM-SHA-256 coroutines over fuzzed reads then EOF, and the first read through the public reply parsers. A first run of 180 seconds made 2,077,223 executions without a finding (coverage 2120, corpus 838 inputs); inputs stayed under 24 KiB, so the limit itself is covered by the unit tests, not by that run.

Capabilities moved: replies (new: "Replies are bounded", "Replies are fuzzed").

Verified: the four `endless_*_too_large` tests feed 1 KiB continuation lines and fail within 100 reads, where the previous code asked for reads forever; `cargo clippy --all-targets --all-features` clean, `cargo test --all-features --lib` 112 passed.
