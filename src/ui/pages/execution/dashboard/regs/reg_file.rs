use super::Regs;
use crate::ui::model::RegFileModel;
use crate::ui::StatefulComponent;

use ratatui::prelude::*;

pub type RegFile = Regs<RegFileModel>;

impl StatefulComponent for RegFile {
    type Model = RegFileModel;

    fn render(&mut self, model: &Self::Model, area: Rect, buf: &mut Buffer) {
        self.render_registers("Registers", model, area, buf)
    }
}
