use serde_json;
use std::env;

// Available if you need it!
// use serde_bencode

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    // If encoded_value starts with a digit, it's a number
    // we want to confirm if the first character of a string is number
    let is_first_char_digit = match encoded_value.chars().next() {
        Some(c) => c.is_digit(10),
        None => false
    };
    if !is_first_char_digit {
        return serde_json::Value::String(encoded_value.to_string());
    }

    let splitted_encoded_string: (&str, &str) = match encoded_value.split_once(':') {
        Some(splitted_encoded_string) => splitted_encoded_string,
        None => return serde_json::Value::String(encoded_value.to_string())
    };
    return serde_json::Value::String(splitted_encoded_string.1.to_string());
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {

        // You can use print statements as follows for debugging, they'll be visible when running tests.
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", serde_json::to_string(&decoded_value).unwrap());
    } else {
        println!("unknown command: {}", args[1])
    }
}

#[allow(dead_code)]
fn get_length_from_bencoded_string(encoded_string: &str) -> Option<usize> {
    let encoded_string_splitted: Vec<&str> = encoded_string.split(':').collect();
    if encoded_string_splitted.len() != 2 {
        return Option::None;
    }

    let length = match encoded_string_splitted[0].parse::<usize>() {
        Ok(length) => Some(length),
        Err(_) => return None,
    };
    return length;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_length_from_bencoded_string() {
        let encoded_string = "4:spam";
        let length = get_length_from_bencoded_string(encoded_string);
        assert_eq!(Some(4), length);
    }

    #[test]
    fn test_no_length_from_bencoded_string() {
        let encoded_string = "spam";
        let length = get_length_from_bencoded_string(encoded_string);
        assert_eq!(None, length);
    }

    #[test]
    fn test_decode_bencoded_value() {
        let encoded_string = "55:http://bittorrent-test-tracker.codecrafters.io";
        let decoded_value = decode_bencoded_value(encoded_string);
        assert_eq!(serde_json::Value::String("http://bittorrent-test-tracker.codecrafters.io".to_string()), decoded_value);
    }
}