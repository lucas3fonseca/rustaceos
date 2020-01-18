#![feature(const_fn, const_if_match, const_loop, const_panic, const_mut_refs)]

use super::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Name {
    pub value: u64,
}

impl Name {
    pub fn new(value: u64) -> Self {
        Name { value }
    }

    pub fn from(name_str: &str) -> Name {
        name_from_str(name_str)
    }

    pub fn to_string() -> String {
        String::from("hello")
    }
}

const fn min(l: usize, r: usize) -> usize {
    if l < r { l } else { r }
}

/// EOS Account Name from String
///
/// # Examples
///
/// ```
/// use eosio_cdt::eos::Name;
///
/// let name = Name::from("").unwrap();
/// assert_eq!(name.value, 0);
///
/// let name = Name::from("abc").unwrap();
/// assert_eq!(name.value, 3589368903014285312);
///
/// let name = Name::from("123").unwrap();
/// assert_eq!(name.value, 614178399182651392);
///
/// let name = Name::from("123.........").unwrap();
/// assert_eq!(name.value, 614178399182651392);
///
/// let name = Name::from(".a.b.c.1.2.3.").unwrap();
/// assert_eq!(name.value, 108209673814966320);
///
/// let name = Name::from("abc.123").unwrap();
/// assert_eq!(name.value, 3589369488740450304);
///
/// let name = Name::from("12345abcdefgj").unwrap();
/// assert_eq!(name.value, 614251623682315983);
///
/// let name = Name::from("hijklmnopqrsj").unwrap();
/// assert_eq!(name.value, 7754926748989239183);
///
/// let name = Name::from("1").unwrap();
/// assert_eq!(name.value, 576460752303423488);
///
/// let name = Name::from("a").unwrap();
/// assert_eq!(name.value, 3458764513820540928);
/// ```
pub const fn name_from_str(name_str: &str) -> Name {
    if name_str.len() > 13 {
        panic!("string is too long to be a valid name");
    }

    if name_str.is_empty() {
        return Name { value: 0 };
    }

    let bytes = name_str.as_bytes();
    let len = min(name_str.len(), 12);
    let mut value: u64 = 0;
    let mut i = 0;

    while i < len {
        value <<= 5;
        let value_char = bytes[i];
        let name_char = char_to_value(value_char);
        value |= name_char as u64;
        i += 1;
    }

    value <<= 4 + 5 * (12 - len);

    if bytes.len() > 12 {
        let char_13 = bytes[12];
        let name_char = char_to_value(char_13);
        if name_char > 0x0F {
            panic!("thirteenth character in name cannot be a letter that comes after j");
        }
        value |= name_char as u64;
    }

    Name { value }
}

const fn char_to_value(c: u8) -> u8 {
    match c {
        b'.' => 0,
        b'1'..=b'5' => (c - b'1') + 1,
        b'a'..=b'z' => (c - b'a') + 6,
        _ => panic!("character is not in allowed character set for names"),
    }
}
