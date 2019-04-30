use crate::cipher::*;

#[derive(Debug, Clone)]
pub struct Rotor {
    pub values: Substitution<usize>,
    pub position: usize,
}

impl Cipher<usize> for Rotor {
    fn encode(&self, input: usize) -> Result<usize, String> {
        self.values.encode(self.wrap(input + self.position))
    }

    fn decode(&self, input: usize) -> Result<usize, String> {
        match self.values.decode(input) {
            Ok(v) => {
                // pad the value to prevent underflowing
                Ok(self.wrap(self.len() + v - self.position))
            }
            Err(e) => Err(e),
        }
    }
}

impl Iterator for Rotor {
    type Item = Rotor;
    fn next(&mut self) -> Option<Self> {
        match self.set_position(self.position + 1) {
            // advance by one
            Ok(_) => Some(self.clone()),
            Err(_) => None,
        }
    }
}

impl Rotor {
    pub fn ascending(size: usize) -> Self {
        Self::from(Substitution::ascending(size))
    }

    pub fn descending(size: usize) -> Self {
        Self::from(Substitution::descending(size))
    }

    pub fn random(size: usize) -> Self {
        Self::from(Substitution::random(size))
    }

    pub fn from(values: Substitution<usize>) -> Self {
        let position = 0;
        Self { values, position }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn set_position(&mut self, position: usize) -> Result<usize, &str> {
        self.position = self.wrap(position);
        Ok(self.position)
    }

    fn wrap(&self, v: usize) -> usize {
        v % self.values.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::rotor::*;

    #[test]
    fn test_roundtrips() {
        let mut r = Rotor::random(10);

        // round trip test all values
        for i in 0..r.len() {
            assert!(can_roundtrip(&r, i));
        }

        // see how it goes with incrementing the position
        for p in 0..r.len() {
            for i in 0..r.len() {
                r.set_position(p);
                assert!(can_roundtrip(&r, i));
            }
        }
    }

    fn can_roundtrip(r: &Rotor, v: usize) -> bool {
        if let Ok(encoded) = r.encode(v) {
            if let Ok(decoded) = r.decode(encoded) {
                if v == decoded {
                    return true;
                } else {
                    println!(
                        "{:?} FAILED with position {}: {} -> {} -> {}",
                        r, r.position, v, encoded, decoded
                    );
                }
            }
        };
        false
    }
}
