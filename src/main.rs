use uuid::Uuid;
use base64::{encode, decode};
use nanoid::{nanoid, alphabet};

fn main() {
    let uuid = Uuid::new_v4();
    println!("uuid:   {}", uuid);
    let uuid_bytes = uuid.to_u128_le().to_ne_bytes();
    println!("bytes:  {:?}", uuid_bytes);
    let b64 = encode(uuid_bytes);
    println!("base64: {}", b64);
    println!("base64 len: {}", b64.len());
    println!(" ... Reconverting ...");
    let reconv_bytes = decode(b64).unwrap();
    println!("bytes:  {:?}", reconv_bytes);
    let reconv_uuid = Uuid::from_slice(&reconv_bytes).expect("cannot reconvert");
    println!("uuid:   {}", reconv_uuid);

    println!("-------");
    let nano = nanoid!();
    println!("nanoid: {}", nano);
    println!("nanoid len: {}  {}", nano.len(), alphabet::SAFE.len());

    let bit_per_char = (alphabet::SAFE.len() as f64).log2();
    let total_bits = (nano.len() as f64) * bit_per_char;
    println!("nanoid total bits: {}", total_bits);

}
