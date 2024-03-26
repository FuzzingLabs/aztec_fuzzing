pub struct Random {
    data: Vec<u8>,
    indice: usize,
    length: usize,
}

impl Random {
    pub fn new(data: &[u8]) -> Self {
        Self {
            data: Vec::from(data),
            indice: 0,
            length: data.len(),
        }
    }

    pub fn gen_range(&mut self, min: usize, max: usize) -> usize {
        if min >= max{
            return min;
        }

        let size: usize = (usize::BITS /8).try_into().unwrap();

        if self.indice + size > self.length {
            self.indice = 0;
        }

        let sub_slice = &self.data[self.indice..self.indice + size];
        
        self.indice += size;

        let mut ret = usize::from_ne_bytes(sub_slice.try_into().unwrap())  % (max + 1 - min) + min;
        if ret == max {
            ret -= 1;
        }

        ret
    }

    pub fn choose_random_item_from_vec<T: Clone>(&mut self, items: &Vec<T>) -> T {
        if items.is_empty() {
            panic!("Cannot choose from an empty vector");
        }

        // Utiliser la méthode choose de rand pour choisir un élément aléatoire
        let ind = self.gen_range(0, items.len());
        items[ind].clone()
    }

    pub fn gen_name(&mut self) -> String {
        let size = self.gen_range(5, 15);
        self.gen_str(size)
    }

    pub fn gen_field(&mut self) -> u128 {
        if self.indice + 16 > self.length {
            self.indice = 0;
        }

        let sub_slice = &self.data[self.indice..self.indice + 16];
        
        self.indice += 16;

        u128::from_ne_bytes(sub_slice.try_into().unwrap())
    }

    pub fn gen_random_uint(&mut self, bit_size: usize) -> u128{
        if self.indice + 16 > self.length {
            self.indice = 0;
        }

        let sub_slice = &self.data[self.indice..self.indice + 16];
        
        self.indice += 16;

        let random_u128 = u128::from_ne_bytes(sub_slice.try_into().unwrap());

        random_u128 & ((1u128 << bit_size) - 1)
    }

    pub fn gen_random_int(&mut self, bit_size: usize) -> i128{
        if self.indice + 16 > self.length {
            self.indice = 0;
        }

        let sub_slice = &self.data[self.indice..self.indice + 16];
        
        self.indice += 16;

        let random_i128 = i128::from_ne_bytes(sub_slice.try_into().unwrap());

        random_i128 & ((1i128 << (bit_size - 1)) - 1)
    }

    pub fn gen_bool(&mut self) -> bool {
        if self.indice + 1 > self.length {
            self.indice = 0;
        }

        let v = self.data[self.indice];

        self.indice += 1;

        v%2 == 0
    }

    pub fn gen_str(&mut self, size: usize) -> String {
        let mut name = String::with_capacity(size);
        let min = b'a' as usize;

        for _ in 0..size {
            let random_char = std::char::from_u32(self.gen_range(b'a' as usize, b'z' as usize) as u32).unwrap_or('\0');
            name.push(random_char);
        }

        name
    }
}