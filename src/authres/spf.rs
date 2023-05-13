use super::ResultValue;
use std::net::IpAddr;

pub enum SPFIdentity {
    Helo,
    MailFrom,
}

pub struct SPFResult {
    pub client_ip: IpAddr,
    pub helo: String,
    pub sender: String,
    pub value: ResultValue,
    pub identity: SPFIdentity,
}
