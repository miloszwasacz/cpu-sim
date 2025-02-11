use super::system_instr;
use crate::instr::execute::ExecuteResult;

system_instr!(Ebreak);

impl_execute!(Ebreak, |&self, _, _, _, _, _| { Ok(ExecuteResult::Ebreak) });
impl_mem_access!(Ebreak);
