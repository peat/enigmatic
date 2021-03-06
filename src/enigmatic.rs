use crate::cipher::*;
use crate::machine::*;

pub struct Enigmatic {}

impl Enigmatic {
    pub fn encode_str(m: &mut Machine, input: &str) -> Result<String, String> {
        input.chars().map(|c| m.next().unwrap().encode(c)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::character_set::*;
    use crate::reflector::*;
    use crate::rotor::*;

    #[test]
    fn test_roundtrips() {
        let char_set = CharacterSet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ");

        let mut m = Machine::new(char_set)
            .with_rotor(Rotor::random)
            .with_rotor(Rotor::ascending)
            .with_rotor(Rotor::descending)
            .with_reflector(Reflector::flipped);

        assert!(can_roundtrip(&mut m, "YOLO"));
    }

    fn can_roundtrip(m: &mut Machine, input: &str) -> bool {
        let encrypted = Enigmatic::encode_str(m, input).unwrap();
        let _ = m.reset();
        let decrypted = Enigmatic::encode_str(m, &encrypted).unwrap();
        input == &decrypted
    }
}
