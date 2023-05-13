mod r#macro;
mod mechanism;
mod parse;
mod verify;

use std::net::IpAddr;

pub use verify::SPFVerifier;

use mechanism::Mechanism;

pub struct SPFRecord(Vec<Term>);

pub enum Term {
    Directive(Directive),
    Modifier,
}

pub struct Directive {
    pub qual: Qualifier,
    pub mech: Mechanism,
}

pub enum Qualifier {
    Pass,
    Softfail,
    Neutral,
    Fail,
}

pub struct SPFContext {
    pub ip: IpAddr,
    pub domain: String,
    pub sender: String,
    pub helo: String,
    pub mail_from: String,
    pub host_name: String,
}
