use cpu_sim::components::diagnostics::cpu::RegStatSnapshot;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Row, Table};

pub struct RegStat<'a>(&'a RegStatSnapshot);

impl<'a> RegStat<'a> {
    pub fn new(regs: &'a RegStatSnapshot) -> Self {
        RegStat(regs)
    }
}

impl Widget for RegStat<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().title("Reg Stat").borders(Borders::ALL);

        let half = self.0.0.len() / 2;
        let names1 = Row::new(self.0.0[..half].iter().map(|(name, _)| name.to_string()));
        let data1 = Row::new(self.0.0[..half].iter().map(|(_, data)| data.to_string()));
        let names2 = Row::new(self.0.0[half..].iter().map(|(name, _)| name.to_string()));
        let data2 = Row::new(self.0.0[half..].iter().map(|(_, data)| data.to_string()));
        let widths = std::iter::repeat(Constraint::Fill(1)).take(half);
        let table = Table::new([names1, data1, Row::new([""]), names2, data2], widths)
            .column_spacing(2)
            .block(block);

        Widget::render(table, area, buf);
    }
}
