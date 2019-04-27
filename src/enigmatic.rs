use crate::plugboard::*;
use crate::rotor::*;

pub struct Enigmatic {
    pub rotors: Vec<Rotor>,
    pub plugboard: PlugBoard,
}

impl Enigmatic {
    pub fn new() -> Self {
        Self {
            rotors: vec![],
            plugboard: PlugBoard::new(),
        }
    }

    pub fn add_rotor(&mut self, rotor: Rotor) {
        self.rotors.push(rotor);
    }

    pub fn encode(&self, a: usize) -> Result<usize, &str> {
        // first, pass through the plug board
        let mut value = self.plugboard.encode(a);

        // second, pass through the rotors, lowest to highest
        for idx in 0..self.rotors.len() {
            value = match self.rotors[idx].encode(value) {
                Ok(v) => v,
                e => return e,
            }
        }

        // third, pass back through the rotors highest to lowest
        for idx in (0..self.rotors.len()).rev() {
            value = match self.rotors[idx].encode(value) {
                Ok(v) => v,
                e => return e,
            }
        }

        // fourth, pass back through the plug board
        value = self.plugboard.encode(value);

        // fifth ... output!
        return Ok(value);
    }
}

#[cfg(test)]
mod tests {
    use super::Enigmatic;
    use crate::rotor::Rotor;

    #[test]
    fn identity_encoding() {
        // tests a bare plug board with no rotors
        let mut e = Enigmatic::new();

        assert_eq!(e.encode(0), Ok(0));
        assert_eq!(e.encode(1234567890), Ok(1234567890));

        // tests with cleartext rotors
        e.add_rotor(Rotor::cleartext(10));
        e.add_rotor(Rotor::cleartext(10));
        e.add_rotor(Rotor::cleartext(10));

        assert_eq!(e.encode(0), Ok(0));
        assert_eq!(e.encode(9), Ok(9));
        assert!(e.encode(10).is_err()); // out of range
    }
}
