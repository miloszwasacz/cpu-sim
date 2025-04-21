use super::Regs;
use crate::ui::model::RegFileModel;
use crate::ui::StatefulComponent;

use cpu_sim::components::diagnostics::cpu::RegData;
use ratatui::prelude::*;
use ratatui::widgets::Row;
use std::iter;

pub type RegFile = Regs<RegFileModel, 4>;

impl StatefulComponent for RegFile {
    type Model = RegFileModel;

    fn render(&mut self, model: &Self::Model, area: Rect, buf: &mut Buffer) {
        self.render_registers(
            "Register File",
            model,
            area,
            buf,
            iter::once(|data: &mut dyn Iterator<Item = &RegData>| {
                Row::new(data.map(ToString::to_string))
            }),
        )
    }
}
