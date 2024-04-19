use rand::seq::SliceRandom;

const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const SYMBOLS: &[u8] = b"!@#$%^&*?";
const NUMBERS: &[u8] = b"0123456789";

pub fn genpass(length: u8, uppercase: bool, symbol: bool, number: bool) -> String {
    let mut rng = rand::thread_rng();
    let mut password = Vec::with_capacity(length as usize);
    let mut charset = Vec::from(LOWERCASE);

    if uppercase {
        charset.extend_from_slice(UPPERCASE);
        password.push(*UPPERCASE.choose(&mut rng).expect("charset wont be empty"));
    }
    if symbol {
        charset.extend_from_slice(SYMBOLS);
        password.push(*SYMBOLS.choose(&mut rng).expect("charset wont be empty"));
    }
    if number {
        charset.extend_from_slice(NUMBERS);
        password.push(*NUMBERS.choose(&mut rng).expect("charset wont be empty"));
    }

    for _ in 0..(length - password.len() as u8) {
        password.push(*charset.choose(&mut rng).expect("charset wont be empty"));
    }

    password.shuffle(&mut rng);

    String::from_utf8(password).unwrap()
}
