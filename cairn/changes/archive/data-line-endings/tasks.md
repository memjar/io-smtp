---
cairn: tasks
change: data-line-endings
---

# Tasks

- [x] Rewrite `prepare_body` as one pass over run copies, reusing a message needing no rewrite
- [x] Take the message by value in `strip_bcc`, scanning the header section only
- [x] Test bare line endings, dots after them, the empty message, and the whole rewrite against a byte-by-byte one over every message of up to 8 bytes from `a . CR LF`
- [x] Time it against the previous code
- [x] CHANGELOG, spec, log
