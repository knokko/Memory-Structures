pub trait Saturating {

    fn saturating_add(&self, other: Self) -> Self;
}

impl Saturating for usize {

    fn saturating_add(&self, other: Self) -> Self {
        (*self).saturating_add(other)
    }
}

impl Saturating for isize {

    fn saturating_add(&self, other: Self) -> Self {
        (*self).saturating_add(other)
    }
}

impl Saturating for u8 {

    fn saturating_add(&self, other: Self) -> Self {
        (*self).saturating_add(other)
    }
}

impl Saturating for u16 {

    fn saturating_add(&self, other: Self) -> Self {
        (*self).saturating_add(other)
    }
}

impl Saturating for u32 {

    fn saturating_add(&self, other: Self) -> Self {
        (*self).saturating_add(other)
    }
}

impl Saturating for u64 {

    fn saturating_add(&self, other: Self) -> Self {
        (*self).saturating_add(other)
    }
}

impl Saturating for u128 {

    fn saturating_add(&self, other: Self) -> Self {
        (*self).saturating_add(other)
    }
}

impl Saturating for i8 {

    fn saturating_add(&self, other: Self) -> Self {
        (*self).saturating_add(other)
    }
}

impl Saturating for i16 {

    fn saturating_add(&self, other: Self) -> Self {
        (*self).saturating_add(other)
    }
}

impl Saturating for i32 {

    fn saturating_add(&self, other: Self) -> Self {
        (*self).saturating_add(other)
    }
}

impl Saturating for i64 {

    fn saturating_add(&self, other: Self) -> Self {
        (*self).saturating_add(other)
    }
}

impl Saturating for i128 {

    fn saturating_add(&self, other: Self) -> Self {
        (*self).saturating_add(other)
    }
}