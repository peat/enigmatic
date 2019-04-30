#[derive(Debug, Clone)]
pub struct PlugBoard {
    pub connections: Vec<Connection>,
}

impl PlugBoard {
    pub fn new() -> Self {
        Self {
            connections: vec![],
        }
    }

    pub fn encode(&self, a: usize) -> usize {
        for c in &self.connections {
            if let Some(good) = c.connected_to(a) {
                return good;
            }
        }
        return a;
    }

    // on success, returns the new connection; on failure,
    // returns the connection that already exists.
    pub fn connect(&mut self, a: usize, b: usize) -> Result<Connection, String> {
        if self.is_connected(a) || self.is_connected(b) {
            return Err("PlugBoard: already connected.".to_owned());
        }
        let c = Connection { a, b };
        self.connections.push(c);
        return Ok(c);
    }

    pub fn disconnect(&mut self, a: usize) {
        self.connections.retain(|&c| c.connected_to(a).is_none());
    }

    pub fn is_connected(&self, a: usize) -> bool {
        for c in &self.connections {
            if c.connected_to(a).is_some() {
                return true;
            }
        }
        return false;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Connection {
    pub a: usize,
    pub b: usize,
}

impl Connection {
    pub fn connected_to(&self, x: usize) -> Option<usize> {
        if self.a == x {
            return Some(self.b);
        }

        if self.b == x {
            return Some(self.a);
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_encodes() {
        let mut p = PlugBoard::new();
        assert_eq!(p.encode(5), 5);

        assert!(p.connect(5, 9).is_ok());
        assert_eq!(p.encode(5), 9);
        assert_eq!(p.encode(9), 5);

        assert!(p.connect(2, 9).is_err());
        assert!(p.connect(5, 2).is_err());

        p.disconnect(5);
        assert_eq!(p.encode(5), 5);
        assert_eq!(p.encode(9), 9);
    }
}
