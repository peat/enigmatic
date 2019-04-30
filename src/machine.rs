use crate::character_set::*;
use crate::cipher::*;
use crate::plugboard::*;
use crate::reflector::Reflector;
use crate::rotor::*;
use crate::rotor_set::*;

#[derive(Clone, Debug)]
pub struct Machine {
    pub character_set: CharacterSet,
    pub rotor_set: RotorSet,
    pub reflector: Reflector,
    pub plugboard: PlugBoard,
}

impl Iterator for Machine {
    type Item = Machine;
    fn next(&mut self) -> Option<Machine> {
        let new_rs = match self.rotor_set.next() {
            Some(rs) => rs,
            None => return None,
        };

        self.rotor_set = new_rs;
        Some(self.clone())
    }
}

impl DirectedCipher<char> for Machine {
    fn encode(&self, input: char) -> Result<char, String> {
        self.character_set
            .encode(input)
            .and_then(|i| self.encode_usize(i))
            .and_then(|e| self.character_set.decode(e))
    }

    fn decode(&self, value: char) -> Result<char, String> {
        self.encode(value)
    }
}

impl Machine {
    pub fn new(character_set: CharacterSet) -> Self {
        let set_len = character_set.len();
        Machine {
            character_set,
            rotor_set: RotorSet::new(),
            reflector: Reflector::flipped(set_len), // default
            plugboard: PlugBoard::new(),
        }
    }

    pub fn with_reflector(&self, builder: fn(usize) -> Reflector) -> Self {
        let mut new_machine = self.clone();
        new_machine.reflector = builder(self.character_set.len());
        new_machine
    }

    pub fn with_rotor(&self, builder: fn(usize) -> Rotor) -> Self {
        let mut new_machine = self.clone();
        let rotor = builder(self.character_set.len());
        new_machine.rotor_set = self.rotor_set.with_rotor(rotor);
        new_machine
    }

    pub fn with_rotor_set(&self, rotor_set: RotorSet) -> Self {
        let mut m = self.clone();
        m.rotor_set = rotor_set;
        m
    }

    pub fn reset(&self) -> Result<Self, String> {
        match self.rotor_set.reset_positions() {
            Ok(rs) => Ok(self.with_rotor_set(rs)),
            Err(e) => Err(e),
        }
    }

    fn encode_usize(&self, input: usize) -> Result<usize, String> {
        self.plugboard
            // first, pass through the plug board
            .encode(input)
            // second, pass through the encoding rotors in order
            .and_then(|v| self.rotor_set.encode(v))
            // third, hit the reflector
            .and_then(|v| self.reflector.encode(v))
            // fourth, pass back through the rotors in reverse order
            .and_then(|v| self.rotor_set.decode(v))
            // finally, pass back through the plug board
            .and_then(|v| self.plugboard.encode(v))
    }
}
