#[derive(Debug, PartialEq)]
pub struct BinaryNumber {
    value: u64,
    size: usize,
}

impl BinaryNumber {
    pub fn new(value: u64, size: usize) -> Self {
        // Ensure that the value fits within the specified size
        let value = value & ((1 << size) - 1);
        BinaryNumber { value, size }
    }

    pub fn shift_left(&mut self) {
        self.value = (self.value << 1) | ((self.value >> (self.size - 1)) & 1);
    }

    pub fn to_u64(&self) -> u64 {
        self.value
    }

    pub fn from_u64(value: u64, size: usize) -> Self {
        BinaryNumber::new(value, size)
    }
}
