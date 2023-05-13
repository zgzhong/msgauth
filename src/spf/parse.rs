use super::SPFRecord;

pub struct SPFParseError {}

const SPFPrefix: &str = "v=spf1";

// pub fn parse_spf_record(txt_rr: &str) -> Result<SPFRecord, SPFParseError> {
//     if !txt_rr.to_lowercase().starts_with(SPFPrefix) {
//         return Err(SPFParseError {});
//     }
//     let txt_rr = &txt_rr[SPFPrefix.len() - 1..];
//     if txt_rr.len() == 0 {
//         return Ok(SPFRecord::new());
//     }

//     if !matches!(&txt_rr[0], '\t' | '\n' | '\x0C' | '\r' | ' ') {
//         return Err(SPFParseError {});
//     }

//     for term in txt_rr.trim_start().split_whitespace() {
//         println!("{}", term)
//     }
// }
