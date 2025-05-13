use std::ops::{BitAndAssign, BitOrAssign};


/// A bitmap data structure that stores bits in a vector of u8 integers.
///
/// # Examples
///
/// ```
/// use std::cmp::min;
/// use bitmap::Bitmap;
/// 
/// let mut bitmap = Bitmap::new(100);
/// bitmap.set(5);
/// assert_eq!(bitmap.get(5), true);
/// bitmap.unset(5);
/// assert_eq!(bitmap.get(5), false);
/// ```
/// 

pub struct Bitmap {
    /// The capacity of the underlying vector in terms of 8-bit chunks.
    bitmap_capacity: usize,

    /// The total number of bits in the bitmap.
    bit_count: usize,

    /// The underlying vector storing the bitmap data.
    map: Vec<u8>
}

impl Bitmap {
    /// Creates a new bitmap with the specified number of bits.
    ///
    /// # Arguments
    ///
    /// * `bit_count` - The total number of bits in the bitmap.
    ///
    /// # Panics
    ///
    /// Panics if `bit_count` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// let bitmap = bitmap::Bitmap::new(100);
    /// assert_eq!(bitmap.get_bit_count(), 100);
    /// ```
    /// 
    pub fn new(bit_count: usize) -> Self {
        assert!(bit_count > 0);
        let bitmap_capacity = bit_count.div_ceil(8);
        let map = vec![0; bitmap_capacity];
        Self {
            bit_count: bit_count,
            bitmap_capacity: bitmap_capacity,
            map: map
        }
    }

    /// Sets the bit at the specified index to 1.
    ///
    /// # Arguments
    ///
    /// * `bit_index` - The index of the bit to set.
    ///
    /// # Panics
    ///
    /// Panics if `bit_index` is greater than or equal to `bit_count`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut bitmap = bitmap::Bitmap::new(10);
    /// bitmap.set(5);
    /// assert_eq!(bitmap.get(5), true);
    /// ```
    /// 
    pub fn set(&mut self, bit_index: usize) {
        assert!(bit_index < self.bit_count);
        self.map[bit_index / 8].bitor_assign(1 << (bit_index % 8));
    }

    /// Unsets the bit at the specified index to 0.
    ///
    /// # Arguments
    ///
    /// * `bit_index` - The index of the bit to unset.
    ///
    /// # Panics
    ///
    /// Panics if `bit_index` is greater than or equal to `bit_count`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut bitmap = bitmap::Bitmap::new(10);
    /// bitmap.set(5);
    /// bitmap.unset(5);
    /// assert_eq!(bitmap.get(5), false);
    /// ```
    /// 
    pub fn unset(&mut self, bit_index: usize) {
        assert!(bit_index < self.bit_count);
        self.map[bit_index / 8].bitand_assign(!(1 << (bit_index % 8)));
    }

    /// Returns the value of the bit at the specified index.
    ///
    /// # Arguments
    ///
    /// * `bit_index` - The index of the bit to check.
    ///
    /// # Panics
    ///
    /// Panics if `bit_index` is greater than or equal to `bit_count`.
    ///
    /// # Returns
    ///
    /// * `true` if the bit is set to 1.
    /// * `false` if the bit is set to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut bitmap = bitmap::Bitmap::new(10);
    /// bitmap.set(5);
    /// assert_eq!(bitmap.get(5), true);
    /// ```
    /// 
    pub fn get(&self, bit_index: usize) -> bool {
        assert!(bit_index < self.bit_count);
        self.map[bit_index / 8] & (1 << (bit_index % 8)) == (1 << (bit_index % 8))
    }

    /// Returns the total number of bits in the bitmap.
    ///
    /// # Examples
    ///
    /// ```
    /// let bitmap = bitmap::Bitmap::new(100);
    /// assert_eq!(bitmap.get_bit_count(), 100);
    /// ```
    /// 
    #[inline]
    pub fn get_bit_count(&self) -> usize {
        self.bit_count
    }

    /// Returns the capacity of the underlying vector in terms of 8-bit chunks.
    ///
    /// # Examples
    ///
    /// ```
    /// let bitmap = bitmap::Bitmap::new(100);
    /// assert_eq!(bitmap.get_bitmap_capacity(), 13);
    /// ```
    /// 
    #[inline]
    pub fn get_bitmap_capacity(&self) -> usize {
        self.bitmap_capacity
    }
}

#[cfg(test)]
mod tests {
    use super::Bitmap;

    #[test]
    pub fn test_new() {
        let bmap = Bitmap::new(64);
        assert_eq!(bmap.get_bit_count(), 64);
        assert_eq!(bmap.get_bitmap_capacity(), 8);
    }

    #[test]
    pub fn test_set() {
        let mut bmap = Bitmap::new(64);
        bmap.set(10);
        assert_eq!(bmap.get(10), true);
        assert_eq!(bmap.get(0), false);
        assert_eq!(bmap.get(9), false);
    }

    #[test]
    pub fn test_unset() {
        let mut bmap = Bitmap::new(64);
        bmap.set(10);
        bmap.unset(10);
        assert_eq!(bmap.get(10), false);
        assert_eq!(bmap.get(0), false);
        assert_eq!(bmap.get(9), false);
    }

    #[test]
    pub fn test_get() {
        let mut bmap = Bitmap::new(64);
        assert_eq!(bmap.get(0), false);
        bmap.set(10);
        assert_eq!(bmap.get(10), true);
        assert_eq!(bmap.get(0), false);
    }

    #[test]
    pub fn test_get_bit_count() {
        let bmap = Bitmap::new(64);
        assert_eq!(bmap.get_bit_count(), 64);
    }

    #[test]
    pub fn test_get_bitmap_capacity() {
        let bmap = Bitmap::new(64);
        assert_eq!(bmap.get_bitmap_capacity(), 8);
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

    #[test]
    #[should_panic]
    pub fn test_set_panic() {
        let mut bmap = Bitmap::new(64);
        bmap.set(64);
    }

    #[test]
    #[should_panic]
    pub fn test_unset_panic() {
        let mut bmap = Bitmap::new(64);
        bmap.unset(64);
    }

    #[test]
    #[should_panic]
    pub fn test_get_panic() {
        let bmap = Bitmap::new(64);
        bmap.get(64);
    }
}
