---
cairn: delta
change: data-line-endings
---

## ADDED Requirements

### Requirement: DATA lines end with CRLF
`SmtpData` SHALL transmit every bare CR and every bare LF of the message as CRLF (RFC 5321 2.3.8), SHALL double the dot starting any line, a line after a bare line ending included (RFC 5321 4.5.2), SHALL end the last line with CRLF, and SHALL then send the `.` line. A message already in that form SHALL be transmitted byte for byte.

## MODIFIED Requirements

### Requirement: Blind recipients stay blind
`SmtpMessageSend` SHALL remove every `Bcc` field from the header section of the transmitted message (RFC 5322 3.6.3), matching the name case-insensitively and with the obsolete whitespace before the colon, removing its folded lines too, and leaving everything after the first empty line untouched. The forward paths are untouched, so the blind recipients still receive the message.

`SmtpMessageSendOptions::keep_bcc` SHALL transmit the header section as given, for a caller whose `Bcc` field must reach the server. The `DATA` rules on line endings and dots still apply.

## REMOVED Requirements
