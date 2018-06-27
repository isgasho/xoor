#![feature(plugin)]
#![plugin(xoor)]

fn decrypt(enc: &[u8]) -> Option<String> {
    let decrypted = enc.iter()
        .map(|x| x ^ 127)
        .collect();

    String::from_utf8(decrypted).ok()
}

fn main() {
    let encrypted = xoor!("Hello, world!");
    let decrypted = decrypt(encrypted)
        .unwrap();

    println!("{:?} -> {:?}", encrypted, decrypted);
}
