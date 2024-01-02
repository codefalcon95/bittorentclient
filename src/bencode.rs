use regex::Regex;
use serde_json;

enum BencodeType {
    Integer,
    String,
    List,
}

pub struct Bencode;

impl Bencode {
    pub fn new() -> Bencode {
        Bencode
    }

    #[allow(dead_code)]
    /// Decode a bencode string into a json string
    pub fn decode(&self, encoded_value: &str) -> serde_json::Value {
        let bencode_type: BencodeType = self.encoded_string_type(encoded_value);
        match bencode_type {
            BencodeType::Integer => self.decode_integer(encoded_value),
            BencodeType::String => self.decode_string(encoded_value),
            BencodeType::List => self.decode_list(encoded_value),
            _ => serde_json::Value::String(encoded_value.to_string()),
        }
    }
    
    /// get encoded string type
    fn encoded_string_type(&self, encoded_string: &str) -> BencodeType {
        if self.is_encoded_integer(encoded_string) {
            return BencodeType::Integer;
        }
        if self.is_encoded_string(encoded_string) {
            return BencodeType::String;
        }
        if self.is_encoded_list(encoded_string) {
            return BencodeType::List;
        }
        return BencodeType::String;
    }

    /// check if encoded string is type string
    fn is_encoded_string(&self, encoded_string: &str) -> bool {
        let re = Regex::new(r"^\d+:[a-zA-Z]+$").unwrap();
        return re.is_match(encoded_string);
    }

    /// check if encoded string is type integer
    fn is_encoded_integer(&self, encoded_string: &str) -> bool {
        let re = Regex::new(r"^i\d+e$").unwrap();
        return re.is_match(encoded_string);
    }

    /// decode encoded string as a string
    fn decode_string(&self, encoded_string: &str) -> serde_json::Value {
        match encoded_string.split_once(':') {
            Some((_, splitted_encoded_string)) => serde_json::Value::String(splitted_encoded_string.to_string()),
            None => serde_json::Value::String(encoded_string.to_string())
        }
    }

    /// decode encoded string as an integer
    fn decode_integer(&self, encoded_string: &str) -> serde_json::Value {
        let encoded_integer = &encoded_string[1..(encoded_string.len() - 1)];
        let parsed_integer = encoded_integer.parse::<i64>()
        .unwrap();
        return serde_json::Value::Number(serde_json::Number::from(parsed_integer));
    }

    /// check if encoded string is type list
    pub fn is_encoded_list(&self, encoded_string: &str) -> bool {
        let re = Regex::new(r"^l.*e$").unwrap();
        return re.is_match(encoded_string);
    }

    /// decode encoded string as a list
    pub fn decode_list(&self, encoded_string: &str) -> serde_json::Value {
        let mut splitted_encoded_string = &encoded_string[1..];
        let mut list = Vec::new();

        while !splitted_encoded_string.is_empty() {
            if splitted_encoded_string.starts_with('i') {
                if let Some((int_str, rest)) = splitted_encoded_string[1..].split_once('e') {
                    list.push(serde_json::Value::Number(int_str.parse::<i64>().unwrap().into()));
                    splitted_encoded_string = rest;
                } else {
                    break;
                }
            }
            else if let Some((length_str, rest)) = splitted_encoded_string.split_once(':') {
                let string_length = length_str.parse::<usize>().unwrap();
                let (string_slice, remainder_string) = rest.split_at(string_length);
                list.push(serde_json::Value::String(string_slice.to_string()));
                splitted_encoded_string = remainder_string;
            }else {
                break;
            }
        }

        serde_json::Value::Array(list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_string() {
        let encoded_string = "55:http://bittorrent-test-tracker.codecrafters.io";
        let decoded_value = Bencode::new().decode(encoded_string);
        assert_eq!(
            serde_json::Value::String("http://bittorrent-test-tracker.codecrafters.io".to_string()),
            decoded_value
        );
    }

    #[test]
    fn test_is_encoded_list() {
        let encoded_string = "l5:helloi52ee";
        let bencode = Bencode::new();
        assert_eq!(bencode.is_encoded_list(encoded_string), true);
    }

    #[test]
    fn test_decode_list() {
        let encoded_string = "l5:helloi76e10:lifeisgood6:worldyi153e";
        let bencode = Bencode::new();
        let resp = bencode.decode_list(encoded_string);
        assert_eq!(serde_json::json!(["hello", 76, "lifeisgood", "worldy", 153]), resp);
    }
}
