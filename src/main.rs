mod bencode;

use bencode::Bencode;
use serde_json;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = Bencode::new().decode(encoded_value);
        println!("{}", serde_json::to_string(&decoded_value).unwrap());
    } else {
        println!("unknown command: {}", args[1])
    }
}
