#[derive(Clone, Debug)]
pub struct CharacterSet {
    characters: Vec<char>,
}

impl CharacterSet {
    pub fn new(s: &str) -> Self {
        let characters = s.chars().collect();
        Self { characters }
    }

    pub fn len(&self) -> usize {
        self.characters.len()
    }

    pub fn encode(&self, c: char) -> Result<usize, String> {
        match self.characters.iter().position(|&x| x == c) {
            Some(i) => Ok(i),
            None => Err("Dictionary: could not find character".to_owned()),
        }
    }

    pub fn decode(&self, i: usize) -> Result<char, String> {
        if i >= self.characters.len() {
            Err("Dictionary: index out of bounds".to_owned())
        } else {
            Ok(self.characters[i])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_encodes() {
        let sample = "abcd";
        let d = CharacterSet::new(sample);

        let chars: Vec<char> = sample.chars().collect();
        assert_eq!(d.encode(chars[0]), Ok(0));
        assert_eq!(d.encode(chars[2]), Ok(2));
    }

    #[test]
    fn it_decodes() {
        let sample = "abcd";
        let d = CharacterSet::new(sample);

        assert_eq!(d.decode(0), Ok('a'));
        assert_eq!(d.decode(2), Ok('c'));
    }
}
