use super::Regs;
use crate::ui::model::RegStatModel;
use crate::ui::StatefulComponent;

use ratatui::prelude::*;

pub type RegStat = Regs<RegStatModel>;

impl StatefulComponent for RegStat {
    type Model = RegStatModel;

    fn render(&mut self, model: &Self::Model, area: Rect, buf: &mut Buffer) {
        self.render_registers("Registers", model, area, buf)
    }
}
