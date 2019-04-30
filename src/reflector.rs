use crate::cipher::*;

#[derive(Clone, Debug)]
pub struct Reflector {
    cipher: Substitution<usize>,
}

impl Cipher<usize> for Reflector {
    fn encode(&self, value: usize) -> Result<usize, String> {
        self.cipher.encode(value)
    }

    fn decode(&self, value: usize) -> Result<usize, String> {
        self.cipher.decode(value)
    }
}

impl Reflector {
    pub fn mirror(size: usize) -> Self {
        Self {
            cipher: Substitution::ascending(size),
        }
    }

    pub fn flipped(size: usize) -> Self {
        Self {
            cipher: Substitution::new((0..size).rev().collect()),
        }
    }
}
