use crate::cipher::*;
use crate::rotor::Rotor;

#[derive(Debug, Clone)]
pub struct RotorSet {
    pub rotors: Vec<Rotor>,
}

impl Iterator for RotorSet {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let mut new_rotors = self.rotors.clone();

        // always advance the first rotor
        let mut advance_next = true;
        for r in &mut new_rotors {
            if advance_next {
                *r = r.next().unwrap();
                advance_next = false;
            }
            if r.position == r.len() - 1 {
                advance_next = true;
            }
        }

        self.rotors = new_rotors;
        Some(self.clone())
    }
}

impl Cipher<usize> for RotorSet {
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

    pub fn positions(&self) -> Vec<usize> {
        self.rotors.iter().map(|r| r.position).collect()
    }

    pub fn set_positions(&mut self, positions: &[usize]) -> Result<bool, &str> {
        if positions.len() != self.rotors.len() {
            return Err("Number of positions doesn't match number of rotors");
        }

        for idx in 0..positions.len() {
            self.rotors[idx].set_position(positions[idx]);
        }

        Ok(true)
    }

    pub fn reset_positions(&self) -> Self {
        let mut rs = self.clone();
        let positions = vec![0; rs.rotors.len()];
        rs.set_positions(&positions);
        rs
    }
}
