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
    #[allow(dead_code)]
    pub fn mirror(size: usize) -> Self {
        Self {
            encoder: SymmetricEncoder::mirror(size),
        }
    }

    #[allow(dead_code)]
    pub fn flipped(size: usize) -> Self {
        Self {
            encoder: SymmetricEncoder::flipped(size),
        }
    }

    #[allow(dead_code)]
    pub fn random(size: usize) -> Self {
        Self {
            encoder: SymmetricEncoder::random(size),
        }
    }
}
