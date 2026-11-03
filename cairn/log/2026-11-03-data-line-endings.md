---
cairn: log
change: data-line-endings
landed: 2026-11-03
---

# DATA lines end with CRLF, in one pass

`SmtpData::prepare_body` documented a line ending normalisation it did not do: it fixed the last line only, so `Subject: x\n\nline1\nline2\r.\r\nend` went on the wire with its bare LFs and its bare `\r.\r\n` (checked against the 0.6.0 code from a scratch crate). RFC 5321 2.3.8 forbids both, servers hardened against SMTP smuggling refuse bare LFs, and a server reading a bare CR as a line end takes `\r.\r\n` for the end of the data. It now turns every bare CR and bare LF into CRLF and doubles the dot after any line ending. himalaya already rewrote bare LFs before handing a message over (its `crlf::normalize`), never bare CRs; other consumers sent both.

Both rewrites now scan through src/scan.rs, the 16-byte block search io-http introduced in 39aed8c, and copy runs between the bytes they change. `prepare_body` reuses a message needing no rewrite, and `strip_bcc` takes the message by value, scans the header section only and returns a message without `Bcc` as is: a CRLF message goes out without a single copy where it used to be copied twice.

Timed through `SmtpMessageSend` up to the `DATA` write, 10 MiB of 78-byte lines, best of 15 wall-clock runs pinned to one core (indicative, not a cycle count): CRLF lines 8255 to 584 us, with a `Bcc` field 8235 to 782 us, LF lines 8213 to 678 us (output 136,182 bytes longer, one CR a line).

The new `matches_a_byte_by_byte_rewrite` test compares the rewrite against a per-byte reference over all 87,381 messages of up to 8 bytes from `a . CR LF`. `turns_bare_line_endings_into_crlf` and `dot_stuffs_after_bare_line_endings` fail against the previous code, which left both endings bare.

`keep_bcc` kept the message "as given" in the spec; it now keeps the header section as given, the line ending and dot rules applying to every message.

Capabilities moved: submission (ADDED "DATA lines end with CRLF", MODIFIED "Blind recipients stay blind").

Verified: `cargo clippy --all-targets --all-features` clean, `cargo test --all-features --lib` 116 passed.
