use regex::Regex;
use serde_json;

enum BencodeType {
    Integer,
    String,
}

pub struct Bencode;

impl Bencode {
    pub fn new() -> Bencode {
        Bencode
    }

    pub fn decode(&self, encoded_value: &str) -> serde_json::Value {
        let bencode_type = self.encoded_string_type(encoded_value);
        match bencode_type {
            BencodeType::Integer => self.decode_integer(encoded_value),
            BencodeType::String => self.decode_string(encoded_value),
        }
    }

    fn encoded_string_type(&self, encoded_string: &str) -> BencodeType {
        if self.is_encoded_integer(encoded_string) {
            return BencodeType::Integer;
        }
        if &self.is_encoded_string(encoded_string) == &true {
            return BencodeType::String;
        }
        return BencodeType::String;
    }

    fn is_encoded_string(&self, encoded_string: &str) -> bool {
        let re = Regex::new(r"^\d+:[a-zA-Z]+$").unwrap();
        return re.is_match(encoded_string);
    }

    fn is_encoded_integer(&self, encoded_string: &str) -> bool {
        let re = Regex::new(r"^i\d+e$").unwrap();
        return re.is_match(encoded_string);
    }

    fn decode_string(&self, encoded_string: &str) -> serde_json::Value {
        let splitted_encoded_string: (&str, &str) = match encoded_string.split_once(':') {
            Some(splitted_encoded_string) => splitted_encoded_string,
            None => return serde_json::Value::String(encoded_string.to_string()),
        };
        return serde_json::Value::String(splitted_encoded_string.1.to_string());
    }

    fn decode_integer(&self, encoded_string: &str) -> serde_json::Value {
        let encoded_integer = encoded_string[1..(encoded_string.len() - 1)]
            .parse::<i64>()
            .unwrap();
        return serde_json::Value::Number(serde_json::Number::from(encoded_integer));
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
}
