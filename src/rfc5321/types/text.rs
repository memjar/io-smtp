//! SMTP text string (RFC 5321 §4.2).
//!
//! The human-readable text following the reply code on a response
//! line.

use core::fmt;

use alloc::{borrow::Cow, vec::Vec};

use bounded_static_derive::ToStatic;
use chumsky::prelude::*;

/// A human-readable text string used in SMTP responses.
#[derive(Clone, Debug, PartialEq, Eq, Hash, ToStatic)]
pub struct SmtpText<'a>(pub(crate) Cow<'a, str>);

impl SmtpText<'_> {
    /// Parses a text string from raw bytes, consuming the whole
    /// input.
    pub fn parse<'a>(bytes: &'a [u8]) -> Result<SmtpText<'a>, Vec<Rich<'a, u8>>> {
        parsers::text()
            .then_ignore(end())
            .parse(bytes)
            .into_result()
    }
}

impl fmt::Display for SmtpText<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.as_ref())
    }
}

impl<'a> From<SmtpText<'a>> for Cow<'a, str> {
    fn from(text: SmtpText<'a>) -> Self {
        text.0
    }
}

impl AsRef<str> for SmtpText<'_> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

pub(crate) mod parsers {
    //! Chumsky parser for the SMTP text string.

    use alloc::string::String;

    use chumsky::prelude::*;

    use crate::{rfc5321::SmtpText, utils::parsers::Extra};

    /// SMTP text string parser.
    ///
    /// ```abnf
    /// textstring     = 1*(%d09 / %d32-126)
    ///                ; HT, SP, Printable US-ASCII
    /// ```
    ///
    /// Bytes from 0x80 are accepted too, decoded as UTF-8 with replacement
    /// characters, since any text must be acceptable (RFC 5321 §4.2).
    ///
    /// Refs: <https://datatracker.ietf.org/doc/html/rfc5321#section-4.2>
    pub(crate) fn text<'a>() -> impl Parser<'a, &'a [u8], SmtpText<'a>, Extra<'a>> + Clone {
        any()
            .filter(|b| matches!(*b, 0x09 | 0x20..=0x7e | 0x80..=0xff))
            .repeated()
            .at_least(1)
            .to_slice()
            .map(String::from_utf8_lossy)
            .map(SmtpText)
    }
}
