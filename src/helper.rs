
pub fn find_char_from_index(input_str: &str, char: char, start_index: usize) -> usize {
    for (i, c) in input_str.chars().skip(start_index).enumerate() {
        if c == char {
            return i;
        }
    }
    return 0;
}
