use self::front_end::FrontEnd;
use self::reg_stat::RegStat;
use self::regs::Regs;
use self::rob::Rob;
use self::schedulers::Schedulers;

use cpu_sim::components::diagnostics::cpu::CpuSnapshot;
use ratatui::prelude::*;

mod front_end;
mod reg_stat;
mod regs;
mod rob;
mod schedulers;

pub struct App(pub CpuSnapshot);

impl Widget for App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [_, front_end, _, bottom] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(area);

        let [rob_area, right] =
            Layout::horizontal([Constraint::Percentage(25), Constraint::Percentage(75)])
                .areas(bottom);
        let [schedulers_area, regs_area, reg_stat_area] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(7),
            Constraint::Length(7),
        ])
        .areas(right);

        FrontEnd::new(&self.0).render(front_end, buf);
        Rob::new(&self.0.rob).render(rob_area, buf);
        Schedulers::new(&self.0.schedulers).render(schedulers_area, buf);
        Regs::new(&self.0.regs).render(regs_area, buf);
        RegStat::new(&self.0.reg_stat).render(reg_stat_area, buf);
    }
}
