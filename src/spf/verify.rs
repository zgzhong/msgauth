use crate::authres;
use crate::dns_resolver::{DNSResolver, DefaultDNSResolver};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub const DEFAULT_MAX_LOOKUP: u16 = 10;
pub const DEFAULT_MAX_VOID_LOOKUP: u16 = 2;

pub struct SPFVerifier {
    lookup_limit: u16,
    void_lookup_limit: u16,
    resolver: Box<dyn DNSResolver>,
}

impl SPFVerifier {
    pub fn new() -> Self {
        SPFVerifier {
            lookup_limit: DEFAULT_MAX_LOOKUP,
            void_lookup_limit: DEFAULT_MAX_VOID_LOOKUP,
            resolver: Box::new(DefaultDNSResolver),
        }
    }

    pub fn with_lookup_limit(mut self, lookup_limit: u16) -> Self {
        self.lookup_limit = lookup_limit;
        self
    }

    pub fn with_void_lookup_limit(mut self, void_lookup_limit: u16) -> Self {
        self.void_lookup_limit = void_lookup_limit;
        self
    }

    pub fn with_resolver(mut self, resolver: Box<dyn DNSResolver>) -> Self {
        self.resolver = resolver;
        self
    }
}

impl SPFVerifier {
    pub fn verify(&self, ip: IpAddr, helo: &str, sender: &str) -> authres::SPFResult {
        let parts: Vec<_> = sender.splitn(2, '@').collect();
        let domain = match parts.len() {
            2 => parts[1],
            _ => helo,

        };


        match self.resolver.txt_lookup(domain) {
            Err(err) => {}
            Ok(record) => {}
        }


        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_spf_verifier() {
        let ip = Ipv4Addr::new(127, 0, 0, 1);
        let helo = "example.com";
        let sender = "test@example.com";

        let verifier = SPFVerifier::new()
            .with_lookup_limit(50)
            .with_void_lookup_limit(10);

        assert_eq!(verifier.lookup_limit, 50);
        assert_eq!(verifier.void_lookup_limit, 10);
    }
}
