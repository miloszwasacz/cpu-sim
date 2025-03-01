use super::InstrDeriveInput;
use crate::*;

pub fn derive_decode(input: TokenStream) -> TokenStream {
    let InstrDeriveInput {
        name,
        format,
        format_ty,
        ..
    } = parse_macro_input!(input as InstrDeriveInput);

    TokenStream::from(quote! {
        impl crate::instr::Decode for #name {
            fn decode(raw: crate::instr::raw::RawInstr) -> Self {
                Self(#format_ty::decode(raw))
            }

            #[inline]
            fn rs1(&self) -> crate::components::cpu::reg::arf::ArchRegName {
                #format.rs1()
            }

            #[inline]
            fn rs2(&self) -> crate::components::cpu::reg::arf::ArchRegName {
                #format.rs2()
            }

            #[inline]
            fn rd(&self) -> crate::components::cpu::reg::arf::ArchRegName {
                #format.rd()
            }

            #[inline]
            fn imm(&self) -> crate::instr::Immediate {
                #format.imm()
            }
        }
    })
}
