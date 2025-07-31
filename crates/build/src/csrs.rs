use self::addr::generate_addr_consts;
use self::csr_trait::generate_csr_impls;
use self::physical::generate_physical_csr_map;
use self::vtable::generate_csr_vtable;

use std::path::Path;

mod addr;
mod csr_trait;
mod physical;
mod vtable;

const SPEC: &str = include_str!("../res/csr.spec");
const MAX_ADDR: AddrType = 0xFFF;

type AddrType = u16;

pub fn code_gen(out_dir: &Path) {
    let csrs = SPEC
        .lines()
        .filter(|s| !s.starts_with("#"))
        .map(CsrMeta::from_spec_line)
        .collect::<Vec<_>>();

    generate_addr_consts(out_dir, &csrs);
    generate_csr_impls(out_dir, &csrs);
    generate_csr_vtable(out_dir, &csrs);
    generate_physical_csr_map(out_dir, &csrs);
}

struct CsrMeta<'a> {
    name: &'a str,
    addr: &'a str,
    addr_parsed: AddrType,
    alias: Option<&'a str>,
}

impl<'a> CsrMeta<'a> {
    fn from_spec_line(line: &'a str) -> Self {
        let alias_split = line.split(" => ").collect::<Vec<_>>();
        let (def, alias) = match alias_split.as_slice() {
            [def] => (def, None),
            [def, alias] => (def, Some(*alias)),
            _ => panic!("invalid CSR definition: `{}`", line),
        };
        let def_split = def.split(',').collect::<Vec<_>>();
        let [addr, name] = def_split.as_slice() else {
            panic!("invalid CSR definition: `{}`", line);
        };

        let addr_hex = addr
            .strip_prefix("0x")
            .expect("CSR address must be in hexadecimal format");
        let addr_parsed = match AddrType::from_str_radix(addr_hex, 16) {
            Ok(addr) if addr <= MAX_ADDR => addr,
            Ok(_) => panic!("CSR address `{}` is too large (max: {:3X})", addr, MAX_ADDR),
            Err(err) => panic!("invalid CSR address ({}): {}", addr, err),
        };

        Self { name, addr, addr_parsed, alias }
    }
}
