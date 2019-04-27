mod enigmatic;
mod plugboard;
mod rotor;

use enigmatic::Enigmatic;
use rotor::Rotor;

fn main() {
    let max_values = 10;

    let mut e = Enigmatic::new();
    e.add_rotor(Rotor::random(max_values));
    println!("Rotors: {:?}", e.rotors);
    println!("Plug board: {:?}", e.plugboard);
    println!();
    for x in 0..max_values {
        println!("Encoding {} => {:?}", x, e.encode(x));
    }
}
