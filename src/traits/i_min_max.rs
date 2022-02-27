pub trait IMinMax where Self: PartialOrd {
    const MAX: Self;
    const MIN: Self;

    // TODO: look for a way to enforce @ compile time ---vvv
    //const INVARIANT: [(); 0 - (Self::MIN <= Self::MAX) as usize] = [];
}

macro_rules! min_max_impl {
    ($($t:ty)*) => ($(
        impl IMinMax for $t {
            const MAX: Self = <Self>::MAX;
            const MIN: Self = <Self>::MIN;
        }
    )*)
}

min_max_impl! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
