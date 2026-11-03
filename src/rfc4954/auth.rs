//! The SMTP AUTH command (RFC 4954 §4).

use alloc::{borrow::Cow, vec::Vec};

use base64_codec::codec::Base64Codec;
use secrecy::{ExposeSecret, SecretBox};

/// The AUTH command (RFC 4954 §4).
///
/// Serializes to `AUTH <mechanism> [<base64-ir>]\r\n`.
pub struct SmtpAuthCommand<'a> {
    /// The SASL mechanism name as it appears on the wire (e.g. `"PLAIN"`).
    pub mechanism: Cow<'a, str>,
    /// Optional initial response (base64-encoded on serialization).
    pub initial_response: Option<SecretBox<[u8]>>,
}

impl<'a> From<SmtpAuthCommand<'a>> for Vec<u8> {
    fn from(cmd: SmtpAuthCommand<'a>) -> Vec<u8> {
        let mut buf = Vec::new();

        buf.extend_from_slice(b"AUTH ");
        buf.extend_from_slice(cmd.mechanism.as_bytes());

        if let Some(ir) = cmd.initial_response {
            let data = ir.expose_secret();

            if data.is_empty() {
                buf.extend_from_slice(b" =");
            } else {
                buf.push(b' ');
                Base64Codec::default().encode_vec(data, &mut buf);
            }
        }

        buf.extend_from_slice(b"\r\n");
        buf
    }
}
