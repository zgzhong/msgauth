use super::DNSResolver;

pub struct DefaultDNSResolver;

impl DNSResolver for DefaultDNSResolver {
    fn ipv4_lookup(&self, query: &str) -> super::DNSResolverResult<Vec<std::net::Ipv4Addr>> {
        unimplemented!()
    }

    fn ipv6_lookup(&self, query: &str) -> super::DNSResolverResult<Vec<std::net::Ipv6Addr>> {
        unimplemented!()
    }

    fn mx_lookup(&self, query: &str) -> super::DNSResolverResult<Vec<super::MXRData>> {
        unimplemented!()
    }

    fn reverse_lookup(&self, query: std::net::IpAddr) -> super::DNSResolverResult<Vec<String>> {
        unimplemented!()
    }

    fn txt_lookup(&self, query: &str) -> super::DNSResolverResult<Vec<super::TXTRData>> {
        unimplemented!()
    }
}
