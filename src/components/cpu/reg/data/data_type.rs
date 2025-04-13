use super::RegData;
use crate::components::memory::ByteConvertible;

#[allow(private_bounds)]
pub trait DataType: DataTypeSealed {}
impl<T: DataTypeSealed> DataType for T {}

trait DataTypeSealed: ByteConvertible + From<RegData> + Into<RegData> {}
impl<T: ByteConvertible + From<RegData> + Into<RegData>> DataTypeSealed for T {}
