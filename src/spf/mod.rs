use std::str::FromStr;

mod verify;

pub use verify::SPFVerifier;

pub type SPFRecord = Vec<Term>;


enum Term {
    Directive,
    Modifier,
}

impl FromStr for SPFRecord {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}


impl 