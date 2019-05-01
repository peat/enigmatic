use crate::cipher::*;
use crate::rotor::Rotor;

#[derive(Debug, Clone)]
pub struct RotorSet {
    pub rotors: Vec<Rotor>,
}

impl Iterator for RotorSet {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        // always advance the first rotor
        let mut advance_next = true;
        for r in self.rotors.iter_mut() {
            if advance_next {
                *r = r.next().unwrap();
                advance_next = false;
            }
            if r.position == r.len() - 1 {
                advance_next = true;
            }
        }

        Some(self.clone())
    }
}

impl DirectedCipher<usize> for RotorSet {
    fn encode(&self, value: usize) -> Result<usize, String> {
        let mut new_value = value;
        for r in &self.rotors {
            new_value = match r.encode(new_value) {
                Ok(v) => v,
                Err(e) => return Err(e),
            }
        }

        Ok(new_value)
    }

    fn decode(&self, value: usize) -> Result<usize, String> {
        let mut new_value = value;
        for r in self.rotors.iter().rev() {
            new_value = match r.decode(new_value) {
                Ok(v) => v,
                Err(e) => return Err(e),
            }
        }

        Ok(new_value)
    }
}

impl RotorSet {
    pub fn new() -> Self {
        Self { rotors: vec![] }
    }

    pub fn with_rotor(&self, rotor: Rotor) -> Self {
        let mut rs = self.clone();
        rs.rotors.push(rotor);
        rs
    }

    pub fn with_rotors(rotors: Vec<Rotor>) -> Self {
        Self { rotors }
    }

    pub fn positions(&self) -> Vec<usize> {
        self.rotors.iter().map(|r| r.position).collect()
    }

    pub fn set_positions(&self, positions: &[usize]) -> Result<Self, String> {
        // combine the positions with rotors
        let merged = positions.iter().zip(self.rotors.iter());

        // create a new set of rotors in those positions
        let results: Result<Vec<Rotor>, String> = merged.map(|(p, r)| r.set_position(*p)).collect();

        // woo!
        match results {
            Ok(rotors) => Ok(Self::with_rotors(rotors)),
            Err(e) => Err(e),
        }
    }

    pub fn reset_positions(&self) -> Result<Self, String> {
        let positions = vec![0; self.rotors.len()];
        self.set_positions(&positions)
    }
}
