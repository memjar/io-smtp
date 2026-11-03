---
cairn: change
id: lenient-replies
status: landed
created: 2026-11-03
---

# Accept the code alone and any reply text

## Why

RFC 5321 4.2.1 says servers SHOULD send the space after the last code "but clients MUST be prepared for it to be omitted", and 4.2 that "any text, including no text at all ... MUST be acceptable". io-smtp did neither:

- A last line made of the code alone (`250\r\n`) never completed the reply, since every completeness check wanted a space as the fourth byte, so the session waited for bytes the server would never send.
- A byte from 0x80 in the text, a server answering in UTF-8, failed the whole parse, and the caller got a parse error instead of the code.

## What

- The reply, greeting and EHLO parsers take a last line of `code [SP [text]] CRLF`.
- `SmtpResponse::is_complete` takes a last line of three bytes as final, and `SmtpGreeting::is_complete` and `SmtpEhloResponse::is_complete`, which were copies of it, call it. The STARTTLS reader ends a reply at the code alone too.
- Reply text accepts bytes from 0x80, decoded as UTF-8 with replacement characters, borrowed when valid.
