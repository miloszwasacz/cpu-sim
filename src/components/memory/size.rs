pub struct MemSize(pub usize);

#[allow(non_snake_case)]
impl MemSize {
    const SI_UNIT_FACTOR: usize = 1000;
    const IEC_UNIT_SHIFT: usize = 10;

    #[inline]
    pub const fn B(self) -> usize {
        self.0
    }

    #[inline]
    pub const fn KB(self) -> usize {
        self.B() * Self::SI_UNIT_FACTOR
    }

    #[inline]
    pub const fn KiB(self) -> usize {
        self.B() << Self::IEC_UNIT_SHIFT
    }

    #[inline]
    pub const fn MB(self) -> usize {
        self.KB() * Self::SI_UNIT_FACTOR
    }

    #[inline]
    pub const fn MiB(self) -> usize {
        self.B() << (2 * Self::IEC_UNIT_SHIFT)
    }

    #[inline]
    pub const fn GB(self) -> usize {
        self.MB() * Self::SI_UNIT_FACTOR
    }

    #[inline]
    pub const fn GiB(self) -> usize {
        self.B() << (3 * Self::IEC_UNIT_SHIFT)
    }
}
