use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct Rotor {
    pub outputs: Vec<usize>,
    pub index: usize,
}

impl Rotor {
    pub fn cleartext(size: usize) -> Self {
        let index = 0;
        let mut outputs = vec![];
        for i in 0..size {
            outputs.push(i);
        }
        return Rotor { outputs, index };
    }

    pub fn random(size: usize) -> Self {
        let index = 0;
        let mut outputs: Vec<usize> = (0..size).collect();
        outputs.shuffle(&mut rand::thread_rng());
        return Rotor { outputs, index };
    }

    pub fn encode(&self, a: usize) -> Result<usize, &str> {
        if a >= self.outputs.len() {
            return Err("Rotor: Out of bounds access");
        }

        return Ok(self.outputs[a]);
    }
}

#[cfg(test)]
mod tests {
    use super::Rotor;

    #[test]
    fn it_encodes() {
        let c = Rotor::cleartext(10);
        assert_eq!(c.encode(5), Ok(5));

        assert!(c.encode(9).is_ok());
        assert!(c.encode(10).is_err());
        assert!(c.encode(11).is_err());
    }
}
