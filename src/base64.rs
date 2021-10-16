const INT_TO_CHAR_MAP: [&str; 64] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l",
    "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "0", "1", "2", "3", "4",
    "5", "6", "7", "8", "9", "+", "/",
];

pub fn encode(num: i32) -> Option<char> {
    if 0 <= num && num < INT_TO_CHAR_MAP.len() as i32 {
        Some(INT_TO_CHAR_MAP[num as usize].parse().unwrap())
    } else {
        None
    }
}

#[test]
fn test_encode() {
    assert_eq!(encode(0), Some('A'));
    assert_eq!(encode(26), Some('a'));
    assert_eq!(encode(64), None);
}
