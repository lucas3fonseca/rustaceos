use super::EosSerialize;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::cmp;

pub struct Name {
    pub value: u64,
}

impl Name {
    pub fn new(value: u64) -> Self {
        Name { value }
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
    pub fn from(name_str: &str) -> Result<Self, &'static str> {
        if name_str.len() > 13 {
            panic!("string is too long to be a valid name");
        }

        if name_str.is_empty() {
            return Ok(Name { value: 0 });
        }

        let len = cmp::min(name_str.len(), 12);
        let mut value: u64 = 0;

        for i in 0..len {
            value <<= 5;
            if let Some(value_char) = name_str.chars().nth(i) {
                let name_char = char_to_value(value_char)?;
                value |= name_char as u64;
            } else {
                return Err("fail name char at pos");
            }
        }

        value <<= 4 + 5 * (12 - len);

        if let Some(char_13) = name_str.chars().nth(12) {
            let name_char = char_to_value(char_13)?;
            if name_char > 0x0F {
                return Err("thirteenth character in name cannot be a letter that comes after j");
            }
            value |= name_char as u64;
        }

        Ok(Name { value })
    }
}

impl EosSerialize for Name {
    fn read(bytes: &mut Bytes) -> Self {
        let value: u64 = bytes.get_u64_le();
        Name { value }
    }

    fn write(&self, buf: &mut BytesMut) {
        buf.put_u64_le(self.value);
    }
}

fn char_to_value(c: char) -> Result<u8, &'static str> {
    match c {
        '.' => Ok(0),
        '1'..='5' => Ok((c as u8 - b'1') + 1),
        'a'..='z' => Ok((c as u8 - b'a') + 6),
        _ => Err("character is not in allowed character set for names"),
    }
}
