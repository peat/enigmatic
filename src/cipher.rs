use rand::prelude::SliceRandom;

pub trait DirectedCipher<T> {
    fn encode(&self, input: T) -> Result<T, String>;
    fn decode(&self, input: T) -> Result<T, String>;
}

pub trait SymmetricCipher<T> {
    fn encode(&self, input: T) -> Result<T, String>;
}

#[derive(Clone, Debug)]
pub struct DirectedEncoder<T> {
    pub encoding: Vec<T>,
    pub decoding: Vec<T>,
}

impl DirectedEncoder<usize> {
    pub fn new(values: Vec<usize>) -> Self {
        let mut decoding = vec![0; values.len()];

        let encoding = values.clone();
        for (index, value) in encoding.iter().enumerate() {
            decoding[*value] = index;
        }

        DirectedEncoder { encoding, decoding }
    }

    pub fn ascending(size: usize) -> Self {
        DirectedEncoder::new((0..size).collect())
    }

    pub fn descending(size: usize) -> Self {
        DirectedEncoder::new((0..size).rev().collect())
    }

    pub fn random(size: usize) -> Self {
        let mut values: Vec<usize> = (0..size).collect();
        values.shuffle(&mut rand::thread_rng());
        DirectedEncoder::new(values)
    }

    pub fn len(&self) -> usize {
        self.encoding.len()
    }

    fn find(set: &[usize], value: usize) -> Result<usize, String> {
        match set.get(value) {
            Some(v) => Ok(*v),
            None => Err("DirectedEncoder: value not found".to_owned()),
        }
    }
}

impl DirectedCipher<usize> for DirectedEncoder<usize> {
    fn encode(&self, input: usize) -> Result<usize, String> {
        Self::find(&self.encoding, input)
    }

    fn decode(&self, input: usize) -> Result<usize, String> {
        Self::find(&self.decoding, input)
    }
}

#[derive(Clone, Debug)]
pub struct SymmetricEncoder<T> {
    pub pairs: Vec<Pair<T>>,
}

#[derive(Clone, Debug)]
pub struct Pair<T> {
    pub a: T,
    pub b: T,
}

impl Pair<usize> {
    pub fn contains(&self, target: usize) -> Result<usize, String> {
        if self.a == target {
            return Ok(self.b);
        }

        if self.b == target {
            return Ok(self.a);
        }

        Err("Pair: target not found".to_owned())
    }
}

impl SymmetricCipher<usize> for SymmetricEncoder<usize> {
    fn encode(&self, value: usize) -> Result<usize, String> {
        for p in self.pairs.iter() {
            if let Ok(output) = p.contains(value) {
                return Ok(output);
            }
        }
        Err("SymmetricCipher: value not found".to_owned())
    }
}

#[allow(dead_code)]
impl SymmetricEncoder<usize> {
    pub fn with_pairs(&self, pairs: Vec<Pair<usize>>) -> Result<Self, String> {
        let mut output = self.clone();
        output.pairs = pairs;
        Ok(output)
    }

    pub fn mirror(size: usize) -> Self {
        let pairs = (0..size).map(|v| Pair { a: v, b: v }).collect();
        Self { pairs }
    }

    pub fn flipped(size: usize) -> Self {
        let pairs = (0..size)
            .map(|v| Pair {
                a: v,
                b: (size - v - 1),
            })
            .collect();
        Self { pairs }
    }

    pub fn random(size: usize) -> Self {
        // shuffle ensures there are no repeats
        let mut shuffled: Vec<usize> = (0..size).collect();
        shuffled.shuffle(&mut rand::thread_rng());

        let pairs = (0..size)
            .map(|i| Pair {
                a: i,
                b: shuffled[i],
            })
            .collect();
        Self { pairs }
    }

    pub fn empty() -> Self {
        Self { pairs: vec![] }
    }

    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    pub fn is_paired(&self, value: usize) -> bool {
        for p in self.pairs.iter() {
            if p.contains(value).is_ok() {
                return true;
            }
        }

        false
    }

    pub fn index_of(&self, v: usize) -> Result<usize, String> {
        for (index, pair) in self.pairs.iter().enumerate() {
            if pair.contains(v).is_ok() {
                return Ok(index);
            }
        }
        Err("SymmetricEncoder: value not found".to_owned())
    }

    pub fn pair(&self, a: usize, b: usize) -> Result<Self, String> {
        // guard against existing pairings
        if self.is_paired(a) || self.is_paired(b) {
            return Err("SymmetricEncoder: only one pair per value".to_owned());
        }
        let mut output = self.clone();
        output.pairs.push(Pair { a, b });
        Ok(output)
    }

    pub fn unpair(&self, v: usize) -> Result<Self, String> {
        self.with_pairs(
            self.pairs
                .iter()
                .filter(|p| p.contains(v).is_err()) // remove any pairs that contain v
                .cloned() // copy 'em off
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::cipher::{DirectedCipher, DirectedEncoder};

    #[test]
    fn test_roundtrips() {
        let sub_size = 100;
        let ascending = DirectedEncoder::ascending(sub_size);
        let descending = DirectedEncoder::descending(sub_size);
        let random = DirectedEncoder::random(sub_size);

        assert!(good_directed(&ascending));
        assert!(good_directed(&descending));
        assert!(good_directed(&random));
    }

    fn directed_roundtrip(s: &DirectedCipher<usize>, v: usize) -> bool {
        if let Ok(e) = s.encode(v) {
            // encoded value
            if let Ok(d) = s.decode(e) {
                // decoded value
                return d == v;
            }
        }

        false
    }

    fn good_directed(s: &DirectedEncoder<usize>) -> bool {
        for o in &s.encoding {
            if !directed_roundtrip(s, *o) {
                return false;
            }
        }
        true
    }
}
