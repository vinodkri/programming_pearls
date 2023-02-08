mod bit_vec {
    use core::num;
    use std::vec::Vec;

    pub struct Bitvec{
        block: Vec<u8>,
        count: u32,
        capacity: usize,
    }

    impl Bitvec {
        fn get_vec_index(&self, bitpos: usize) -> usize {
            let block_size = std::mem::size_of::<u8>() * 8;

            bitpos/block_size
        }

        pub fn new(num_of_bits: usize) -> Self {
            let block_size = std::mem::size_of::<u8>() * 8;
            let size = if num_of_bits % block_size == 0 {
                num_of_bits/block_size
            } else {
                num_of_bits/block_size + 1
            };

            Self { block: vec![0;size], count: 0, capacity: num_of_bits }
        }

        pub fn set(&mut self, bit: usize) {
            if bit >= self.capacity {
                panic!("{} bit doesnot exists in the set", bit);
            }
            let index = self.get_vec_index(bit);
            let pos = (0x1 << (bit % 8)) as u8;
            self.block[index] = self.block[index] | pos;
            self.count += 1;
        }

        pub fn reset(&mut self, bit: usize) {
            if bit >= self.capacity {
                panic!("{} bit doesnot exists in the set", bit);
            }
            let index = self.get_vec_index(bit);
            let pos = (0x1 << (bit % 8)) as u8;
            self.block[index] = self.block[index] & !pos;
            self.count -= 1;
        }

        pub fn flip(&mut self, bit: usize) {
            if self.is_set(bit) {
                self.reset(bit);
            } else {
                self.set(bit);
            }
        }

        pub fn is_set(&self, bit: usize) -> bool {
            let index = self.get_vec_index(bit);
            let pos = (0x1 << (bit % 8)) as u8;
            if self.block[index] & pos > 0 { true } else { false }
        }

        pub fn is_reset(&self, bit: usize) -> bool {
            if self.is_set(bit) { false } else { true }
        }

        pub fn count(&self) -> u32 {
            self.count
        }

        pub fn capacity(&self) -> usize {
            self.capacity
        }

    }

}

#[cfg(test)]
mod tests {
    use super::bit_vec::Bitvec;

    #[test]
    fn set_test() {
        let mut block = Bitvec::new(10);
        block.set(9);
        assert_eq!(block.is_set(9), true);
    }

    #[test]
    fn reset_test() {
        let mut block = Bitvec::new(10);
        block.set(8);
        assert_eq!(block.is_set(8), true);
        block.reset(8);
        assert_eq!(block.is_reset(8), true);
    }

    #[test]
    fn flip_test() {
        let mut block = Bitvec::new(128);
        
        block.set(100);

        assert_eq!(block.is_set(100), true);

        block.flip(100);

        assert_eq!(block.is_reset(100), true);
    }

    #[test]
    fn random_test() {
        let mut block = Bitvec::new(128);
        for _ in 0..128 {
            let bit = rand::random::<usize>() % 128 ;
            if block.is_set(bit) {
                block.reset(bit);
            } else {
                block.set(bit);
            }
        }
    }

    #[test]
    fn latge_test() {
        let mut block = Bitvec::new(27000);
        for _ in 0..128 {
            let bit = rand::random::<usize>() % 27000;
            block.flip(bit);
        }
    }


}
