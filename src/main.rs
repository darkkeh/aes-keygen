fn main() {
    use std::io;
    use std::io::prelude::*;
    use base64::{encode};
    use rand::rngs::{OsRng};
    use rand::{RngCore};

    // Get randomness straight from the operating system
    let mut gen = OsRng::new().ok().expect("Failed to get OS random generator");

    // Create arrays for key and iv
    let mut key = [0; 32];
    let mut iv = [0; 16];

    // Fill arrays with random values
    gen.fill_bytes(&mut key[..]);
    gen.fill_bytes(&mut iv[..]);

    // Encode and print the arrays
    println!("Key: {}", encode(&key));
    println!("IV: {}", encode(&iv));

    // Wait for user to hit enter before we exit
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press enter to exit...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}
