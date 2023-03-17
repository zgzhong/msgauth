mod default_resolver;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub use default_resolver::DefaultDNSResolver;

pub enum DNSResolverErrorKind {
    NoConnections,
    NoRecordFound(ResponseCode),
    Timeout,
    Others(String),
}

pub enum ResponseCode {
    NoError,
    FormErr,
    ServFail,
    NXDomain,
    NotImp,
    Refused,
    YXDomain,
    YXRRSet,
    NXRRSet,
    NotAuth,
    NotZone,
    BADVERS,
    BADSIG,
    BADKEY,
    BADTIME,
    BADMODE,
    BADNAME,
    BADALG,
    BADTRUNC,
    BADCOOKIE,
    Unknown(u16),
}


pub type DNSResolverResult<T> = Result<T, DNSResolverErrorKind>;


pub struct  TXTRData {
    txt_data: Box<[String]>
}

pub struct MXRData {
    preference: u16,
    exchange: String,
}


pub trait DNSResolver {
    fn txt_lookup(&self, query: &str) -> DNSResolverResult<Vec<TXTRData>>;
    fn mx_lookup(&self, query: &str) -> DNSResolverResult<Vec<MXRData>>;
    fn ipv4_lookup(&self, query: &str) -> DNSResolverResult<Vec<Ipv4Addr>>;
    fn ipv6_lookup(&self, query: &str) -> DNSResolverResult<Vec<Ipv6Addr>>;
    fn reverse_lookup(&self, query: IpAddr) -> DNSResolverResult<Vec<String>>;
}
