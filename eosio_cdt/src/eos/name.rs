use super::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
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
    if l < r {
        l
    } else {
        r
    }
}

/// EOS Account Name from String
///
/// # Examples
///
/// ```
/// use eosio_cdt::eos::name_from_str;
///
/// let name = name_from_str("");
/// assert_eq!(name.value, 0);
///
/// let name = name_from_str("abc");
/// assert_eq!(name.value, 3589368903014285312);
///
/// let name = name_from_str("123");
/// assert_eq!(name.value, 614178399182651392);
///
/// let name = name_from_str("123.........");
/// assert_eq!(name.value, 614178399182651392);
///
/// let name = name_from_str(".a.b.c.1.2.3.");
/// assert_eq!(name.value, 108209673814966320);
///
/// let name = name_from_str("abc.123");
/// assert_eq!(name.value, 3589369488740450304);
///
/// let name = name_from_str("12345abcdefgj");
/// assert_eq!(name.value, 614251623682315983);
///
/// let name = name_from_str("hijklmnopqrsj");
/// assert_eq!(name.value, 7754926748989239183);
///
/// let name = name_from_str("1");
/// assert_eq!(name.value, 576460752303423488);
///
/// let name = name_from_str("a");
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

const CHAR_MAP: &str = ".12345abcdefghijklmnopqrstuvwxyz";
const CHAR_MASK: u64 = 0xF800000000000000;

pub fn name_to_string(name: Name) -> String {
    let mut buffer = ['.'; 13];

    let mut v: u64 = name.value;

    let mut i = 0;
    loop {
        if v == 0 {
            break;
        }

        let index_correction = if i == 12 { 60 } else { 59 };
        let index_char = (v & CHAR_MASK) >> index_correction;
        buffer[i] = CHAR_MAP.chars().nth(index_char as usize).unwrap();

        v <<= 5;
        i += 1;
    }

    if i == 0 {
        i += 1
    };

    let name_string: String = buffer[..i].into_iter().collect();
    name_string
}

#[test]
fn test_name_to_string_basics() {
    let value = 0;
    assert_eq!(name_to_string(Name { value }), ".");

    let value = 3589368903014285312;
    assert_eq!(name_to_string(Name { value }), "abc");

    let value = 614178399182651392;
    assert_eq!(name_to_string(Name { value }), "123");

    let value = 108209673814966326;
    assert_eq!(name_to_string(Name { value }), ".a.b.c.1.2.3a");

    let value = 3589369488740450304;
    assert_eq!(name_to_string(Name { value }), "abc.123");

    let value = 614251623682315983;
    assert_eq!(name_to_string(Name { value }), "12345abcdefgj");

    let value = 7754926748989239183;
    assert_eq!(name_to_string(Name { value }), "hijklmnopqrsj");

    let value = 576460752303423488;
    assert_eq!(name_to_string(Name { value }), "1");

    let value = 3458764513820540928;
    assert_eq!(name_to_string(Name { value }), "a");
}
