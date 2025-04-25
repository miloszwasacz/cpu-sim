#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TwoBitPredict {
    #[default]
    StronglyNotTaken,
    WeaklyNotTaken,
    WeaklyTaken,
    StronglyTaken,
}

impl TwoBitPredict {
    pub fn predict(&self) -> bool {
        match self {
            Self::StronglyNotTaken | Self::WeaklyNotTaken => false,
            Self::WeaklyTaken | Self::StronglyTaken => true,
        }
    }

    pub fn updated(&self, taken: bool) -> Self {
        match (self, taken) {
            (Self::StronglyNotTaken, false) => Self::StronglyNotTaken,
            (Self::StronglyNotTaken, true) => Self::WeaklyNotTaken,
            (Self::WeaklyNotTaken, false) => Self::StronglyNotTaken,
            (Self::WeaklyNotTaken, true) => Self::StronglyTaken,
            (Self::WeaklyTaken, false) => Self::StronglyNotTaken,
            (Self::WeaklyTaken, true) => Self::StronglyTaken,
            (Self::StronglyTaken, false) => Self::WeaklyTaken,
            (Self::StronglyTaken, true) => Self::StronglyTaken,
        }
    }
}
