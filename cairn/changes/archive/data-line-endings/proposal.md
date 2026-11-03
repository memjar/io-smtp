---
cairn: change
id: data-line-endings
status: landed
created: 2026-11-03
---

# Send DATA with CRLF line endings, in one pass

## Why

RFC 5321 2.3.8 forbids a client to send a bare CR or a bare LF: every line ends with CRLF. `SmtpData::prepare_body` claimed to normalise line endings but only fixed the last one, so a message written with Unix line endings went out with bare LFs, which servers refusing them (a common hardening against SMTP smuggling) reject. A bare CR passed too, and `\r.\r\n` is a sequence a server reading bare CR as a line end takes for the end of the data.

The preparation also cost more than it had to: `strip_bcc` copied the whole message, the body line by line, then `prepare_body` copied it again byte by byte, so a message lived three times in memory.

## What

- `prepare_body` turns every bare CR and bare LF into CRLF and doubles a dot starting any line, bare line endings included, in one pass that copies runs between the bytes it rewrites; a message needing neither is reused, not copied.
- `strip_bcc` takes the message by value, scans the header section only, and returns the message as is when it holds no `Bcc` field.
- Both scan through a crate-private 16-byte block search, the one io-http uses.
- `keep_bcc` keeps the header section as given; the line endings and dot-stuffing still apply, the wire allowing nothing else.
