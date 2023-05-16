use super::SPFContext;
use std::net::{IpAddr, Ipv6Addr};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use hex::encode;

pub trait ToDomainString {
    fn get_domain_string(&self, ctx: &SPFContext) -> String;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainSpec {
    pub macro_string: MacroString,
    pub domain_end: DomainEnd,
}

impl ToDomainString for DomainSpec {
    fn get_domain_string(&self, ctx: &SPFContext) -> String {
        let macro_str = self.macro_string.get_domain_string(ctx);
        let domain_end = self.domain_end.get_domain_string(ctx);
        return macro_str + &domain_end;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainEnd {
    TopLabel(String),
    MacroExpand(MacroExpand),
}

impl ToDomainString for DomainEnd {
    fn get_domain_string(&self, ctx: &SPFContext) -> String {
        match self {
            Self::TopLabel(tl) => tl.to_owned(),
            Self::MacroExpand(e) => e.get_domain_string(ctx),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MacroStringItem {
    MacroExpand(MacroExpand),
    MacroLiteral(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroString {
    pub items: Vec<MacroStringItem>,
}

impl ToDomainString for MacroString {
    fn get_domain_string(&self, ctx: &SPFContext) -> String {
        unimplemented!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MacroExpand {
    MacroPayload(MacroPayload),
    PercentSign,
    SingleSpace,
    UrlEncodeSpace,
}

impl ToDomainString for MacroExpand {
    fn get_domain_string(&self, ctx: &SPFContext) -> String {
        match self {
            Self::MacroPayload(payload) => payload.get_domain_string(ctx),
            Self::PercentSign => "%".to_owned(),
            Self::SingleSpace => " ".to_owned(),
            Self::UrlEncodeSpace => "%20".to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroPayload {
    pub letter: MacroLetter,
    pub transformers: Transformers,
    pub delimiters: Vec<Delimiter>,
}

impl ToDomainString for MacroPayload {
    fn get_domain_string(&self, ctx: &SPFContext) -> String {
        unimplemented!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transformers {
    pub digit: Option<u16>,
    pub reverse: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Delimiter {
    Dot,
    Hyphen,
    Plus,
    Comma,
    Slash,
    Underscore,
    Equal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MacroLetter {
    Sender,
    SenderLocalPart,
    SenderDomain,
    Domain,
    IP,
    DomainOfIP,
    IPVersion,
    HELODomain,
    SMTPClientIP,
    ReceiverDomain,
    Timestamp,
}

impl ToDomainString for MacroLetter {
    fn get_domain_string(&self, ctx: &SPFContext) -> String {
        match self {
            Self::Sender => ctx.sender.to_owned(),
            Self::SenderLocalPart => ctx.sender.split_once('@').unwrap().0.to_owned(),
            Self::SenderDomain => ctx.sender.split_once('@').unwrap().1.to_owned(),
            Self::Domain => ctx.domain.to_owned(),
            Self::IP => match ctx.ip {
                IpAddr::V4(v4) => v4.to_string(),
                IpAddr::V6(v6) => v6.get_domain_string(ctx),
            },
            Self::DomainOfIP => unimplemented!(), // TODO: Get the PTR of the IP
            Self::IPVersion => match ctx.ip {
                IpAddr::V4(_) => "in-addr".to_owned(),
                IpAddr::V6(_) => "ip6".to_owned(),
            },
            Self::HELODomain => ctx.helo.clone(),
            Self::SMTPClientIP => ctx.ip.to_string(),
            Self::ReceiverDomain => ctx.host_name.clone(),
            Self::Timestamp => SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
                .to_string(),
        }
    }
}

impl ToDomainString for Ipv6Addr {
    fn get_domain_string(&self, _ctx: &SPFContext) -> String {
        let oct_str = encode(self.octets());
        let mut chars = oct_str.chars();
        let mut ret = String::with_capacity(oct_str.len() * 2);
        if let Some(c) = chars.next() {
            ret.push(c)
        }
        for c in chars {
            ret.push('.');
            ret.push(c);
        }
        ret
    }
}

#[cfg(test)]
mod test {
    use hex::encode;
    use std::net::Ipv6Addr;

    #[test]
    fn test_ipv6_format() {
        let ip = Ipv6Addr::new(0xff, 0x11, 0x12, 0x3e, 0x72, 0x80, 0x01, 0x00);
        let s = encode(ip.octets());
        assert_eq!(s, "00ff00110012003e0072008000010000".to_owned());

        let mut chars = s.chars();
        let mut string = String::with_capacity(s.len() * 2);
        if let Some(c) = chars.next() {
            string.push(c);
        }
        for c in chars {
            string.push('.');
            string.push(c);
        }
        assert_eq!(
            &string,
            "0.0.f.f.0.0.1.1.0.0.1.2.0.0.3.e.0.0.7.2.0.0.8.0.0.0.0.1.0.0.0.0"
        )
    }
}