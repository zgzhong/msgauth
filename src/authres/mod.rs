mod spf;

pub use spf::{SPFIdentity, SPFResult};





pub enum ResultValue {
    Unknown,
    None,
    Neutral,
    Fail,
    Softfail,
    Pass,
    Policy,
    Temperror,
    Permerror,
}

pub enum Method {
    ARC,
    Auth,
    DKIM,
    DMARC,
    DKIM_ATPS,
    DNSWL,
    IPRev,
    RRVS,
    SMIME,
    SPF,
    VBR,
}


