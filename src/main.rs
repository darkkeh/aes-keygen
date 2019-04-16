fn main() {
    use std::io;
    use std::io::prelude::*;
    use base64::{encode};
    use rand::rngs::{OsRng};
    use rand::{RngCore};

    // Get randomness straight from the operating system
    let mut gen = OsRng::new().ok().expect("Failed to get OS random generator");

    // Create array for key
    let mut key = [0; 32];

    // Fill the array with random values
    gen.fill_bytes(&mut key[..]);

    // Encode and print the array
    println!("Key: {}", encode(&key));

    // Wait for user to hit enter before we exit
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press enter to exit...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}
