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
    pub fn decode(&self, encoded_value: &str) -> serde_json::Value {
        let bencode_type: BencodeType = self.encoded_string_type(encoded_value);
        match bencode_type {
            BencodeType::Integer => self.decode_integer(encoded_value),
            BencodeType::String => self.decode_string(encoded_value),
            BencodeType::List => self.decode_list(encoded_value),
            _ => serde_json::Value::String(encoded_value.to_string()),
        }
    }

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

    fn is_encoded_string(&self, encoded_string: &str) -> bool {
        let re = Regex::new(r"^\d+:[a-zA-Z]+$").unwrap();
        return re.is_match(encoded_string);
    }

    fn is_encoded_integer(&self, encoded_string: &str) -> bool {
        let re = Regex::new(r"^i\d+e$").unwrap();
        return re.is_match(encoded_string);
    }


    fn decode_string(&self, encoded_string: &str) -> serde_json::Value {
        match encoded_string.split_once(':') {
            Some((_, splitted_encoded_string)) => serde_json::Value::String(splitted_encoded_string.to_string()),
            None => serde_json::Value::String(encoded_string.to_string())
        }
    }

    fn decode_integer(&self, encoded_string: &str) -> serde_json::Value {
        let encoded_integer = &encoded_string[1..(encoded_string.len() - 1)];
        let parsed_integer = encoded_integer.parse::<i64>()
        .unwrap();
        return serde_json::Value::Number(serde_json::Number::from(parsed_integer));
    }

    pub fn is_encoded_list(&self, encoded_string: &str) -> bool {
        let re = Regex::new(r"^l.*e$").unwrap();
        return re.is_match(encoded_string);
    }

    pub fn decode_list(&self, encoded_string: &str) -> serde_json::Value {
        let mut splitted_encoded_string: &str = &encoded_string[1..];
        let mut previous_char = ' ';
        let mut index: usize = 0;
        let mut chars: Vec<char> = splitted_encoded_string.chars().collect();
        let mut list = Vec::new();

        while index < chars.len() {
            // che
            if chars[index] == ':' && previous_char.is_digit(10) {
                 // 
                let (string_length_, split_string_by_column)= splitted_encoded_string.split_once(':').unwrap();
                // cant character to the front
                let string_length = string_length_.to_string().parse::<usize>().unwrap();
                let string_slice: &str = &split_string_by_column[0..string_length];
            
                let (_, remainder_string) =
                    splitted_encoded_string.split_once(string_slice).unwrap();
                splitted_encoded_string = remainder_string;
                chars = splitted_encoded_string.chars().collect();
                list.push(string_slice);
                index = 0;
                continue;
            }

            if previous_char == 'i' && chars[index].is_digit(10) {
                // scan forward to find e
                let mut i_index = 1;
                let integer_find_string: &str = &splitted_encoded_string[index..];
                let inner_char: Vec<char> = integer_find_string.chars().collect();

                while i_index < inner_char.len() {
                    if inner_char[i_index] == 'e' {
                        let string_slice = &splitted_encoded_string[index..(index + i_index)];
                        let (_, remainder_string) =
                        splitted_encoded_string.split_once((string_slice.to_string() +"e").as_str()).unwrap();
                        splitted_encoded_string = remainder_string;
                        chars = splitted_encoded_string.chars().collect();
                        list.push(string_slice);
                        index = 0;
                        break;
                    }
                    // check for e
                    if !inner_char[i_index].is_digit(10) {
                        break;
                    }

                    i_index += 1;
                }
            }

            if chars.len() > 0 {
                previous_char = chars[index];
                index += 1;
            }
           
        }
       return serde_json::to_string(&list).unwrap().into();
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
        println!("{:?}", resp);
        assert!(true);
    }
}
