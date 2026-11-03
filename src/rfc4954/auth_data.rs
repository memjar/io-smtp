//! SMTP AUTH continuation data (RFC 4954 §4).

use alloc::{boxed::Box, vec::Vec};

use base64_codec::codec::Base64Codec;
use secrecy::{ExposeSecret, SecretBox};
use thiserror::Error;

/// Errors that can occur while parsing auth data.
#[derive(Debug, Error)]
pub enum SmtpAuthDataError {
    /// The input carries no terminating CRLF yet.
    #[error("Parse SMTP auth data error: incomplete input")]
    Incomplete,
    /// The payload holds a byte outside the base64 alphabet, or data after
    /// its padding.
    #[error("Parse SMTP auth data error: invalid base64")]
    Base64,
}

/// Errors that can occur while decoding a server challenge.
#[derive(Clone, Debug, Error)]
pub enum SmtpAuthChallengeError {
    /// The payload holds a byte outside the base64 alphabet, or data after
    /// its padding.
    #[error("Parse SMTP auth challenge error: invalid base64")]
    Base64,
}

/// Decodes the payload a server challenge carries.
///
/// The text of a 334 reply is the base64 of the SASL message the
/// mechanism has to answer, empty when the mechanism expects no
/// message. The same encoding carries the final server message when a
/// server puts it in the 235 reply instead, as RFC 4954 section 4
/// allows.
///
/// The payload decodes leniently: a missing padding, non-zero trailing
/// bits and blanks are accepted.
pub fn parse_challenge(text: &str) -> Result<Vec<u8>, SmtpAuthChallengeError> {
    let mut decoded = Vec::new();

    match Base64Codec::default().decode_vec(text.as_bytes(), &mut decoded) {
        true => Ok(decoded),
        false => Err(SmtpAuthChallengeError::Base64),
    }
}

/// Data line used during SMTP AUTH exchange.
///
/// Holds the raw binary data, i.e., a `Vec<u8>`, *not* the BASE64
/// string.
#[derive(Debug)]
pub enum SmtpAuthData {
    /// Continue SASL authentication with response data.
    Continue(SecretBox<[u8]>),
    /// Cancel SASL authentication.
    ///
    /// The client sends a single `*` to cancel the authentication
    /// exchange.
    Cancel,
}

impl SmtpAuthData {
    /// Create a continuation response with the given data.
    pub fn r#continue(data: impl Into<Box<[u8]>>) -> Self {
        Self::Continue(SecretBox::new(data.into()))
    }

    /// Returns true if `buf` contains a complete auth data line.
    pub fn is_complete(buf: &[u8]) -> bool {
        buf.ends_with(b"\r\n")
    }

    /// Parse auth data from bytes.
    pub fn parse(input: &[u8]) -> Result<SmtpAuthData, SmtpAuthDataError> {
        if !input.ends_with(b"\r\n") {
            return Err(SmtpAuthDataError::Incomplete);
        }

        let line = &input[..input.len() - 2];
        if line == b"*" {
            return Ok(SmtpAuthData::Cancel);
        }

        let mut decoded = Vec::new();

        if !Base64Codec::default().decode_vec(line, &mut decoded) {
            return Err(SmtpAuthDataError::Base64);
        }

        Ok(SmtpAuthData::r#continue(decoded.into_boxed_slice()))
    }
}

impl From<SmtpAuthData> for Vec<u8> {
    fn from(data: SmtpAuthData) -> Vec<u8> {
        let mut buf = Vec::new();

        match data {
            SmtpAuthData::Continue(secret) => {
                Base64Codec::default().encode_vec(secret.expose_secret(), &mut buf);
            }
            SmtpAuthData::Cancel => buf.push(b'*'),
        }

        buf.extend_from_slice(b"\r\n");
        buf
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use secrecy::ExposeSecret;

    use crate::rfc4954::auth_data::*;

    #[test]
    fn challenge_decodes_leniently() {
        assert_eq!(parse_challenge("").unwrap(), b"");
        assert_eq!(parse_challenge("VXNlcm5hbWU6").unwrap(), b"Username:");
        assert_eq!(parse_challenge(" VXNlcm5hbWU ").unwrap(), b"Username");

        assert!(matches!(
            parse_challenge("dTo=dTpw"),
            Err(SmtpAuthChallengeError::Base64)
        ));
    }

    #[test]
    fn data_round_trips() {
        let bytes: Vec<u8> = SmtpAuthData::r#continue(*b"\0alice\0secret").into();
        assert_eq!(bytes, b"AGFsaWNlAHNlY3JldA==\r\n");

        let SmtpAuthData::Continue(data) = SmtpAuthData::parse(&bytes).unwrap() else {
            panic!("expected SmtpAuthData::Continue");
        };
        assert_eq!(data.expose_secret(), b"\0alice\0secret");

        let cancel: Vec<u8> = SmtpAuthData::Cancel.into();
        assert!(matches!(
            SmtpAuthData::parse(&cancel),
            Ok(SmtpAuthData::Cancel)
        ));
    }
}
