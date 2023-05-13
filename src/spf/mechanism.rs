use super::r#macro::DomainSpec;
use std::net::{Ipv4Addr, Ipv6Addr};

pub enum Mechanism {
    All(All),
    Include(Include),
    A(A),
    MX(MX),
    PTR(PTR),
    IP4(IP4),
    IP6(IP6),
    Exists(Exists),
}

pub struct All;

pub struct Include {
    domain_spec: DomainSpec,
}

pub struct A {
    domain_spec: Option<DomainSpec>,
    dual_cidr_len: Option<DualCidrLength>,
}

pub struct MX {
    domain_spec: Option<DomainSpec>,
    dual_cidr_len: Option<DualCidrLength>,
}

pub struct PTR {
    domain_spec: Option<DomainSpec>,
}

pub struct Exists {
    domain_spec: DomainSpec,
}

pub struct IP4 {
    addr: Ipv4Addr,
    cidr_length: Option<IPv4CidrLength>,
}

pub struct IP6 {
    addr: Ipv6Addr,
    cidr_length: Option<IPv6CidrLength>,
}

struct IPv4CidrLength(u8);
struct IPv6CidrLength(u8);
enum DualCidrLength {
    IP4(IPv4CidrLength),
    IP6(IPv6CidrLength),
    Both(IPv4CidrLength, IPv6CidrLength),
}
