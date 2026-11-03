---
cairn: tasks
change: auth-mechanism-reader
---

# Tasks

- [x] Add `SmtpAuthCapability::available_mechanisms`, ordered most preferred first and LOGIN last.
- [x] Gate the SCRAM-SHA-256 arm on the `scram` feature.
- [x] Cover an unrecognised name, an empty AUTH line, the ordering, and both sides of the `scram` gate.
- [x] CHANGELOG.md.
- [x] Fold the delta into cairn/spec/auth.md and log it.
