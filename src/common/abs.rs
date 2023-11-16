pub trait Abs {
    fn abs(&self) -> Self;
    fn abs_beween(&self, other: &Self) -> Self;
    fn is_negative(&self) -> bool;
}

macro_rules! signed_impl {
    ($($t:ty)*) => ($(
        impl Abs for $t {
            #[inline]
            fn abs(&self) -> $t {
                if self.is_negative() { -*self } else { *self }
            }
            #[inline]
            fn abs_beween(&self, other: &Self) -> Self {
                (*self - *other).abs()
            }
            #[inline]
            fn is_negative(&self) -> bool {
                *self < 0
            }
        }
    )*)
}
signed_impl!(isize i8 i16 i32 i64 i128);

macro_rules! unsigned_impl {
    ($($t:ty)*) => ($(
        impl Abs for $t {
            #[inline]
            fn abs(&self) -> $t {
                *self
            }
            #[inline]
            fn abs_beween(&self, other: &Self) -> Self {
                if *self >= *other {
                    *self - *other
                } else {
                    *other-*self
                }
            }
            #[inline]
            fn is_negative(&self) -> bool {
                false
            }
        }
    )*)
}
unsigned_impl!(usize u8 u16 u32 u64 u128);
