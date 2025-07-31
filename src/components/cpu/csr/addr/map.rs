use super::{CsrAddr, CsrAddrRepr};

use phf::Map;

/// Creates a new [`CsrAddrMap`] with a mapping generated in the provided `file`
/// using [`phf_codegen`](https://docs.rs/phf_codegen/latest/phf_codegen/).
macro_rules! csr_addr_map {
    ($file:literal) => {
        CsrAddrMap::new(crate::include_generated!($file as expr))
    };
}
pub(crate) use csr_addr_map;

/// A mapping from [`CsrAddr`] to `T`, generated at compile-time.
#[derive(Debug)]
pub struct CsrAddrMap<T: 'static>(Map<CsrAddrRepr, T>);

impl<T: 'static> CsrAddrMap<T> {
    //noinspection RsUnnecessaryQualifications
    /// Wraps the [`phf::Map`] in a [`CsrAddrMap`].
    ///
    /// This method should never be used directly -- use the [`csr_addr_map!`](csr_addr_map) macro
    /// to create an address map.
    #[doc(hidden)]
    pub const fn new(map: Map<CsrAddrRepr, T>) -> Self {
        Self(map)
    }

    /// Returns a reference to the value mapped to the provided CSR address (if defined).
    pub fn get(&self, addr: CsrAddr) -> Option<&T> {
        self.0.get(&addr.0)
    }

    /// Returns an iterator over the values in the map.
    ///
    /// Values are returned in an arbitrary but fixed order.
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.0.values()
    }
}
