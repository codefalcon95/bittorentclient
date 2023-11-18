use serde_json;
use std::env;

enum BencodeType {
    Integer,
    String,
}

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    let bencode_type = encoded_string_type(encoded_value);
    match bencode_type {
        BencodeType::Integer => decode_bencoded_integer(encoded_value),
        BencodeType::String => decode_bencoded_string(encoded_value),
    }
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", serde_json::to_string(&decoded_value).unwrap());
    } else {
        println!("unknown command: {}", args[1])
    }
}

fn encoded_string_type(encoded_string: &str) -> BencodeType {
    let is_bencoded_integer = encoded_string_is_bencoded_integer(encoded_string);
    if is_bencoded_integer {
        return BencodeType::Integer;
    }
    let is_bencoded_string = encoded_string_is_bencoded_string(encoded_string);
    if is_bencoded_string {
        return BencodeType::String;
    }
    return BencodeType::String;
}

fn encoded_string_is_bencoded_string(encoded_string: &str) -> bool {
    let is_first_char_digit = match encoded_string.chars().next() {
        Some(c) => c.is_digit(10),
        None => false,
    };
    return is_first_char_digit;
}

fn encoded_string_is_bencoded_integer(encoded_string: &str) -> bool {
    let is_first_char_digit = encoded_string.chars().next() == Some('i');
    let is_last_char_letter_e = encoded_string.chars().last() == Some('e');
    let character_length = encoded_string.chars().count() - 1;
    let is_index_string_digit = encoded_string[1..character_length].parse::<i64>().is_ok();
    return is_first_char_digit && is_last_char_letter_e && is_index_string_digit;
}

fn decode_bencoded_string(encoded_string: &str) -> serde_json::Value {
    let splitted_encoded_string: (&str, &str) = match encoded_string.split_once(':') {
        Some(splitted_encoded_string) => splitted_encoded_string,
        None => return serde_json::Value::String(encoded_string.to_string()),
    };
    return serde_json::Value::String(splitted_encoded_string.1.to_string());
}

fn decode_bencoded_integer(encoded_string: &str) -> serde_json::Value {
    let encoded_integer = encoded_string[1..(encoded_string.len() - 1)]
        .parse::<i64>()
        .unwrap();
    return serde_json::Value::Number(serde_json::Number::from(encoded_integer));
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
        assert_eq!(
            serde_json::Value::String("http://bittorrent-test-tracker.codecrafters.io".to_string()),
            decoded_value
        );
    }
}
