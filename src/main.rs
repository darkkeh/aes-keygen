fn main() {
    use base64::{encode};
    use rand::rngs::{OsRng};
    use rand::{RngCore};

    // Get randomness straight from the operating system
    let mut gen = OsRng::new().ok().expect("Failed to get OS random generator");

    // Create vec for key and iv containing only 0s until a certain length
    let mut key = vec![0; 32];
    let mut iv = vec![0; 16];

    // Fill vecs with random values
    gen.fill_bytes(key.as_mut_slice());
    gen.fill_bytes(iv.as_mut_slice());

    // Encode and print the vecs
    println!("Key: {}", encode(&key));
    println!("IV: {}", encode(&iv));
}
