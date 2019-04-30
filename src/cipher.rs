use rand::prelude::*;

pub trait Cipher<T> {
    fn encode(&self, input: T) -> Result<T, String>;
    fn decode(&self, input: T) -> Result<T, String>;
}

#[derive(Clone, Debug)]
pub struct Substitution<T> {
    pub encoding: Vec<T>,
    pub decoding: Vec<T>,
}

impl Substitution<usize> {
    pub fn new(values: Vec<usize>) -> Self {
        // encoding is encoding[i] => v
        // decoding is decoding[v] => i
        let encoding = values.clone();
        let mut decoding = vec![0; values.len()];
        for i in 0..encoding.len() {
            let v = encoding[i];
            decoding[v] = i;
        }

        Self { encoding, decoding }
    }

    pub fn ascending(size: usize) -> Self {
        Substitution::new((0..size).collect())
    }

    pub fn descending(size: usize) -> Self {
        Substitution::new((0..size).rev().collect())
    }

    pub fn random(size: usize) -> Self {
        let mut values: Vec<usize> = (0..size).collect();
        values.shuffle(&mut rand::thread_rng());
        Substitution::new(values)
    }

    pub fn len(&self) -> usize {
        self.encoding.len()
    }

    fn find(set: &[usize], value: usize) -> Result<usize, String> {
        match value >= set.len() {
            true => Err("Substitution: value out of bounds".to_owned()),
            false => Ok(set[value]),
        }
    }
}

impl Cipher<usize> for Substitution<usize> {
    fn encode(&self, input: usize) -> Result<usize, String> {
        Self::find(&self.encoding, input)
    }

    fn decode(&self, input: usize) -> Result<usize, String> {
        Self::find(&self.decoding, input)
    }
}

#[cfg(test)]
mod tests {
    use crate::cipher::{Cipher, Substitution};

    #[test]
    fn test_roundtrips() {
        let sub_size = 100;
        let ascending = Substitution::ascending(sub_size);
        let descending = Substitution::descending(sub_size);
        let random = Substitution::random(sub_size);

        assert!(is_symmetrical(&ascending));
        assert!(is_symmetrical(&descending));
        assert!(is_symmetrical(&random));
    }

    fn can_roundtrip(s: &Cipher<usize>, v: usize) -> bool {
        if let Ok(e) = s.encode(v) {
            // encoded value
            if let Ok(d) = s.decode(e) {
                // decoded value
                return d == v;
            }
        }

        false
    }

    fn is_symmetrical(s: &Substitution<usize>) -> bool {
        for o in &s.encoding {
            if !can_roundtrip(s, *o) {
                return false;
            }
        }
        true
    }
}
