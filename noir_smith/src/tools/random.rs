/// This object uses a Vec<u8> as a source of randomness, drawing each requested data from this vector.
pub struct Random {
    data: Vec<u8>,
    index: usize,
    length: usize,
}

impl Random {
    pub fn new(data: &[u8]) -> Self {
        Self {
            data: Vec::from(data),
            index: 0,
            length: data.len(),
        }
    }

    /// Generates a random usize within the range [min, max)
    pub fn gen_range(&mut self, min: usize, max: usize) -> usize {
        if min >= max {
            return min;
        }

        let size: usize = (usize::BITS / 8).try_into().unwrap();

        if self.index + size > self.length {
            // Currently, the code cycles through the data if the Vec<u8> is too small, but this might not be optimal for honggfuzz.
            self.index = 0;
        }

        let sub_slice = &self.data[self.index..self.index + size];
        
        self.index += size;

        let mut ret = usize::from_ne_bytes(sub_slice.try_into().unwrap()) % (max + 1 - min) + min;
        if ret == max {
            ret -= 1;
        }

        ret
    }

    /// Returns a random element from the given Vec
    pub fn choose_random_item_from_vec<T: Clone>(&mut self, items: &Vec<T>) -> T {
        if items.is_empty() {
            panic!("Cannot choose from an empty vector");
        }

        let ind = self.gen_range(0, items.len());
        items[ind].clone()
    }

    /// Generates a random u128
    pub fn gen_field(&mut self) -> u128 {
        if self.index + 16 > self.length {
            self.index = 0;
        }

        let sub_slice = &self.data[self.index..self.index + 16];
        
        self.index += 16;

        u128::from_ne_bytes(sub_slice.try_into().unwrap())
    }

    /// Generates a random unsigned integer with the specified bit size
    pub fn gen_random_uint(&mut self, bit_size: usize) -> u128 {
        if self.index + 16 > self.length {
            self.index = 0;
        }

        let sub_slice = &self.data[self.index..self.index + 16];
        
        self.index += 16;

        let random_u128 = u128::from_ne_bytes(sub_slice.try_into().unwrap());

        random_u128 & ((1u128 << bit_size) - 1)
    }

    /// Generates a random signed integer with the specified bit size
    pub fn gen_random_int(&mut self, bit_size: usize) -> i128 {
        if self.index + 16 > self.length {
            self.index = 0;
        }

        let sub_slice = &self.data[self.index..self.index + 16];
        
        self.index += 16;

        let random_i128 = i128::from_ne_bytes(sub_slice.try_into().unwrap());

        random_i128 & ((1i128 << (bit_size - 1)) - 1)
    }

    /// Generates a random bool
    pub fn gen_bool(&mut self) -> bool {
        if self.index + 1 > self.length {
            self.index = 0;
        }

        let v = self.data[self.index];

        self.index += 1;

        v % 2 == 0
    }

    /// Generates a random String with the specified length
    pub fn gen_str(&mut self, size: usize) -> String {
        let mut name = String::with_capacity(size);

        for _ in 0..size {
            let random_char = std::char::from_u32(self.gen_range(b'a' as usize, b'z' as usize) as u32).unwrap_or('\0');
            name.push(random_char);
        }

        name
    }
}
