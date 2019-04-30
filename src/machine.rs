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

impl Cipher<char> for Machine {
    fn encode(&self, input: char) -> Result<char, String> {
        let i: usize = match self.character_set.encode(input) {
            Ok(c) => c,
            Err(e) => return Err(e),
        };

        let e: usize = match self.process(i) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        self.character_set.decode(e)
    }

    fn decode(&self, value: char) -> Result<char, String> {
        self.encode(value)
    }
}

impl Machine {
    pub fn new(character_set: CharacterSet) -> Self {
        let set_len = character_set.len();
        Machine {
            character_set: character_set,
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

    pub fn reset(&mut self) {
        self.rotor_set = self.rotor_set.reset_positions();
    }

    fn process(&self, input: usize) -> Result<usize, String> {
        let mut value = input;
        print!("{} -> ", value);

        // first, pass through the plug board
        value = self.plugboard.encode(value);
        print!("PB -> {} -> ", value);

        // second, pass through the encoding rotors in order
        value = match self.rotor_set.encode(value) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        print!("RE -> {} -> ", value);

        // third, hit the reflector
        value = match self.reflector.encode(value) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        print!("X -> {} -> ", value);

        // fourth, pass back through the rotors in reverse order
        value = match self.rotor_set.decode(value) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        print!("RD -> {} -> ", value);

        // finally, pass back through the plug board
        value = self.plugboard.encode(value);
        println!("PB -> {}", value);

        Ok(value)
    }
}
