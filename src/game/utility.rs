pub fn string_to_square(s: &str) -> Option<(u32, u32)> {
    // Check if the input string has exactly 2 characters
    if s.len() != 2 {
        return None;
    }

    // Extract the characters from the string
    let chars: Vec<char> = s.chars().collect();
    let first_char = chars[0];
    let second_char = chars[1];

    // Convert the first character to a zero-based index (assuming it is a letter)
    let alphabet_index = match first_char {
        'a'..='z' => (first_char as u32) - ('a' as u32),
        _ => return None, // Return None if the first character is not a letter
    };

    let digit_value = second_char.to_digit(10).map(|d| 8 - d);

    // Return None if the second character is not a digit
    if let Some(d) = digit_value {
        Some((d, alphabet_index))
    } else {
        None
    }
}

pub fn square_to_index(row: u32, col: u32) -> u32 {
    return (row << 3) + col;
}

pub fn index_to_square(index: u32) -> (u32, u32) {
    let row = index >> 3;
    let col = index & 0b111;
    return (row, col);
}

pub fn square_to_string(row: u32, col: u32) -> String {
    let str_col = std::char::from_u32(col + ('a' as u32)).unwrap();
    return format!("{}{}", str_col, 8 - row);
}

pub fn extract_bit(bits: u64, index: u8) -> u64 {
    return (bits >> index) & 0b1;
}

pub fn west_one(bits: u64) -> u64 {
    return bits >> 1;
}

pub fn east_one(bits: u64) -> u64 {
    return bits << 1;
}
