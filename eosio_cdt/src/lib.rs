pub struct Name {
    pub value: u64,
}

impl Name {
    pub fn new(value: u64) -> Self {
        Name { value }
    }

    pub fn from(value_str: &str) -> Self {
        Name { }
    }
}
