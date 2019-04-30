mod character_set;
mod cipher;
mod machine;
mod plugboard;
mod reflector;
mod rotor;
mod rotor_set;

use character_set::CharacterSet;
use cipher::*;
use machine::*;
use reflector::*;
use rotor::Rotor;

fn encode_str(m: &mut Machine, s: &str) -> Result<String, String> {
    let mut encoded: Vec<char> = vec![];
    for c in s.chars() {
        let next_machine = match m.next() {
            // advance the machine
            None => return Err("Machine iteration failed!".to_owned()),
            Some(n) => n,
        };
        let encoded_char = match next_machine.encode(c) {
            // encode the character
            Ok(o) => o,
            Err(e) => return Err(e),
        };
        encoded.push(encoded_char);
    }

    Ok(encoded.iter().collect())
}

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
        m.plugboard.connections.len()
    );

    println!("Encrypting sample string: {:?}\n", sample);

    let encoded = match encode_str(&mut m, sample) {
        Ok(s) => s,
        Err(e) => {
            println!("ARGH => {}", e);
            std::process::exit(1);
        }
    };

    println!("\nEncrypted string: {:?}", encoded);
    println!("\nresetting rotors and decrypting ... \n");
    m.reset();

    let decoded = match encode_str(&mut m, &encoded) {
        Ok(s) => s,
        Err(e) => {
            println!("ARGH => {}", e);
            std::process::exit(1);
        }
    };

    println!("\nDecrypted: {:?}", decoded);
}
