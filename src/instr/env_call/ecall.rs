use super::system_instr;

system_instr!(Ecall);

// TODO Properly implement `Execute` and `MemoryAccess` for `Ecall`
impl_execute!(Ecall, |&self, _, _, _, _, _| {
    todo!()
});
impl_mem_access!(Ecall);
