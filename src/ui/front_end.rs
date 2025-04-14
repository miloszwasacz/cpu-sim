use cpu_sim::components::diagnostics::cpu::{CpuSnapshot, IdIsRegs, IfIdRegs};
use cpu_sim::components::diagnostics::fmt_addr;
use cpu_sim::components::memory::Address;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

pub struct FrontEnd<'a> {
    pc: Address,
    if_id_regs: Option<&'a IfIdRegs>,
    id_is_regs: Option<&'a IdIsRegs>,
}

impl<'a> FrontEnd<'a> {
    pub fn new(cpu_snapshot: &'a CpuSnapshot) -> Self {
        Self {
            pc: cpu_snapshot.pc,
            if_id_regs: cpu_snapshot.if_id_regs.as_ref(),
            id_is_regs: cpu_snapshot.id_is_regs.as_ref(),
        }
    }
}

impl Widget for FrontEnd<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [pc_area, if_id_area, id_is_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let fetched = self
            .if_id_regs
            .map(|regs| match regs.instr {
                Ok(instr) => format!("{:032b}", instr),
                Err(ex) => format!("Exception({})", ex),
            })
            .unwrap_or_default();

        let decoded = self
            .id_is_regs
            .map(|regs| match regs.instr {
                Ok(instr) => format!("{:?}", instr),
                Err(ex) => format!("Exception({})", ex),
            })
            .unwrap_or_default();

        Paragraph::new(format!("PC: {}", fmt_addr(self.pc))).render(pc_area, buf);
        Paragraph::new(format!("Fetched: {}", fetched)).render(if_id_area, buf);
        Paragraph::new(format!("Decoded: {}", decoded)).render(id_is_area, buf);
    }
}
