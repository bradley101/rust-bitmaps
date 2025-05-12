use std::ops::{BitAndAssign, BitOrAssign};

pub struct Bitmap {
    bitmap_capacity: usize,
    bit_count: usize,
    map: Vec<u8>
}

impl Bitmap {
    pub fn new(bit_count: usize) -> Self {
        let bitmap_capacity = bit_count.div_ceil(8);
        let map = vec![0; bitmap_capacity];
        Self {
            bit_count: bit_count,
            bitmap_capacity: bitmap_capacity,
            map: map
        }
    }

    pub fn set(&mut self, bit_index: usize) {
        assert!(bit_index < self.bit_count);
        self.map[bit_index / 8].bitor_assign(1 << (bit_index % 8));
    }

    pub fn unset(&mut self, bit_index: usize) {
        assert!(bit_index < self.bit_count);
        self.map[bit_index / 8].bitand_assign(!(1 << (bit_index % 8)));
    }

    pub fn get(&self, bit_index: usize) -> bool {
        assert!(bit_index < self.bit_count);
        self.map[bit_index / 8] & (1 << (bit_index % 8)) == (1 << (bit_index % 8))
    }

    #[inline]
    pub fn get_bit_count(&self) -> usize {
        self.bit_count
    }

    #[inline]
    pub fn get_bitmap_capacity(&self) -> usize {
        self.bitmap_capacity
    }
}

#[cfg(test)]
mod tests {
    use super::Bitmap;

    #[test]
    pub fn test1() {
        let bmap = Bitmap::new(64);

        for bit in 0..64_usize {
            assert_eq!(bmap.get(bit), false);
        }
    }

    fn check_false_except(bmap: &Bitmap, idx: usize) {
        assert_eq!(bmap.get(idx), true);
        for bit in 0..idx {
            assert_eq!(bmap.get(bit), false);
        }

        for bit in idx + 1..64 {
            assert_eq!(bmap.get(bit), false);
        }
    }

    #[test]
    pub fn test2() {
        let mut bmap = Bitmap::new(64);
        bmap.set(10);
        check_false_except(&bmap, 10);
    }

    #[test]
    pub fn test3() {
        let mut bmap = Bitmap::new(64);
        bmap.set(8);
        check_false_except(&bmap, 8);
    }
}
