mod character_set;
mod cipher;
mod enigmatic;
mod machine;
mod plugboard;
mod reflector;
mod rotor;
mod rotor_set;

use character_set::CharacterSet;
use enigmatic::Enigmatic;
use machine::*;
use reflector::*;
use rotor::Rotor;

fn main() {
    let chars = CharacterSet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let sample = "HELLOWORLD";

    let mut m = Machine::new(chars)
        .with_rotor(Rotor::random)
        .with_rotor(Rotor::ascending)
        .with_rotor(Rotor::descending)
        .with_reflector(Reflector::flipped);

    println!(
        "Machine:\n  {} rotors (random, ascending, descending)\n  flipped reflector\n  {} connected plugs\n",
        m.rotor_set.rotors.len(),
        m.plugboard.encoder.len()
    );

    print!("{:?} -> ", sample);

    let encoded = match Enigmatic::encode_str(&mut m, sample) {
        Ok(s) => s,
        Err(e) => panic!("ARGH => {}", e),
    };

    print!("{:?} -> ", encoded);

    let decoded = match Enigmatic::encode_str(&mut m, &encoded) {
        Ok(s) => s,
        Err(e) => panic!("ARGH => {}", e),
    };

    println!("{:?}", decoded);
}
