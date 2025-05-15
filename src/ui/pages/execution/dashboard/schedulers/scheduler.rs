use crate::ui::pages::execution::dashboard::scrolling::{Scrolling, ScrollingModel};
use crate::ui::pages::execution::dashboard::{NOT_READY_COLOR, READY_COLOR};
use crate::ui::StatefulComponent;

use cpu_sim::components::diagnostics::cpu::{
    Branch, RegValue, SchedulerEntrySnapshot, SchedulerNotReady as NotReady,
    SchedulerReady as Ready, SchedulerSnapshot,
};
use cpu_sim::components::diagnostics::fmt_addr;
use ratatui::prelude::*;

pub type Scheduler = Scrolling<SchedulerSnapshot>;

impl Scheduler {
    pub const MIN_WIDTH: u16 = 29;
    pub const WIDTH: Constraint = Constraint::Min(Self::MIN_WIDTH);
}

impl StatefulComponent for Scheduler {
    type Model = SchedulerSnapshot;

    fn render(&mut self, model: &Self::Model, area: Rect, buf: &mut Buffer) {
        let base_to_string = |base| match base {
            RegValue::Rob(index) => (format!("#{}", index), NOT_READY_COLOR),
            RegValue::Value(value) => (fmt_addr(value.addr()), READY_COLOR),
        };
        self.render_scrolling(model.ops.join("/"), model, area, buf, |entries| {
            entries.enumerate().map(|(i, entry)| {
                macro_rules! alu_not_ready_line {
                    ($line:expr, $ctrl:expr, $src1:expr, $src2:expr) => {
                        let get_value = |reg_value| {
                            let (text, color) = match reg_value {
                                RegValue::Rob(index) => (format!("#{}", index), NOT_READY_COLOR),
                                RegValue::Value(value) => (value.i().to_string(), READY_COLOR),
                            };
                            Span::from(format!("{:>5}", text)).fg(color)
                        };

                        let src1 = get_value($src1);
                        let src2 = get_value($src2);
                        $line += src1;
                        $line += "  ".into();
                        $line += src2;
                        $line += Span::from(format!(", {:?}", $ctrl)).fg(NOT_READY_COLOR);
                    };
                }

                let mut line = Line::from(format!("{:>2}: ", i));
                match entry {
                    SchedulerEntrySnapshot::Empty => {}
                    SchedulerEntrySnapshot::NotReady(entry) => {
                        line += Span::from(format!("{:>3}, ", format!("#{}", entry.dest)))
                            .fg(NOT_READY_COLOR);
                        match entry.data {
                            NotReady::Alu { ctrl, src1, src2 } => {
                                alu_not_ready_line!(line, ctrl, src1, src2);
                            }
                            NotReady::Mul { ctrl, src1, src2 } => {
                                alu_not_ready_line!(line, ctrl, src1, src2);
                            }
                            NotReady::Branch { ctrl, src1, src2 } => {
                                let get_value = |reg_value| {
                                    let (text, color) = match reg_value {
                                        RegValue::Rob(index) => {
                                            (format!("#{}", index), NOT_READY_COLOR)
                                        }
                                        RegValue::Value(value) => (
                                            match ctrl {
                                                Branch::Eq
                                                | Branch::Ne
                                                | Branch::Lt
                                                | Branch::Ge => value.i().to_string(),
                                                Branch::Ltu | Branch::Geu => value.u().to_string(),
                                            },
                                            READY_COLOR,
                                        ),
                                    };
                                    Span::from(format!("{:>5}", text)).fg(color)
                                };

                                let src1 = get_value(src1);
                                let src2 = get_value(src2);
                                line += src1;
                                line += "  ".into();
                                line += src2;
                                line += Span::from(format!(", {:?}", ctrl)).fg(NOT_READY_COLOR);
                            }
                            NotReady::Jump { base, offset, .. } => {
                                let (base, color) = base_to_string(base);
                                line += Span::from(format!("{:>7}", base)).fg(color);
                                line += "  ".into();
                                line += Span::from(format!("{:>3}", offset)).fg(READY_COLOR);
                                line += Span::from(", Jump").fg(NOT_READY_COLOR)
                            }
                            NotReady::Load1 { base, offset, .. }
                            | NotReady::Store { base, offset } => {
                                let (base, color) = base_to_string(base);
                                line += Span::from(format!("{:>5}", base)).fg(color);
                                line += "  ".into();
                                line += Span::from(format!("{:>5}", offset)).fg(READY_COLOR);
                            }
                        }
                    }
                    SchedulerEntrySnapshot::Ready(entry) => {
                        line += Span::from(format!(
                            "{:>3}, {}",
                            format!("#{}", entry.dest),
                            match entry.data {
                                Ready::Alu { ctrl, src1, src2 } => {
                                    format!("{:>5}  {:>5}, {:?}", src1.i(), src2.i(), ctrl)
                                }
                                Ready::Mul { ctrl, src1, src2 } => {
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
                                Ready::Jump { base, offset, .. } => {
                                    format!("{:>7}  {:>3}, Jump", fmt_addr(base.addr()), offset)
                                }
                                Ready::Load1 { base, offset, .. }
                                | Ready::Store { base, offset } => {
                                    format!("{:>5}  {:>5}", fmt_addr(base.addr()), offset)
                                }
                            }
                        ))
                        .fg(READY_COLOR);
                    }
                };
                line
            })
        })
    }
}

impl ScrollingModel for SchedulerSnapshot {
    type Entry = SchedulerEntrySnapshot;

    #[inline(always)]
    fn entries(&self) -> &[Self::Entry] {
        &self.entries
    }
}
