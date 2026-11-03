---
cairn: delta
change: lenient-replies
---

## ADDED Requirements

### Requirement: A last line may be the code alone
The reply, greeting and `EHLO` parsers and their completeness checks SHALL take a last line of the code alone, with or without its space, as the end of the reply (RFC 5321 4.2.1), its text being empty.

### Requirement: Any reply text is acceptable
Reply text SHALL accept HT, every byte from 0x20 but DEL, decoded as UTF-8 with replacement characters for invalid sequences (RFC 5321 4.2), so a server answering in UTF-8 still yields its code.

## MODIFIED Requirements

## REMOVED Requirements
