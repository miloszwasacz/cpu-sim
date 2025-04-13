macro_rules! tag {
    ($tag:literal) => {
        const_format::formatcp!("#{}!", $tag)
    };
}
pub(super) use tag;

macro_rules! missing_tag {
    ($tag:expr) => {
        panic!("could not find tag `{}`", $tag)
    };
}
pub(super) use missing_tag;
