use super::Regs;
use crate::ui::model::FutureFileModel;
use crate::ui::StatefulComponent;

use cpu_sim::components::diagnostics::cpu::RegStatSnapshot;
use ratatui::prelude::*;
use ratatui::widgets::Row;

pub type FutureFile = Regs<FutureFileModel, 5>;

impl StatefulComponent for FutureFile {
    type Model = FutureFileModel;

    fn render(&mut self, model: &Self::Model, area: Rect, buf: &mut Buffer) {
        self.render_registers(
            "Future File",
            model,
            area,
            buf,
            [
                |data: &mut dyn Iterator<Item = &RegStatSnapshot>| {
                    Row::new(data.map(|s| s.data.to_string()))
                },
                |data: &mut dyn Iterator<Item = &RegStatSnapshot>| {
                    Row::new(data.map(|s| match s.writing {
                        None => "✓".to_string(),
                        Some(index) => format!("#{}", index),
                    }))
                },
            ],
        )
    }
}
