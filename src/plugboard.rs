use crate::cipher::*;

#[derive(Debug, Clone)]
pub struct PlugBoard {
    pub encoder: SymmetricEncoder<usize>,
}

impl SymmetricCipher<usize> for PlugBoard {
    fn encode(&self, value: usize) -> Result<usize, String> {
        // this particular encoder passes through any values it can't find
        match self.encoder.encode(value) {
            Ok(v) => Ok(v),
            Err(_) => Ok(value),
        }
    }
}

impl PlugBoard {
    pub fn new() -> Self {
        // by default, the plug board doesn't have any connections in it
        Self {
            encoder: SymmetricEncoder::empty(),
        }
    }

    // on success, returns the new connection; on failure,
    // returns the connection that already exists.
    pub fn connect(&self, a: usize, b: usize) -> Result<Self, String> {
        match self.encoder.pair(a, b) {
            Ok(encoder) => {
                let mut output = self.clone();
                output.encoder = encoder;
                Ok(output)
            }
            Err(e) => Err(e),
        }
    }

    pub fn disconnect(&self, a: usize) -> Result<Self, String> {
        match self.encoder.unpair(a) {
            Ok(encoder) => {
                let mut output = self.clone();
                output.encoder = encoder;
                Ok(output)
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_encodes() {
        let mut p = PlugBoard::new();
        assert_eq!(p.encode(5), Ok(5));

        assert!(p.connect(5, 9).is_ok());
        assert_eq!(p.encode(5), Ok(9));
        assert_eq!(p.encode(9), Ok(5));

        assert!(p.connect(2, 9).is_err());
        assert!(p.connect(5, 2).is_err());

        p.disconnect(5);
        assert_eq!(p.encode(5), Ok(5));
        assert_eq!(p.encode(9), Ok(9));
    }
}
