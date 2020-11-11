
// Type used for ids
pub type IDType = usize;

// Dictionary of characters
pub const CHAR_SET: &'static str = "_ abcdefghijklmnopqrstuvwxyz,.\n";

// Length of Dictionary
pub const LENGTH: usize = CHAR_SET.len() as usize;

// Bits a dictionary ID takes up
pub fn length_bits() -> usize {
    LENGTH.next_power_of_two().trailing_zeros() as usize
}

// Maximum amount of ID that can fit in a IDType
pub fn max_str_size() -> usize {
    const MAX_BITS: usize = (0 as IDType).count_zeros() as usize;
    MAX_BITS / length_bits()
}

// Get letter from id
pub fn get_char(id: IDType) -> char {
    CHAR_SET.chars().nth(id).unwrap()
}

// Get id from letter
pub fn get_id(l: char) -> usize {
    CHAR_SET.find(l.to_ascii_lowercase()).unwrap_or(0)
}

pub fn get_str_id(word: &str) -> IDType {
    let mut id = 0 as IDType;

    for l in word.chars() {
        id <<= length_bits();
        id |= get_id(l);
    }

    id
}