use super::{CsrAddr, CsrFile, CSR_INFO};
use crate::components::cpu::csr::CsrAccessCtrl;
use crate::components::diagnostics::Diagnostics;

use itertools::Itertools;

pub struct CsrFileSnapshot(pub Box<[CsrSnapshot]>);

pub struct CsrSnapshot {
    pub name: &'static str,
    pub addr: CsrAddr,
    pub data: String,
    pub access: CsrAccessCtrl,
}

impl Diagnostics for CsrFile {
    type Output = CsrFileSnapshot;

    fn diagnostics(&self) -> Self::Output {
        CsrFileSnapshot(
            CSR_INFO
                .values()
                .map(|info| {
                    let name = info.name;
                    let addr = info.logical_addr;
                    let data = {
                        let physical = self.get(info.physical_addr);
                        let data = physical.read();
                        (info.format)(data)
                    };
                    let access = info.access_ctrl();

                    CsrSnapshot {
                        name,
                        addr,
                        data,
                        access,
                    }
                })
                .sorted_by_key(|csr| csr.addr)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
    }
}
