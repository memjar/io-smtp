//! AUTH EHLO capability (RFC 4954 §4).

use alloc::vec::Vec;

use io_sasl::mechanism::SaslMechanism;
use thiserror::Error;

/// EHLO capability keyword for SMTP authentication.
pub const AUTH: &str = "AUTH";

/// The AUTH EHLO capability: the set of SASL mechanisms offered by the server.
///
/// Borrows directly from the raw capability line returned by
/// `SmtpEhloResponse::get_capability(AUTH)`.
///
/// # Example
///
/// ```ignore
/// use io_smtp::rfc4954::capability::{AUTH, SmtpAuthCapability};
/// use io_smtp::rfc4616::plain::PLAIN;
/// use io_smtp::login::LOGIN;
///
/// let cap = SmtpAuthCapability::parse(ehlo.get_capability(AUTH).unwrap()).unwrap();
/// assert!(cap.has(PLAIN));
/// assert!(cap.has(LOGIN));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmtpAuthCapability<'a>(Vec<&'a str>);

impl<'a> SmtpAuthCapability<'a> {
    /// Parses the raw AUTH capability line (e.g. `"AUTH PLAIN LOGIN"`).
    pub fn parse(s: &'a str) -> Result<Self, SmtpAuthCapabilityError> {
        let mut parts = s.split_ascii_whitespace();

        match parts.next() {
            Some(kw) if kw.eq_ignore_ascii_case(AUTH) => {}
            _ => return Err(SmtpAuthCapabilityError),
        }

        Ok(SmtpAuthCapability(parts.collect()))
    }

    /// Returns `true` if the given SASL mechanism is advertised.
    ///
    /// The comparison is case-insensitive.
    pub fn has(&self, mechanism: &str) -> bool {
        self.0.iter().any(|m| m.eq_ignore_ascii_case(mechanism))
    }

    /// Returns an iterator over the advertised mechanism names.
    pub fn mechanisms(&self) -> impl Iterator<Item = &str> {
        self.0.iter().copied()
    }

    /// Returns the advertised mechanisms this build frames, most preferred
    /// first and LOGIN last, dropping the others.
    pub fn available_mechanisms(&self) -> Vec<SaslMechanism> {
        let preferred = [
            #[cfg(feature = "scram")]
            SaslMechanism::ScramSha256,
            SaslMechanism::Plain,
            SaslMechanism::OAuthBearer,
            SaslMechanism::XOAuth2,
            SaslMechanism::Anonymous,
            SaslMechanism::Login,
        ];

        preferred
            .into_iter()
            .filter(|mechanism| self.has(mechanism.as_str()))
            .collect()
    }
}

/// Error returned when parsing an AUTH capability string fails.
#[derive(Debug, Error)]
#[error("invalid AUTH capability string")]
pub struct SmtpAuthCapabilityError;

#[cfg(test)]
mod tests {
    use alloc::vec;

    use io_sasl::mechanism::SaslMechanism;

    use crate::rfc4954::capability::*;

    #[test]
    fn available_mechanisms_are_ordered_and_framed_only() {
        let line = "AUTH LOGIN xoauth2 CRAM-MD5 PLAIN SCRAM-SHA-256 OAUTHBEARER GSSAPI";
        let cap = SmtpAuthCapability::parse(line).unwrap();

        let expected = vec![
            #[cfg(feature = "scram")]
            SaslMechanism::ScramSha256,
            SaslMechanism::Plain,
            SaslMechanism::OAuthBearer,
            SaslMechanism::XOAuth2,
            SaslMechanism::Login,
        ];

        assert_eq!(cap.available_mechanisms(), expected);
        assert_eq!(cap.mechanisms().count(), 7);
    }

    #[test]
    fn an_empty_auth_line_offers_nothing() {
        let cap = SmtpAuthCapability::parse("AUTH").unwrap();
        assert!(cap.available_mechanisms().is_empty());
    }
}
