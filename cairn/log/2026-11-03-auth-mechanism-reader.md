---
cairn: log
change: auth-mechanism-reader
landed: 2026-11-03
---

# The AUTH line reads into mechanisms

`SmtpAuthCapability::available_mechanisms` returns the advertised mechanisms this build frames as `SaslMechanism` values, in io-imap's order (SCRAM-SHA-256, PLAIN, OAUTHBEARER, XOAUTH2, ANONYMOUS, LOGIN), so neverest's wizard can offer what the SMTP server accepts the way it already does for IMAP. It filters a fixed preference list through `has`, so the names match case-insensitively; SCRAM-SHA-256 sits behind the `scram` feature, the only one this crate frames under a feature, and its `-PLUS` name is left out since `SmtpAuthScramSha256` always sends the plain one. An empty `AUTH` line yields nothing: unlike IMAP, there is no `LOGIN` command to fall back on. `mechanisms` is unchanged.

The proposal had waited since 2026-08-27; it landed with the pre-release pass of 2026-11-03.

Capabilities moved: auth (MODIFIED "The advertised mechanisms are readable from the EHLO response").

Verified: `available_mechanisms_are_ordered_and_framed_only` and `an_empty_auth_line_offers_nothing` pass with the default features and with `--no-default-features`, where SCRAM-SHA-256 drops out; `cargo clippy --all-targets --all-features` clean.
