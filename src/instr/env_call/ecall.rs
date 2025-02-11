use super::system_instr;
use crate::instr::execute::ExecuteResult;

system_instr!(Ecall);

impl_execute!(Ecall, |&self, _, _, _, _, _| { Ok(ExecuteResult::Ecall) });
impl_mem_access!(Ecall);
