---
cairn: log
change: lenient-replies
landed: 2026-11-03
---

# The code alone and any reply text are accepted

RFC 5321 4.2.1 lets a server drop the space after the last code ("clients MUST be prepared for it to be omitted") and 4.2 makes any text acceptable. A scratch crate on the 0.6.0 code showed both failing: `SmtpResponse::is_complete(b"250\r\n")` was false, so a coroutine would wait forever for the rest of a reply already complete, and `SmtpResponse::parse` refused `550 5.7.1 Größe`, the caller losing the code to a parse error.

The final line of the reply, greeting and EHLO parsers is now `code [SP [text]] CRLF`. `SmtpGreeting::is_complete` and `SmtpEhloResponse::is_complete` were copies of `SmtpResponse::is_complete` and now call it, which takes a three-byte last line as final; the STARTTLS `reply_end` matches the code alone too. The text parser accepts bytes from 0x80 and decodes them with `String::from_utf8_lossy`, borrowing when the text is valid UTF-8. EHLO capability lines stay ASCII.

A 120-second run of the `reply` fuzz target made 1,297,147 executions on the changed parsers without a finding.

Capabilities moved: replies (ADDED "A last line may be the code alone", "Any reply text is acceptable").

Verified: tests/parsers.rs gains `response_bare_code`, `response_parse_non_ascii_text`, `greeting_parse_multiline_bare_last_code` and `ehlo_response_bare_last_code` (40 passed), starttls.rs `bare_code_success_returns_remaining`; `cargo clippy --all-targets --all-features` clean, 117 unit tests passed.
