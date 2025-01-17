use crate::{codec::Encode, types::core::NString};
use std::io::Write;

/// An address structure describes an electronic mail address.
#[derive(Debug, Clone, PartialEq)]
pub struct Address {
    /// Personal name
    name: NString,
    /// At-domain-list (source route)
    adl: NString,
    /// Mailbox name
    mailbox: NString,
    /// Host name
    host: NString,
}

impl Address {
    pub fn new(name: NString, adl: NString, mailbox: NString, host: NString) -> Address {
        Address {
            name,
            adl,
            mailbox,
            host,
        }
    }
}

impl Encode for Address {
    fn encode(&self, writer: &mut impl Write) -> std::io::Result<()> {
        writer.write_all(b"(")?;
        self.name.encode(writer)?;
        writer.write_all(b" ")?;
        self.adl.encode(writer)?;
        writer.write_all(b" ")?;
        self.mailbox.encode(writer)?;
        writer.write_all(b" ")?;
        self.host.encode(writer)?;
        writer.write_all(b")")?;

        Ok(())
    }
}
