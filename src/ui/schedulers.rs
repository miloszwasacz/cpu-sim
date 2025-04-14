use cpu_sim::components::diagnostics::cpu::{
    Branch, RegValue, SchedulerEntrySnapshot, SchedulerNotReady as NotReady,
    SchedulerReady as Ready, SchedulerSnapshot,
};
use cpu_sim::components::diagnostics::fmt_addr;
use ratatui::prelude::*;
use ratatui::style::palette::tailwind::{GREEN, RED};
use ratatui::widgets::{Block, Borders, List, ListItem};

pub struct Schedulers<'a>(&'a [SchedulerSnapshot]);

impl<'a> Schedulers<'a> {
    pub fn new(rob: &'a [SchedulerSnapshot]) -> Self {
        Self(rob)
    }
}

impl Widget for Schedulers<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().title("Schedulers").borders(Borders::ALL);

        let widths = std::iter::repeat(Constraint::Fill(1)).take(self.0.len());
        let (scheduler_areas, _) = Layout::horizontal(widths)
            .spacing(1)
            .split_with_spacers(block.inner(area));

        block.render(area, buf);
        for (scheduler, scheduler_area) in self.0.iter().zip(scheduler_areas.iter().copied()) {
            Scheduler(scheduler).render(scheduler_area, buf);
        }
    }
}

struct Scheduler<'a>(&'a SchedulerSnapshot);

impl Widget for Scheduler<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(self.0.ops.join("/"))
            .borders(Borders::ALL);

        let entries = self.0.entries.iter().enumerate().map(|(i, entry)| {
            const NOT_READY_COLOR: Color = RED.c200;
            const READY_COLOR: Color = GREEN.c200;

            let mut line = Line::from(format!("{:>2}: ", i));

            //TODO Add colors to specific ready or not ready fields
            match entry {
                SchedulerEntrySnapshot::Empty => line += Span::from("-"),
                SchedulerEntrySnapshot::NotReady(entry) => {
                    line += Span::from(format!("{:>3}, ", format!("#{}", entry.dest)))
                        .fg(NOT_READY_COLOR);
                    line += Span::from(match entry.data {
                        NotReady::Alu { ctrl, src1, src2 } => {
                            let src1 = match src1 {
                                RegValue::Rob(index) => format!("#{}", index),
                                RegValue::Value(value) => value.i().to_string(),
                            };
                            let src2 = match src2 {
                                RegValue::Rob(index) => format!("#{}", index),
                                RegValue::Value(value) => value.i().to_string(),
                            };
                            format!("{:>5}  {:>5}, {:?}", src1, src2, ctrl)
                        }
                        NotReady::Branch { ctrl, src1, src2 } => {
                            let src1 = match src1 {
                                RegValue::Rob(index) => format!("#{}", index),
                                RegValue::Value(value) => match ctrl {
                                    Branch::Eq | Branch::Ne | Branch::Lt | Branch::Ge => {
                                        value.i().to_string()
                                    }
                                    Branch::Ltu | Branch::Geu => value.u().to_string(),
                                },
                            };
                            let src2 = match src2 {
                                RegValue::Rob(index) => format!("#{}", index),
                                RegValue::Value(value) => match ctrl {
                                    Branch::Eq | Branch::Ne | Branch::Lt | Branch::Ge => {
                                        value.i().to_string()
                                    }
                                    Branch::Ltu | Branch::Geu => value.u().to_string(),
                                },
                            };
                            format!("{:>5}  {:>5}, {:?}", src1, src2, ctrl)
                        }
                        NotReady::Jump { base, offset, .. }
                        | NotReady::Load1 { base, offset, .. }
                        | NotReady::Store { base, offset } => {
                            let base = match base {
                                RegValue::Rob(index) => format!("#{}", index),
                                RegValue::Value(value) => fmt_addr(value.addr()),
                            };
                            format!("{:>5}  {:>5}", base, offset)
                        }
                    })
                    .fg(NOT_READY_COLOR)
                }
                SchedulerEntrySnapshot::Ready(entry) => {
                    line +=
                        Span::from(format!("{:>3}, ", format!("#{}", entry.dest))).fg(READY_COLOR);
                    line += Span::from(match entry.data {
                        Ready::Alu { ctrl, src1, src2 } => {
                            format!("{:>5}  {:>5}, {:?}", src1.i(), src2.i(), ctrl)
                        }
                        Ready::Branch { ctrl, src1, src2 } => match ctrl {
                            Branch::Eq | Branch::Ne | Branch::Lt | Branch::Ge => {
                                format!("{:>5}  {:>5}, {:?}", src1.i(), src2.i(), ctrl)
                            }
                            Branch::Ltu | Branch::Geu => {
                                format!("{:>5}  {:>5}, {:?}", src1.u(), src2.u(), ctrl)
                            }
                        },
                        Ready::Jump { base, offset, .. }
                        | Ready::Load1 { base, offset, .. }
                        | Ready::Store { base, offset } => {
                            format!("{:>5}  {:>5}", fmt_addr(base.addr()), offset)
                        }
                    })
                    .fg(READY_COLOR)
                }
            };
            ListItem::new(line)
        });

        let list = List::new(entries).block(block);
        Widget::render(list, area, buf);
    }
}
