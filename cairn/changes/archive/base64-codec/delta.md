---
cairn: delta
change: base64-codec
---

## ADDED Requirements

### Requirement: AUTH payloads decode leniently
Challenges and continuation lines SHALL decode leniently, accepting a missing padding, non-zero trailing bits and blanks, and SHALL fail with the `Base64` variant of their error on any other byte outside the alphabet or on data after the padding.

The text of a SCRAM-SHA-256 success reply SHALL be read as the server-final-message only when, past its enhanced status code, it is one blank-free token decoding to a message starting with `v=` or `e=`. Any other text, such as `Authentication successful` or `Accepted`, is human text and leaves the mechanism to refuse the unverified exchange or to complete the verified one.

## MODIFIED Requirements

## REMOVED Requirements
