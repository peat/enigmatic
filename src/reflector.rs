use crate::cipher::*;

#[derive(Clone, Debug)]
pub struct Reflector {
    encoder: SymmetricEncoder<usize>,
}

impl SymmetricCipher<usize> for Reflector {
    fn encode(&self, value: usize) -> Result<usize, String> {
        self.encoder.encode(value)
    }
}

impl Reflector {
    pub fn mirror(size: usize) -> Self {
        Self {
            encoder: SymmetricEncoder::mirror(size),
        }
    }

    pub fn flipped(size: usize) -> Self {
        Self {
            encoder: SymmetricEncoder::flipped(size),
        }
    }

    pub fn random(size: usize) -> Self {
        Self {
            encoder: SymmetricEncoder::random(size),
        }
    }
}
