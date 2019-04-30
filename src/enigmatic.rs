use crate::cipher::*;
use crate::machine::*;

pub struct Enigmatic {}

impl Enigmatic {
    pub fn encode_str(m: &mut Machine, input: &str) -> Result<String, String> {
        input
            .chars()
            .map(|c| match m.next() {
                None => Err("Machine iteration failed!".to_owned()),
                Some(n) => n.encode(c),
            })
            .collect()
    }
}
