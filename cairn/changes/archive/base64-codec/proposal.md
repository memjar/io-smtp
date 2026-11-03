---
cairn: change
id: base64-codec
status: landed
created: 2026-11-03
---

# Run the AUTH base64 on base64-codec

## Why

The family moves every base64 use to base64-codec (decided 2026-11-03), io-http and io-sasl first. io-smtp's only base64 is the `AUTH` framing of RFC 4954: the initial response, the continuation lines and the challenges. Its two error types carried the base64 crate's error as a string, so that crate's messages reached io-smtp's API.

## What

- `SmtpAuthCommand`, `SmtpAuthData` and `parse_challenge` encode and decode through `Base64Codec::default()`, encoding straight into the command buffer.
- `SmtpAuthDataError::Base64` and `SmtpAuthChallengeError::Base64` become unit variants. Breaking.
- Decoding becomes lenient (missing padding, non-zero trailing bits and blanks accepted), the codec's only mode.
- The SCRAM success reply stops relying on strict decoding to tell a server-final-message from human text: it takes one blank-free token decoding to a `v=` or `e=` message, and reads anything else as text.
