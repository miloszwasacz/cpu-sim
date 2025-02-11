use super::system_instr;

system_instr!(Ebreak);

// TODO Properly implement `Execute` and `MemoryAccess` for `Ebreak`
impl_execute!(Ebreak, |&self, _, _, _, _, _| {
    todo!()
});
impl_mem_access!(Ebreak);
